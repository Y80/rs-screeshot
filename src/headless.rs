use headless_chrome::{protocol::cdp::Page, Browser, LaunchOptionsBuilder};
use std::error::Error;

pub fn browse_page() -> Result<Vec<u8>, Box<dyn Error>> {
    let launch_opts = LaunchOptionsBuilder::default()
        .headless(true)
        // 这里宽高是 window 的，不是 view 的，注意概念区别
        .window_size(Some((1600, 1200)))
        .build()
        .unwrap();

    let browser = Browser::new(launch_opts)?;

    let tab = browser.new_tab()?;

    tab.navigate_to("https://lccl.cc")?;

    tab.wait_until_navigated()?;

    let jpeg_data =
        tab.capture_screenshot(Page::CaptureScreenshotFormatOption::Jpeg, None, None, true)?;
    // Save the screenshot to disc

    Ok(jpeg_data)
}
