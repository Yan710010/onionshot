use crate::{
    error::{AppError, JSONValidationError, Result},
    external::common::{fetch_cmd_output, is_number_array},
};

use super::common::Geometry;

pub fn get_active_window() -> Result<Geometry> {
    let data = fetch_cmd_output("hyprctl", &["-j", "activewindow"])?;
    let obj = json::parse(&data)?;
    if !obj.is_object() {
        return Err(AppError::JSONValidationError(
            "hyprctl's active window output".into(),
            JSONValidationError::TopLevelTypeError("object".into()),
        ));
    }
    if !obj.has_key("at") {
        return Err(AppError::JSONValidationError(
            "hyprctl's active window output".into(),
            JSONValidationError::TopLevelPropertyNotFound("at".into()),
        ));
    }
    if !obj.has_key("size") {
        return Err(AppError::JSONValidationError(
            "hyprctl's active window output".into(),
            JSONValidationError::TopLevelPropertyNotFound("size".into()),
        ));
    }

    let at = &obj["at"];
    let size = &obj["size"];

    if !is_number_array(&at, 2) {
        return Err(AppError::JSONValidationError(
            "hyprctl's active window output".into(),
            JSONValidationError::PropertyTypeError(
                "at".into(),
                "an array of two numbers".into(),
            ),
        ));
    }
    if !is_number_array(&size, 2) {
        return Err(AppError::JSONValidationError(
            "hyprctl's active window output".into(),
            JSONValidationError::PropertyTypeError(
                "size".into(),
                "an array of two numbers".into(),
            ),
        ));
    }
    Ok(Geometry {
        x: at[0].as_i32().unwrap(),
        y: at[1].as_i32().unwrap(),
        w: size[0].as_u32().unwrap(),
        h: size[1].as_u32().unwrap(),
    })
}

pub fn get_active_screen() -> Result<Geometry> {
    // Get active output
    let data = fetch_cmd_output("hyprctl", &["-j", "activeworkspace"])?;
    let workspace_obj = json::parse(&data)?;
    let monitor_id = workspace_obj["monitorID"].as_usize().ok_or(
        AppError::JSONValidationError(
            "hyprctl's active workspace output".into(),
            JSONValidationError::PropertyTypeError(
                "monitorID".into(),
                "a number".into(),
            ),
        ),
    )?;

    // Get active output's bound
    let data = fetch_cmd_output("hyprctl", &["-j", "monitors"])?;
    let monitor = &json::parse(&data)?[monitor_id];

    let scale =
        monitor["scale"]
            .as_f32()
            .ok_or(AppError::JSONValidationError(
                "hyprctl's monitor output".into(),
                JSONValidationError::PropertyTypeError(
                    "scale".into(),
                    "a float".into(),
                ),
            ))?;
    let x = monitor["x"].as_i32().ok_or(AppError::JSONValidationError(
        "hyprctl's monitor output".into(),
        JSONValidationError::PropertyTypeError(
            "scale".into(),
            "a float".into(),
        ),
    ))?;
    let y = monitor["y"].as_i32().ok_or(AppError::JSONValidationError(
        "hyprctl's monitor output".into(),
        JSONValidationError::PropertyTypeError(
            "scale".into(),
            "a float".into(),
        ),
    ))?;
    let w = monitor["width"]
        .as_f32()
        .ok_or(AppError::JSONValidationError(
            "hyprctl's monitor output".into(),
            JSONValidationError::PropertyTypeError(
                "scale".into(),
                "a float".into(),
            ),
        ))?
        / scale;
    let h = monitor["height"]
        .as_f32()
        .ok_or(AppError::JSONValidationError(
            "hyprctl's monitor output".into(),
            JSONValidationError::PropertyTypeError(
                "scale".into(),
                "a float".into(),
            ),
        ))?
        / scale;

    Ok(Geometry {
        x,
        y,
        w: w as u32,
        h: h as u32,
    })
}
