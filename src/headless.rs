use headless_chrome::{protocol::cdp::Page, Browser, LaunchOptions};
use std::error::Error;

pub fn browse_page(url: &str) -> Result<Vec<u8>, Box<dyn Error>> {
    println!("Enter browse_page()");

    let launch_opts = LaunchOptions::default_builder()
        .headless(true)
        .devtools(false)
        .sandbox(false)
        .port(Some(8010))
        // 这里宽高是 window 的，不是 view 的，注意概念区别
        .window_size(Some((1600, 1200)))
        .build()?;

    // println!("Test chrome");
    // let output =
    //     Command::new("/root/.local/share/headless-chrome/linux-1095492/chrome-linux/chrome")
    //         .output()
    //         .unwrap();
    // println!("{}", String::from_utf8_lossy(output.stdout.borrow()));
    // println!("{}", String::from_utf8_lossy(output.stderr.borrow()));

    let browser = Browser::new(launch_opts)?;

    let tab = browser.new_tab()?;
    tab.navigate_to(url)?;
    tab.wait_until_navigated()?;

    let jpeg_data =
        tab.capture_screenshot(Page::CaptureScreenshotFormatOption::Jpeg, None, None, true)?;

    Ok(jpeg_data)
}
