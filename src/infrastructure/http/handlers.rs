use axum::{
    extract::{State, Json},
    http::{StatusCode, header},
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

pub async fn transactional(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<Vec<Request>>,
) -> impl IntoResponse {
    let ctx = crate::application::ports::RequestContext {
        stage: "productive".to_string(), // This should ideally come from config or path
        is_mesap_endpoint: false, // This should come from path
        data_category: crate::domain::DataCategory::TimeSeries, // Default or derived
    };

    match state.pipeline.execute(ctx, payload).await {
        Ok(data) => (StatusCode::OK, Json(data)).into_response(),
        Err(err) => (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()).into_response(),
    }
}

pub async fn transactional_stream(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<Vec<Request>>,
) -> impl IntoResponse {
    let ctx = crate::application::ports::RequestContext {
        stage: "productive".to_string(),
        is_mesap_endpoint: false,
        data_category: crate::domain::DataCategory::TimeSeries,
    };

    match state.pipeline.stream(ctx, payload).await {
        Ok(stream) => {
            let ndjson_stream: std::pin::Pin<Box<dyn futures::Stream<Item = Result<String, std::io::Error>> + Send>> = Box::pin(try_stream! {
                let mut stream = stream;
                while let Some(item_result) = stream.next().await {
                    let item = item_result.map_err(|e| {
                        tracing::error!("Error in transactional_stream pipeline: {:?}", e);
                        std::io::Error::new(std::io::ErrorKind::Other, e.to_string())
                    })?;
                    let mut line = serde_json::to_string(&item).map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))?;
                    line.push('\n');
                    yield line;
                }
            });

            Response::builder()
                .status(StatusCode::OK)
                .header(header::CONTENT_TYPE, "application/x-ndjson")
                .body(axum::body::Body::from_stream(ndjson_stream))
                .unwrap()
        }
        Err(err) => (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()).into_response(),
    }
}

pub async fn generic_csv(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<crate::domain::request::GenericRequest>,
) -> impl IntoResponse {
    let ctx = crate::application::ports::RequestContext {
        stage: "productive".to_string(),
        is_mesap_endpoint: false,
        data_category: crate::domain::DataCategory::TimeSeries,
    };

    let request = payload.into_request();

    match state.pipeline.execute(ctx, vec![request]).await {
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
        Err(err) => (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()).into_response(),
    }
}

pub async fn generic_csv_stream(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<crate::domain::request::GenericRequest>,
) -> impl IntoResponse {
    let ctx = crate::application::ports::RequestContext {
        stage: "productive".to_string(),
        is_mesap_endpoint: false,
        data_category: crate::domain::DataCategory::TimeSeries,
    };

    let request = payload.into_request();

    match state.pipeline.stream(ctx, vec![request]).await {
        Ok(stream) => {
            let csv_stream: std::pin::Pin<Box<dyn futures::Stream<Item = Result<String, std::io::Error>> + Send>> = Box::pin(try_stream! {
                let mut stream = stream;
                let mut header_sent = false;

                while let Some(item_result) = stream.next().await {
                    let item = item_result.map_err(|e| {
                        tracing::error!("Error in generic_csv_stream pipeline: {:?}", e);
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
                }
            });

            Response::builder()
                .status(StatusCode::OK)
                .header(header::CONTENT_TYPE, "text/csv")
                .header(header::CONTENT_DISPOSITION, "attachment; filename=\"data.csv\"")
                .body(axum::body::Body::from_stream(csv_stream))
                .unwrap()
        }
        Err(err) => (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()).into_response(),
    }
}

pub async fn lite_csv() -> impl IntoResponse {
    StatusCode::NOT_IMPLEMENTED
}
