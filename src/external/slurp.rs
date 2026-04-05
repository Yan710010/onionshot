use std::process::{Command, Stdio};

use crate::error::{AppError, Result};

use super::common::Geometry;

pub fn slurp_geometry() -> Result<Geometry> {
    let output = Command::new("slurp")
        .arg("-d")
        .arg("-w")
        .arg("1")
        .arg("-f")
        .arg("%x %y %w %h")
        .stdout(Stdio::piped())
        .output()?;
    if !output.status.success() {
        return Err(AppError::ExecutionFailed("slurp".into()));
    }

    let data =
        str::from_utf8(&output.stdout).map_err(|_| AppError::Encoding)?;
    let nums = data
        .trim()
        .split(' ')
        .map(|x| x.parse::<i64>().ok())
        .fold(Some(Vec::new()), |init, item| {
            if let Some(init) = init
                && let Some(item) = item
            {
                Some(init.iter().chain([item].iter()).map(|x| *x).collect())
            } else {
                None
            }
        })
        //.map(|x| x.iter().collect::<Vec<_>>())
        .ok_or(AppError::CommandInvalidOuput("slurp".into(), data.into()))?;
    if nums.len() != 4 {
        return Err(AppError::CommandInvalidOuput("slurp".into(), data.into()));
    }
    Ok(Geometry {
        x: nums[0] as i32,
        y: nums[1] as i32,
        w: nums[2] as u32,
        h: nums[3] as u32,
    })
}
