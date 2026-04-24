use std::process::ExitCode;

use clap::Parser;
use onionshot::{
    argparse::{ApplicationArgs, Mode},
    depcheck::check_dep,
    lock::Lock,
    onionshot::{active_window_shot, fullscreen_shot, region_shot},
};

fn main() -> ExitCode {
    if cfg!(not(debug_assertions)) {
        std::panic::set_hook(Box::new(|info| {
            eprint!("\x1b[31mFATAL\x1b[0m ");
            if let Some(msg) = info.payload().downcast_ref::<&str>() {
                eprintln!("{msg}");
            } else if let Some(msg) = info.payload().downcast_ref::<String>() {
                eprintln!("{msg}");
            } else {
                eprintln!("<Some magic payload that no one understands. >")
            }
        }));
    }

    let _lock = match Lock::new() {
        Ok(l) => l,
        Err(e) => {
            eprintln!("\x1b[31mFATAL ERROR:\x1b[0m {e}");
            return ExitCode::FAILURE;
        }
    };
    let args = ApplicationArgs::parse();

    if !args.skip_depcheck {
        if let Some(missing) = check_dep() {
            if missing.len() == 1 {
                eprintln!("Missing dependency: {}", missing[0]);
            } else {
                eprintln!("Missing dependencies: {}", missing.join(", "));
            }
            return ExitCode::FAILURE;
        }
    }

    if let Err(err) = match args.mode {
        Mode::Fullscreen => fullscreen_shot(&args),
        Mode::ActiveWindow => active_window_shot(&args),
        Mode::Region => region_shot(&args),
    } {
        eprintln!("FATAL ERROR: {}", err);
        return ExitCode::FAILURE;
    }
    return ExitCode::SUCCESS;
}
