use arboard::Clipboard;
use simplelog::*;
use std::fs::File;

fn main() {
    let log_file = if cfg!(target_os = "macos") {
        "/tmp/blipclip.log"
    } else {
        "C:/temp/blipclip.log"
    };
    WriteLogger::init(
        LevelFilter::Debug,
        Config::default(),
        File::create(log_file).unwrap(),
    ).unwrap();
    let mut clipboard = Clipboard::new().unwrap();

    log::info!("app started");
    log::info!("clipboard text: {}", clipboard.get_text().unwrap())
    
}
