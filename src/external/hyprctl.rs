use crate::{
    error::Result,
    external::common::fetch_cmd_output,
    validation::{
        assume_object_key_numarr, assume_object_toplevel, try_get_object_key,
        try_get_object_key_as_f32, try_get_object_key_as_i32,
        try_get_object_key_as_usize,
    },
};

use super::common::Geometry;
use json::JsonValue;
use std::process::Command;

fn get_data(args: Vec<String>) -> JsonValue {
    let output = Command::new("hyprctl")
        .arg("-j")
        .args(args)
        .output()
        .expect("Failed to spawn hyprctl");
    if !output.status.success() {
        panic!("Failed to execute hyprctl");
    }
    let Ok(data) = str::from_utf8(&output.stdout) else {
        panic!("non-utf8 fuck off");
    };
    json::parse(data).expect("hyprctl -j not returning json")
}


pub fn get_active_window() -> Result<Geometry> {
    const CONTEXT: &str = "hyprctl's active window output";
    let data = fetch_cmd_output("hyprctl", &["-j", "activewindow"])?;
    let obj = json::parse(&data)?;
    assume_object_toplevel(CONTEXT, &obj)?;
    let at = try_get_object_key(CONTEXT, &obj, "at")?;
    let size = try_get_object_key(CONTEXT, &obj, "size")?;

    assume_object_key_numarr(CONTEXT, "at", at, 2)?;
    assume_object_key_numarr(CONTEXT, "size", size, 2)?;
    Ok(Geometry {
        x: at[0].as_i32().unwrap(),
        y: at[1].as_i32().unwrap(),
        w: size[0].as_u32().unwrap(),
        h: size[1].as_u32().unwrap(),
    })
}

pub fn get_active_screen() -> Result<Geometry> {
    const CTX_AW: &str = "hyprctl's active workspace output";
    const CTX_MON: &str = "hyprctl's monitor output";
    // Get active output
    let data = fetch_cmd_output("hyprctl", &["-j", "activeworkspace"])?;
    let workspace_obj = json::parse(&data)?;
    assume_object_toplevel(CTX_AW, &workspace_obj)?;
    let monitor_id =
        try_get_object_key_as_usize(CTX_AW, &workspace_obj, "monitorID")?;

    // Get active output's bound
    let data = fetch_cmd_output("hyprctl", &["-j", "monitors"])?;
    let monitor = &json::parse(&data)?[monitor_id];

    let scale = try_get_object_key_as_f32(CTX_MON, &monitor, "scale")?;
    let x = try_get_object_key_as_i32(CTX_MON, &monitor, "x")?;
    let y = try_get_object_key_as_i32(CTX_MON, &monitor, "y")?;
    let w = try_get_object_key_as_f32(CTX_MON, &monitor, "width")? / scale;
    let h = try_get_object_key_as_f32(CTX_MON, &monitor, "height")? / scale;

    Ok(Geometry {
        x,
        y,
        w: w as u32,
        h: h as u32,
    })
}

pub fn set_animation(f: bool) -> bool {
    let status =
        get_data(vec!["getoption".into(), "animations:enabled".into()])["int"]
            .as_i8()
            .expect("Unknown animation status:");
    let status = status != 0;

    Command::new("hyprctl")
        .arg("keyword")
        .arg("animations:enabled")
        .arg(if f { "yes" } else { "no" })
        .output()
        .expect("Failed to execute hyprctl");
    // Return previous animation status
    status
}
