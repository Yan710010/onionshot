use std::{
    fs::{File, read_to_string},
    io::{ErrorKind, Write},
    path::{Path, PathBuf},
};

pub struct Lock {
    file: PathBuf,
}

impl Lock {
    pub fn new() -> Result<Self, String> {
        let lock_path = std::env::temp_dir().join("onionshot.lock");
        match File::create_new(&lock_path) {
            Ok(mut file) => {
                let _ = writeln!(file, "{}", std::process::id());
                Ok(Lock { file: lock_path })
            }
            Err(e) => match e.kind() {
                ErrorKind::AlreadyExists => {
                    // Check the pid in the lock file
                    if let Ok(s) = read_to_string(&lock_path)
                        && let Ok(pid) = s.trim().parse::<u32>()
                        && Path::new(&format!("/proc/{pid}")).exists()
                    {
                        Err("Another instance is already running.".into())
                    } else {
                        if let Err(e) = std::fs::remove_file(&lock_path) {
                            return Err(format!("A dirty lock file exists and cannot be removed: {e}"));
                        }
                        Self::new()
                    }
                }
                ErrorKind::PermissionDenied => Err(
                    "Oops you even do not have the permossion to create file in temp dir, kekw.".into()
                ),
                _ => Err(format!("Error creating lock file: {e}")),
            },
        }
    }
}

impl Drop for Lock {
    fn drop(&mut self) {
        if let Err(e) = std::fs::remove_file(&self.file) {
            match e.kind() {
                ErrorKind::NotFound => {
                    eprintln!("Maybe someone stole the lock...")
                }
                _ => eprintln!("Error removing lock file: {e}"),
            }
        }
    }
}
