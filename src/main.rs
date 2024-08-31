mod headless;

use axum::{body::Body, http::StatusCode, response::Response, routing::get, Router};
use std::{collections::HashMap, fs::read};

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let router = Router::new()
        .route("/", get(root))
        .route("/img", get(img))
        .route("/screenshot", get(screenshot));
    Ok(router.into())
}

async fn root() -> &'static str {
    "Hello, world!"
}

async fn screenshot(query: axum::extract::Query<HashMap<String, String>>) -> Response {
    let query_url = query.get("url");
    if query_url.is_none() {
        return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from("Query string parameters 'url' is missing."))
            .unwrap();
    }
    match headless::browse_page(query_url.unwrap()) {
        Ok(img) => Response::builder().body(Body::from(img)).unwrap(),
        Err(err) => Response::builder()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .body(Body::from(err.to_string()))
            .expect("err"),
    }
}

async fn img() -> Response {
    Response::builder()
        .body(Body::from(read("./img.jpg").unwrap()))
        .unwrap()
}
