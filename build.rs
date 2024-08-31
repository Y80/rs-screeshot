use std::{borrow::Borrow, process::Command};

fn main() {
    let hostname = std::env::var("HOSTNAME").unwrap_or_default();

    // target/debug/build/<pkg>/output
    // https://doc.rust-lang.org/cargo/reference/build-scripts.html
    println!("HOSTNAME: {}", hostname);

    if hostname.contains("shuttle") {
        let output = Command::new("apt")
            .args([
                "install",
                "-y",
                "ca-certificates",
                "fonts-liberation",
                "libasound2",
                "libatk-bridge2.0-0",
                "libatk1.0-0",
                "libc6",
                "libcairo2",
                "libcups2",
                "libdbus-1-3",
                "libexpat1",
                "libfontconfig1",
                "libgbm1",
                "libgcc1",
                "libglib2.0-0",
                "libgtk-3-0",
                "libnspr4",
                "libnss3",
                "libpango-1.0-0",
                "libpangocairo-1.0-0",
                "libstdc++6",
                "libx11-6",
                "libx11-xcb1",
                "libxcb1",
                "libxcomposite1",
                "libxcursor1",
                "libxdamage1",
                "libxext6",
                "libxfixes3",
                "libxi6",
                "libxrandr2",
                "libxrender1",
                "libxss1",
                "libxtst6",
                "lsb-release",
                "wget",
                "xdg-utils",
            ])
            .output()
            .unwrap();
        if !output.status.success() {
            eprintln!("ERR: {}", String::from_utf8_lossy(output.stderr.borrow()));
        }
    }
}
