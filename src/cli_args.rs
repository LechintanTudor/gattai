use clap::{Parser, ValueEnum};
use std::path::PathBuf;

/// Combine multiple images into a single sprite sheet.
#[derive(Parser)]
pub struct CliArgs {
    /// Output image file name
    #[arg(short, long, default_value = "sprite-sheet.png")]
    pub output_file: PathBuf,

    /// Output mode for the data file
    #[arg(
        short = 'm',
        long,
        value_enum,
        default_value_t = OutputMode::Map
    )]
    pub output_mode: OutputMode,

    /// Padding between the sprites and the edge of the sprite sheet
    #[arg(short, long, default_value_t = 1)]
    pub padding: u32,

    /// Horizontal padding between the sprites and the edge of the sprite sheet
    #[arg(long)]
    pub padding_x: Option<u32>,

    /// Vertical padding between the sprites and the edge of the sprite sheet
    #[arg(long)]
    pub padding_y: Option<u32>,

    /// Spacing between the sprites
    #[arg(short, long, default_value_t = 1)]
    pub spacing: u32,

    /// Horizontal spacing between the sprites
    #[arg(long)]
    pub spacing_x: Option<u32>,

    /// Vertical spacing between the sprites
    #[arg(long)]
    pub spacing_y: Option<u32>,

    /// Input image files
    pub input_files: Vec<PathBuf>,
}

#[derive(Clone, Copy, ValueEnum)]
pub enum OutputMode {
    Map,
    Array,
}
