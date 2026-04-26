use anyhow::Ok;
use arboard::Clipboard;
use simplelog::*;
use std::fs::File;
use notify_rust::Notification;

fn main() -> anyhow::Result<()> {
    let log_path = std::env::temp_dir().join("blipclip.log");
    WriteLogger::init(
        LevelFilter::Debug,
        Config::default(),
        File::create(&log_path)?,
    )?;
    let mut clipboard = Clipboard::new()?;

    log::info!("app started");
    log::info!("clipboard text: {}", clipboard.get_text().unwrap())
    Notification::new()
    .summary("Blipclip")
    .body("Clipboard text copied to log file!")
    .show()?;
    Ok(())
}
