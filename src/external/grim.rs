use std::{
    path::Path,
    process::{Child, Command},
};

use super::common::Geometry;
use crate::error::Result;

pub fn grim(output: &Path) -> Result<Child> {
    Command::new("grim")
        .arg(output.to_string_lossy().to_string())
        .spawn()
        .map_err(|x| x.into())
}

pub fn grim_with_geometry(output: &Path, geometry: Geometry) -> Result<()> {
    let _ = Command::new("grim")
        .arg("-g")
        .arg(format!(
            "{},{} {}x{}",
            geometry.x, geometry.y, geometry.w, geometry.h
        ))
        .arg(output.to_string_lossy().to_string())
        .output()?;
    Ok(())
}
