use crate::external::common::is_number_array;

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

pub fn get_active_window() -> Geometry {
    let obj = get_data(vec!["activewindow".into()]);
    if !obj.is_object() {
        panic!(
            "failed to parse hyprctl's active window output: not an object at top level"
        );
    }
    if !obj.has_key("at") {
        panic!(
            "failed to parse hyprctl's active window output: property `at' not found"
        );
    }
    if !obj.has_key("size") {
        panic!(
            "failed to parse hyprctl's active window output: property `size' not found"
        );
    }

    let at = &obj["at"];
    let size = &obj["size"];

    if !is_number_array(&at, 2) {
        panic!(
            "failed to parse hyprctl's active window output: property `at' isn't an array of two numbers"
        );
    }
    if !is_number_array(&size, 2) {
        panic!(
            "failed to parse hyprctl's active window output: property `size' isn't an array of two numbers"
        );
    }
    Geometry {
        x: at[0].as_i32().unwrap(),
        y: at[1].as_i32().unwrap(),
        w: size[0].as_u32().unwrap(),
        h: size[1].as_u32().unwrap(),
    }
}

pub fn get_active_screen() -> Geometry {
    let workspace_obj = get_data(vec!["activeworkspace".into()]);
    let monitor_id = workspace_obj["monitorID"]
        .as_usize()
        .expect("failed to parse hyprctl's active workspace output");

    let monitor = &get_data(vec!["monitors".into()])[monitor_id];

    let msg = "failed to parse hyprctl's monitor output";

    let scale = monitor["scale"].as_f32().expect(msg);
    let x = monitor["x"].as_i32().expect(msg);
    let y = monitor["y"].as_i32().expect(msg);
    let w = monitor["width"].as_f32().expect(msg) / scale;
    let h = monitor["height"].as_f32().expect(msg) / scale;

    Geometry {
        x,
        y,
        w: w as u32,
        h: h as u32,
    }
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
