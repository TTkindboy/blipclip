use anyhow::{Context, Ok};
use arboard::Clipboard;
use simplelog::*;
use std::{fs, fs::File};

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
    log::info!("opening {}", file_path.to_string_lossy());

    let contents = fs::read_to_string(&file_path)
        .with_context(|| format!("failed to read {}", file_path.to_string_lossy()))?;
    log::info!("read {} bytes", contents.len());

    Clipboard::new()?
        .set_text(contents)
        .context("failed to copy file contents to clipboard")?;
    log::info!("copied file contents to clipboard");

    fs::remove_file(&file_path)
        .with_context(|| format!("failed to delete {}", file_path.to_string_lossy()))?;
    log::info!("deleted {}", file_path.to_string_lossy());

    Ok(())
}
