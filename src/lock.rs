use std::{
    fs::{File, read_to_string},
    io::{ErrorKind, Write},
    path::{Path, PathBuf},
};

use crate::error::{AppError, LockError, Result};

pub struct Lock {
    file: PathBuf,
}

impl Lock {
    pub fn new() -> Result<Self> {
        let lock_path = std::env::temp_dir().join("onionshot.lock");
        match File::create_new(&lock_path) {
            Ok(mut file) => {
                let _ = write!(file, "{}", std::process::id());
                Ok(Lock { file: lock_path })
            }
            Err(e) => match e.kind() {
                ErrorKind::AlreadyExists => {
                    // Check the pid in the lock file
                    if let Ok(s) = read_to_string(&lock_path)
                        && let Ok(pid) = s.trim().parse::<u32>()
                        && Path::new(&format!("/proc/{pid}")).exists()
                    {
                        Err(AppError::LockError(LockError::ExistingInstance))
                    } else {
                        if let Err(e) = std::fs::remove_file(&lock_path) {
                            return Err(AppError::LockError(
                                LockError::DitryLock(lock_path, e),
                            ));
                        }
                        Self::new()
                    }
                }
                _ => Err(AppError::LockError(e.into())),
            },
        }
    }
}

impl Drop for Lock {
    fn drop(&mut self) {
        let _ = std::fs::remove_file(&self.file);
    }
}
