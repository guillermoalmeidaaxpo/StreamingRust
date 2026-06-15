use axum::{
    extract::{State, Json},
    http::{StatusCode, header, Uri},
    response::{IntoResponse, Response},
};
use std::sync::Arc;
use futures::StreamExt;
use async_stream::try_stream;
use crate::domain::Request;
use super::router::AppState;

pub async fn health() -> impl IntoResponse {
    StatusCode::OK
}

fn resolve_context(uri: &Uri, default_stage: &str) -> crate::application::ports::RequestContext {
    let path = uri.path();
    
    let stage = if path.contains("/design") {
        "design".to_string()
    } else if path.contains("/validation") {
        "validation".to_string()
    } else if path.contains("/migration") {
        "migration".to_string()
    } else if path.contains("/productive") {
        "productive".to_string()
    } else {
        default_stage.to_string()
    };

    let data_category = if path.contains("/curves") {
        crate::domain::DataCategory::Curves
    } else if path.contains("/surfaces") {
        crate::domain::DataCategory::Surfaces
    } else {
        crate::domain::DataCategory::TimeSeries
    };

    let is_mesap_endpoint = path.contains("/mesap") || path.contains("mesaptransition");

    crate::application::ports::RequestContext {
        stage,
        is_mesap_endpoint,
        data_category,
    }
}

pub async fn transactional(
    State(state): State<Arc<AppState>>,
    uri: Uri,
    Json(payload): Json<Vec<Request>>,
) -> impl IntoResponse {
    let start_time = std::time::Instant::now();
    let path = uri.path().to_string();
    tracing::info!("Starting request - Endpoint: POST {}, Payload: {:?}", path, payload);
    let ctx = resolve_context(&uri, &state.meta_config.stage);

    let response = match state.pipeline.execute(ctx, payload).await {
        Ok(data) => (StatusCode::OK, Json(data)).into_response(),
        Err(err) => {
            tracing::error!("Error in transactional endpoint ({}): {:?}", path, err);
            (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()).into_response()
        }
    };
    tracing::info!("Finished request - Endpoint: POST {}, Duration: {:?}", path, start_time.elapsed());
    response
}

pub async fn transactional_stream(
    State(state): State<Arc<AppState>>,
    headers: axum::http::HeaderMap,
    uri: Uri,
    Json(payload): Json<Vec<Request>>,
) -> impl IntoResponse {
    let start_time = std::time::Instant::now();
    let path = uri.path().to_string();
    tracing::info!("Starting request - Endpoint: POST {}, Payload: {:?}", path, payload);
    let ctx = resolve_context(&uri, &state.meta_config.stage);

    let accept_header = headers.get(axum::http::header::ACCEPT)
        .and_then(|v| v.to_str().ok())
        .unwrap_or("");
    let is_ndjson = accept_header.to_lowercase().contains("application/x-ndjson")
        || accept_header.to_lowercase().contains("ndjson");

    let setup_start = std::time::Instant::now();
    match state.pipeline.stream(ctx, payload).await {
        Ok(stream) => {
            let setup_duration = setup_start.elapsed();
            tracing::info!("Stream initialized - Endpoint: POST {}, Setup duration: {:?}", path, setup_duration);
            let path_clone = path.clone();
            
            let content_type = if is_ndjson {
                "application/x-ndjson"
            } else {
                "application/json"
            };

            let response_stream: std::pin::Pin<Box<dyn futures::Stream<Item = Result<String, std::io::Error>> + Send>> = Box::pin(try_stream! {
                let mut stream = stream;
                let mut count = 0;
                let mut buffer = String::with_capacity(65536);
                
                if !is_ndjson {
                    buffer.push('[');
                }
                
                while let Some(item_result) = stream.next().await {
                    let item = item_result.map_err(|e| {
                        tracing::error!("Error in transactional_stream pipeline ({}): {:?}", path_clone, e);
                        std::io::Error::new(std::io::ErrorKind::Other, e.to_string())
                    })?;
                    
                    let serialized = serde_json::to_string(&item).map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))?;
                    
                    if is_ndjson {
                        let mut line = serialized;
                        line.push('\n');
                        buffer.push_str(&line);
                    } else {
                        if count > 0 {
                            buffer.push(',');
                        }
                        buffer.push_str(&serialized);
                    }
                    
                    count += 1;
                    
                    if buffer.len() >= 65536 {
                        let chunk = std::mem::replace(&mut buffer, String::with_capacity(65536));
                        yield chunk;
                    }
                }
                
                if !is_ndjson {
                    buffer.push(']');
                }
                
                if !buffer.is_empty() {
                    yield buffer;
                }
                
                tracing::info!("Finished streaming response - Endpoint: POST {}, Items sent: {}, Duration: {:?}", path_clone, count, start_time.elapsed());
            });

            Response::builder()
                .status(StatusCode::OK)
                .header(header::CONTENT_TYPE, content_type)
                .body(axum::body::Body::from_stream(response_stream))
                .unwrap()
        }
        Err(err) => {
            tracing::error!("Error in transactional_stream endpoint ({}): {:?}", path, err);
            (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()).into_response()
        }
    }
}

