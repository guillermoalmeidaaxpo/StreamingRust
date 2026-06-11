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
    uri: Uri,
    Json(payload): Json<Vec<Request>>,
) -> impl IntoResponse {
    let start_time = std::time::Instant::now();
    let path = uri.path().to_string();
    tracing::info!("Starting request - Endpoint: POST {}, Payload: {:?}", path, payload);
    let ctx = resolve_context(&uri, &state.meta_config.stage);

    match state.pipeline.stream(ctx, payload).await {
        Ok(stream) => {
            let path_clone = path.clone();
            let ndjson_stream: std::pin::Pin<Box<dyn futures::Stream<Item = Result<String, std::io::Error>> + Send>> = Box::pin(try_stream! {
                let mut stream = stream;
                let mut count = 0;
                while let Some(item_result) = stream.next().await {
                    let item = item_result.map_err(|e| {
                        tracing::error!("Error in transactional_stream pipeline ({}): {:?}", path_clone, e);
                        std::io::Error::new(std::io::ErrorKind::Other, e.to_string())
                    })?;
                    let mut line = serde_json::to_string(&item).map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))?;
                    line.push('\n');
                    yield line;
                    count += 1;
                }
                tracing::info!("Finished streaming response - Endpoint: POST {}, Items sent: {}, Duration: {:?}", path_clone, count, start_time.elapsed());
            });

            Response::builder()
                .status(StatusCode::OK)
                .header(header::CONTENT_TYPE, "application/x-ndjson")
                .body(axum::body::Body::from_stream(ndjson_stream))
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
                if !header_sent {
                    let mut header = "Id".to_string();
                    let mut sorted_keys: Vec<_> = item.fields.keys().collect();
                    sorted_keys.sort();
                    for key in &sorted_keys {
                        header.push(',');
                        header.push_str(key);
                    }
                    header.push('\n');
                    csv_output.push_str(&header);
                    header_sent = true;
                }

                let mut line = item.id.to_string();
                let mut sorted_keys: Vec<_> = item.fields.keys().collect();
                sorted_keys.sort();
                for key in sorted_keys {
                    line.push(',');
                    let value = &item.fields[key];
                    line.push_str(&value.to_string().replace("\"", ""));
                }
                line.push('\n');
                csv_output.push_str(&line);
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

    match state.pipeline.stream(ctx, vec![request]).await {
        Ok(stream) => {
            let path_clone = path.clone();
            let csv_stream: std::pin::Pin<Box<dyn futures::Stream<Item = Result<String, std::io::Error>> + Send>> = Box::pin(try_stream! {
                let mut stream = stream;
                let mut header_sent = false;
                let mut count = 0;

                while let Some(item_result) = stream.next().await {
                    let item = item_result.map_err(|e| {
                        tracing::error!("Error in generic_csv_stream pipeline ({}): {:?}", path_clone, e);
                        std::io::Error::new(std::io::ErrorKind::Other, e.to_string())
                    })?;
                    
                    if !header_sent {
                        let mut header = "Id".to_string();
                        let mut sorted_keys: Vec<_> = item.fields.keys().collect();
                        sorted_keys.sort();
                        for key in &sorted_keys {
                            header.push(',');
                            header.push_str(key);
                        }
                        header.push('\n');
                        yield header;
                        header_sent = true;
                    }

                    let mut line = item.id.to_string();
                    let mut sorted_keys: Vec<_> = item.fields.keys().collect();
                    sorted_keys.sort();
                    for key in sorted_keys {
                        line.push(',');
                        let value = &item.fields[key];
                        line.push_str(&value.to_string().replace("\"", ""));
                    }
                    line.push('\n');
                    yield line;
                    count += 1;
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
