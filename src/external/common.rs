use std::process::{Command, Stdio};

use json::JsonValue;

use crate::error::{AppError, Result};

#[derive(Clone, Copy, Debug)]
pub struct Geometry {
    pub x: i32,
    pub y: i32,
    pub w: u32,
    pub h: u32,
}

pub fn is_number_array(obj: &JsonValue, len: usize) -> bool {
    obj.is_array()
        && obj.len() == len
        && obj.members().fold(true, |m, x| m && x.is_number())
}

pub fn fetch_cmd_output(cmd: &str, args: &[&str]) -> Result<String> {
    let output = Command::new(cmd)
        .args(args)
        .stdout(Stdio::piped())
        .output()?;
    if !output.status.success() {
        Err(AppError::ExecutionFailed(cmd.into()))
    } else {
        String::from_utf8(output.stdout).map_err(|_| AppError::Encoding)
    }
}
