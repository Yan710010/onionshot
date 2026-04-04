use concat_idents::concat_idents;
use json::JsonValue;

use crate::{
    error::{AppError, JSONValidationError, Result},
    external::common::is_number_array,
};

pub fn assume_object_toplevel(prompt: &str, object: &JsonValue) -> Result<()> {
    if !object.is_object() {
        return Err(AppError::JSONValidationError(
            prompt.into(),
            JSONValidationError::TopLevelTypeError("object".into()),
        ));
    }
    Ok(())
}

pub fn try_get_object_key<'a>(
    prompt: &str,
    object: &'a JsonValue,
    key: &str,
) -> Result<&'a JsonValue> {
    if !object.has_key(key) {
        return Err(AppError::JSONValidationError(
            prompt.into(),
            JSONValidationError::TopLevelPropertyNotFound(key.into()),
        ));
    }
    Ok(&object[key])
}

pub fn assume_object_key_numarr(
    prompt: &str,
    keyname: &str,
    object: &JsonValue,
    len: usize,
) -> Result<()> {
    if !is_number_array(object, len) {
        return Err(AppError::JSONValidationError(
            prompt.into(),
            JSONValidationError::PropertyTypeError(
                keyname.into(),
                "an array of two numbers".into(),
            ),
        ));
    }
    Ok(())
}

macro_rules! gen_try_get_object_key {
    ($target: ident) => {
        concat_idents!(name = try_get_object_key_as_, $target {
            pub fn name(prompt: &str, object: &JsonValue, key: &str) -> Result<$target> {
                concat_idents!(target_method = as_, $target {
                    return object[key].target_method().ok_or(AppError::JSONValidationError(
                            prompt.into(),
                            JSONValidationError::PropertyTypeError(key.into(), stringify!($target).into())
                            ))
                });
            }
        });
    }
}

gen_try_get_object_key!(usize);
gen_try_get_object_key!(f32);
gen_try_get_object_key!(i32);
