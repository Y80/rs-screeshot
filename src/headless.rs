use headless_chrome::{protocol::cdp::Page, Browser, LaunchOptions};
use std::error::Error;

pub fn browse_page() -> Result<Vec<u8>, Box<dyn Error>> {
    println!("Enter browse_page()");

    let launch_opts = LaunchOptions::default_builder()
        .headless(true)
        .devtools(false)
        .sandbox(true)
        .port(Some(8010))
        // 这里宽高是 window 的，不是 view 的，注意概念区别
        .window_size(Some((1600, 1200)))
        .build()
        .expect("build launch options failed");

    println!("Start construct browse");

    let browser = Browser::new(launch_opts).expect("浏览器实例化失败");

    println!("Start create a tab");

    let tab = browser.new_tab()?;

    println!("Start navigate");

    tab.navigate_to("https://google.com")?;

    tab.wait_until_navigated()?;

    let jpeg_data = tab
        .capture_screenshot(Page::CaptureScreenshotFormatOption::Jpeg, None, None, true)
        .expect("截图失败");

    // tab.close_target()?;

    Ok(jpeg_data)
}
