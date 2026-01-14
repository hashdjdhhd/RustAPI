use lambda_http::{run, service_fn, Body, Error, Request, RequestExt, Response};
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
struct HelloResponse {
    message: String,
    event_id: String,
}

#[derive(Deserialize)]
struct HelloRequest {
    name: String,
}

async fn function_handler(event: Request) -> Result<Response<Body>, Error> {
    let context = event.lambda_context();
    let request_id = context.request_id;

    let name = match event.body() {
        Body::Text(text) => {
            if let Ok(req) = serde_json::from_str::<HelloRequest>(text) {
                req.name
            } else {
                "World".to_string()
            }
        }
        Body::Binary(data) => {
            if let Ok(req) = serde_json::from_slice::<HelloRequest>(data) {
                req.name
            } else {
                "World".to_string()
            }
        }
        _ => "World".to_string(),
    };

    let resp = HelloResponse {
        message: format!("Hello, {}!", name),
        event_id: request_id,
    };

    let resp = Response::builder()
        .status(200)
        .header("content-type", "application/json")
        .body(serde_json::to_string(&resp)?.into())
        .map_err(Box::new)?;

    Ok(resp)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .with_target(false)
        .without_time()
        .init();

    run(service_fn(function_handler)).await
}
