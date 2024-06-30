use clap::{Parser, ValueEnum};
use std::path::PathBuf;

#[derive(Parser)]
#[command(about = "Combine multiple images into a single sprite sheet.")]
pub struct CliArgs {
    #[arg(
        short,
        long,
        help = "Name of the output sprite sheet file",
        default_value = "sprite-sheet"
    )]
    pub output_file: PathBuf,

    #[arg(
        short = 'm',
        long,
        value_enum,
        help = "Output mode for the sprite data JSON file",
        default_value_t = OutputMode::Map
    )]
    pub output_mode: OutputMode,

    #[arg(
        short,
        long,
        help = "Padding between the sprites and the edge of the sprite sheet",
        default_value_t = 1
    )]
    pub padding: u32,

    #[arg(
        long,
        help = "Horizontal padding between the sprites and the edge of the sprite sheet"
    )]
    pub padding_x: Option<u32>,

    #[arg(
        long,
        help = "Vertical padding between the sprites and the edge of the sprite sheet"
    )]
    pub padding_y: Option<u32>,

    #[arg(
        short,
        long,
        help = "Spacing between the sprites",
        default_value_t = 1
    )]
    pub spacing: u32,

    #[arg(long, help = "Horizontal spacing between the sprites")]
    pub spacing_x: Option<u32>,

    #[arg(long, help = "Vertical spacing between the sprites")]
    pub spacing_y: Option<u32>,

    #[arg(help = "Input image files")]
    pub input_files: Vec<PathBuf>,
}

#[derive(Clone, Copy, ValueEnum)]
pub enum OutputMode {
    Map,
    Array,
}
