use std::{path::Path, process::Command};

use crate::error::{AppError, Result};

pub fn notify_screenshot_save(screenshot_path: &Path) -> Result<()> {
    Command::new("notify-send")
        .arg("-i")
        .arg(screenshot_path.to_string_lossy().to_string())
        .arg("-a")
        .arg("onionshot")
        .arg("Screenshot saved")
        .arg(format!(
            "Screenshot successfully saved to {}",
            screenshot_path.to_string_lossy().to_string()
        ))
        .output()?;
    Ok(())
}

pub fn notify_clipboard_save() -> Result<()> {
    Command::new("notify-send")
        .arg("-a")
        .arg("onionshot")
        .arg("Screenshot saved")
        .arg("Screenshot successfully saved to clipboard")
        .output()?;
    Ok(())
}

pub fn notify_save_fail(error: AppError) -> Result<()> {
    Command::new("notify-send")
        .arg("-a")
        .arg("onionshot")
        .arg("Screenshot saving failed")
        .arg(format!("Screenshot failed to save: {}", error))
        .output()?;
    Ok(())
}
