mod headless;

use std::fs::read;

use axum::{body::Body, http::StatusCode, response::Response, routing::get, Router};

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let router = Router::new()
        .route("/", get(hello_world))
        .route("/img", get(img))
        .route("/screenshot", get(root));
    Ok(router.into())
}

async fn hello_world() -> &'static str {
    "Hello, world!"
}

async fn root() -> axum::response::Response {
    match headless::browse_page() {
        Ok(img) => axum::response::Response::builder()
            .body(Body::from(img))
            .unwrap(),
        Err(err) => axum::response::Response::builder()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .body(Body::from(err.to_string()))
            .unwrap(),
    }
}

async fn img() -> Response {
    Response::builder()
        .body(Body::from(read("./img.jpg").unwrap()))
        .unwrap()
}
