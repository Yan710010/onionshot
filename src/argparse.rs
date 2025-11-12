use clap::{Parser, ValueEnum};

/// A screenshot utility program for Hyprland.
#[derive(Parser, Debug)]
#[command(version, about)]
pub struct ApplicationArgs {
    #[arg(short, long, default_value = "fullscreen")]
    pub mode: Mode,

    /// Freeze the screen
    #[arg(short, long)]
    pub freeze: bool,

    /// Storage mode
    #[arg(short, long, default_value = "both")]
    pub storage: StorageMode,

    /// Skip dependency check on start
    #[arg(long)]
    pub skip_depcheck: bool,

    /// Disable hyprland animation when freeze the screen
    #[arg(long)]
    pub disable_animation: bool,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, ValueEnum)]
pub enum Mode {
    Fullscreen,
    Region,
    ActiveWindow,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, ValueEnum)]
pub enum StorageMode {
    FilesystemOnly,
    ClipboardOnly,
    Both,
}
