use anyhow::{Context, Ok, bail};
use arboard::Clipboard;
use notify_rust::Notification;
use simplelog::*;
use std::{ffi::OsStr, fs, fs::File, path::Path};

fn main() -> anyhow::Result<()> {
    init_logger()?;

    if let Err(error) = run() {
        log::error!("{error:#}");
        return Err(error);
    }

    Ok(())
}

fn init_logger() -> anyhow::Result<()> {
    let log_path = std::env::temp_dir().join("blipclip.log");
    WriteLogger::init(
        LevelFilter::Debug,
        Config::default(),
        File::create(&log_path)?,
    )?;

    Ok(())
}

fn run() -> anyhow::Result<()> {
    let file_path = std::env::args_os()
        .nth(1)
        .context("no blipc file provided")?;
    let file_path = Path::new(&file_path);
    log::info!("opening {}", file_path.display());

    if file_path.extension() != Some(OsStr::new("blipc")) {
        bail!("refusing to open non-blipc file: {}", file_path.display());
    }

    let contents = fs::read_to_string(file_path)
        .with_context(|| format!("failed to read {}", file_path.display()))?;
    log::info!("read {} bytes", contents.len());

    Clipboard::new()?
        .set_text(contents)
        .context("failed to copy file contents to clipboard")?;
    log::info!("copied file contents to clipboard");

    fs::remove_file(file_path)
        .with_context(|| format!("failed to delete {}", file_path.display()))?;
    log::info!("deleted {}", file_path.display());

    notify("BlipClip", "Copied to clipboard");

    Ok(())
}

fn notify(summary: &str, body: &str) {
    if let Err(error) = Notification::new().summary(summary).body(body).show() {
        log::warn!("failed to show notification: {error:#}");
    }
}
