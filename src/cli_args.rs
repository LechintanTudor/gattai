use clap::{Parser, ValueEnum};
use std::path::PathBuf;

#[derive(Parser)]
pub struct CliArgs {
    #[arg(short, long, default_value = "sprite-sheet")]
    pub output_file: PathBuf,

    #[arg(long, value_enum, default_value_t = OutputMode::Map)]
    pub output_mode: OutputMode,

    #[arg(short, long, default_value_t = 1)]
    pub padding: u32,

    #[arg(long)]
    pub padding_x: Option<u32>,

    #[arg(long)]
    pub padding_y: Option<u32>,

    #[arg(short, long, default_value_t = 1)]
    pub spacing: u32,

    #[arg(long)]
    pub spacing_x: Option<u32>,

    #[arg(long)]
    pub spacing_y: Option<u32>,

    pub input_files: Vec<PathBuf>,
}

#[derive(Clone, Copy, ValueEnum)]
pub enum OutputMode {
    Map,
    Array,
}