pub async fn generic_csv(
    State(state): State<Arc<AppState>>,
    uri: Uri,
    Json(payload): Json<crate::domain::request::GenericRequest>,
) -> impl IntoResponse {
    let start_time = std::time::Instant::now();
    let path = uri.path().to_string();
    tracing::info!("Starting request - Endpoint: POST {}, Payload: {:?}", path, payload);
    let ctx = resolve_context(&uri, &state.meta_config.stage);

    let request = payload.into_request();

    let response = match state.pipeline.execute(ctx, vec![request]).await {
        Ok(items) => {
            let mut csv_output = String::new();
            let mut header_sent = false;

            for item in items {
                let mut fields = item.fields.clone();
                if !fields.contains_key("Identifier") {
                    fields.insert("Identifier".to_string(), serde_json::Value::Number(item.id.into()));
                }

                if !header_sent {
                    let mut sorted_keys: Vec<_> = fields.keys().collect();
                    sorted_keys.sort();
                    let header = sorted_keys.iter().map(|k| k.as_str()).collect::<Vec<_>>().join(",");
                    csv_output.push_str(&header);
                    csv_output.push('\n');
                    header_sent = true;
                }

                let mut sorted_keys: Vec<_> = fields.keys().collect();
                sorted_keys.sort();
                let line_vals: Vec<String> = sorted_keys.iter().map(|key| {
                    let value = &fields[*key];
                    value.to_string().replace("\"", "")
                }).collect();
                csv_output.push_str(&line_vals.join(","));
                csv_output.push('\n');
            }

            Response::builder()
                .status(StatusCode::OK)
                .header(header::CONTENT_TYPE, "text/csv")
                .header(header::CONTENT_DISPOSITION, "attachment; filename=\"data.csv\"")
                .body(axum::body::Body::from(csv_output))
                .unwrap()
        }
        Err(err) => {
            tracing::error!("Error in generic_csv endpoint ({}): {:?}", path, err);
            (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()).into_response()
        }
    };
    tracing::info!("Finished request - Endpoint: POST {}, Duration: {:?}", path, start_time.elapsed());
    response
}

pub async fn generic_csv_stream(
    State(state): State<Arc<AppState>>,
    uri: Uri,
    Json(payload): Json<crate::domain::request::GenericRequest>,
) -> impl IntoResponse {
    let start_time = std::time::Instant::now();
    let path = uri.path().to_string();
    tracing::info!("Starting request - Endpoint: POST {}, Payload: {:?}", path, payload);
    let ctx = resolve_context(&uri, &state.meta_config.stage);

    let request = payload.into_request();

    let setup_start = std::time::Instant::now();
    match state.pipeline.stream(ctx, vec![request]).await {
        Ok(stream) => {
            let setup_duration = setup_start.elapsed();
            tracing::info!("Stream initialized - Endpoint: POST {}, Setup duration: {:?}", path, setup_duration);
            let path_clone = path.clone();
            let csv_stream: std::pin::Pin<Box<dyn futures::Stream<Item = Result<String, std::io::Error>> + Send>> = Box::pin(try_stream! {
                let mut stream = stream;
                let mut header_sent = false;
                let mut count = 0;
                let mut buffer = String::with_capacity(65536);

                while let Some(item_result) = stream.next().await {
                    let item = item_result.map_err(|e| {
                        tracing::error!("Error in generic_csv_stream pipeline ({}): {:?}", path_clone, e);
                        std::io::Error::new(std::io::ErrorKind::Other, e.to_string())
                    })?;
                    
                    let mut fields = item.fields.clone();
                    if !fields.contains_key("Identifier") {
                        fields.insert("Identifier".to_string(), serde_json::Value::Number(item.id.into()));
                    }

                    if !header_sent {
                        let mut sorted_keys: Vec<_> = fields.keys().collect();
                        sorted_keys.sort();
                        let header = sorted_keys.iter().map(|k| k.as_str()).collect::<Vec<_>>().join(",");
                        buffer.push_str(&header);
                        buffer.push('\n');
                        header_sent = true;
                    }

                    let mut sorted_keys: Vec<_> = fields.keys().collect();
                    sorted_keys.sort();
                    let line_vals: Vec<String> = sorted_keys.iter().map(|key| {
                        let value = &fields[*key];
                        value.to_string().replace("\"", "")
                    }).collect();
                    
                    buffer.push_str(&line_vals.join(","));
                    buffer.push('\n');
                    count += 1;
                    
                    if buffer.len() >= 65536 {
                        let chunk = std::mem::replace(&mut buffer, String::with_capacity(65536));
                        yield chunk;
                    }
                }
                
                if !buffer.is_empty() {
                    yield buffer;
                }
                
                tracing::info!("Finished streaming response - Endpoint: POST {}, Items sent: {}, Duration: {:?}", path_clone, count, start_time.elapsed());
            });

            Response::builder()
                .status(StatusCode::OK)
                .header(header::CONTENT_TYPE, "text/csv")
                .header(header::CONTENT_DISPOSITION, "attachment; filename=\"data.csv\"")
                .body(axum::body::Body::from_stream(csv_stream))
                .unwrap()
        }
        Err(err) => {
            tracing::error!("Error in generic_csv_stream endpoint ({}): {:?}", path, err);
            (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()).into_response()
        }
    }
}

pub async fn lite_csv() -> impl IntoResponse {
    StatusCode::NOT_IMPLEMENTED
}
