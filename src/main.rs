mod headless;

use std::fs::read;

use axum::{body::Body, http::StatusCode, response::Response, routing::get, Router};

#[tokio::main]
async fn main() {
    let router = Router::new()
        .route("/", get(root))
        .route("/img", get(img))
        .route("/screenshot", get(screenshot));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:9000").await.unwrap();
    println!("🚀 http://localhost:9000");
    axum::serve(listener, router).await.unwrap();
}

async fn root() -> &'static str {
    "Hello, world!"
}

async fn screenshot() -> Response {
    match headless::browse_page() {
        Ok(img) => {
            if let Ok(res) = Response::builder().body(Body::from(img)) {
                return res;
            }
            return Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(Body::from("build error"))
                .unwrap();
        }
        Err(err) => Response::builder()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .body(Body::from(err.to_string()))
            .expect("err"),
    }
}

async fn img() -> Response {
    Response::builder()
        .body(Body::from(
            read(format!("{}/img.jpg", std::env!("PWD"))).unwrap(),
        ))
        .unwrap()
}
