use axum::{body::Body, http::StatusCode, response::Response, routing::get, Router};
use headless_chrome::{protocol::cdp::Page, Browser, LaunchOptions};
use std::collections::HashMap;
use std::error::Error;

#[tokio::main]
async fn main() {
    let router = Router::new()
        .route("/", get(|| async { "Hello World!" }))
        .route("/screenshot", get(handle_screenshot));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:9000").await.unwrap();
    println!("🚀 http://localhost:9000");
    axum::serve(listener, router).await.unwrap();
}

async fn handle_screenshot(query: axum::extract::Query<HashMap<String, String>>) -> Response {
    let url = query.get("url");
    if url.is_none() {
        return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .header("Content-Type", "text/plain;charset=UTF-8")
            .body(Body::from("请传入 url".to_string()))
            .unwrap();
    }
    match screenshot(url.unwrap()) {
        Ok(img) => Response::builder().body(Body::from(img)).unwrap(),
        Err(err) => Response::builder()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .body(Body::from(err.to_string()))
            .unwrap(),
    }
}

fn screenshot(url: &str) -> Result<Vec<u8>, Box<dyn Error>> {
    let launch_opts = LaunchOptions::default_builder()
        .headless(true)
        .devtools(false)
        .sandbox(false)
        .port(Some(8010))
        // 这里宽高是 window 的，不是 view 的，注意概念区别
        .window_size(Some((1600, 1200)))
        .build()?;
    // 如果不需要自定义配置项，可以用 Browser::default() 快速实例化一个实例
    let browser = Browser::new(launch_opts).expect("浏览器实例化失败");
    let tab = browser.new_tab()?;
    tab.navigate_to(url)?;
    tab.wait_until_navigated()?;
    let jpeg_data = tab
        .capture_screenshot(Page::CaptureScreenshotFormatOption::Jpeg, None, None, true)
        .expect("截图失败");
    Ok(jpeg_data)
}
