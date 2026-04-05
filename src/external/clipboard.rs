use crate::error::{AppError, Result};
use std::{
    fs::OpenOptions,
    io::{Read, Write},
    path::Path,
    process::{Command, Stdio},
};

pub fn copy_png(path: &Path) -> Result<()> {
    let mut pngdata = Vec::new();
    OpenOptions::new()
        .read(true)
        .open(path)?
        .read_to_end(&mut pngdata)?;
    let mut wlcopy = Command::new("wl-copy")
        .arg("--type")
        .arg("image/png")
        .stdin(Stdio::piped())
        .spawn()?;
    let Some(mut input) = wlcopy.stdin else {
        wlcopy.kill().unwrap();
        return Err(AppError::CommandStdinError("wl-copy".into()));
    };
    input.write_all(&pngdata)?;
    Ok(())
}
