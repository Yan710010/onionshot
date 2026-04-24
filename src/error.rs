use std::io;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error(transparent)]
    IOError(#[from] io::Error),

    #[error("Failed to execute {0}")]
    ExecutionFailed(String),

    #[error("NON-UTF8 FUCK OFF")]
    Encoding,

    #[error(transparent)]
    JsonError(#[from] json::Error),

    #[error("Failed to parse {0}: {1}")]
    JSONValidationError(String, JSONValidationError),

    #[error("Invalid output from {0}: {1}")]
    CommandInvalidOuput(String, String),

    #[error("Failed to obtain stdin of {0}")]
    CommandStdinError(String),

    #[error(transparent)]
    LockError(LockError),
}

#[derive(Error, Debug)]
pub enum JSONValidationError {
    #[error("not an {0} at top level")]
    TopLevelTypeError(String),

    #[error("property `{0}' not found")]
    TopLevelPropertyNotFound(String),

    #[error("property `{0}' isn't {1}")]
    PropertyTypeError(String, String),
}

#[derive(Error, Debug)]
pub enum LockError {
    #[error("Another instance is already running.")]
    ExistingInstance,

    #[error("Dirty lock {0} exists and can not remove: {1}")]
    DitryLock(std::path::PathBuf, io::Error),

    #[error("Failed to create lock: {0}")]
    CreatingLockFailed(#[from] io::Error),
}

pub type Result<T> = std::result::Result<T, AppError>;
