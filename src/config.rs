use crate::error::{AppError, AppErrorKind, AppResult};
use clap::{Parser, ValueEnum};
use image::ImageFormat;
use std::path::PathBuf;

pub fn read_from_cli_args() -> AppResult<Config> {
    CliArgs::parse().try_into()
}

#[derive(Clone, Debug)]
pub struct Config {
    // Paths
    pub input_paths: Vec<PathBuf>,
    pub output_json_path: PathBuf,
    pub output_image_path: PathBuf,

    // Format
    pub sprite_naming: SpriteNaming,
    pub image_format: ImageFormat,

    // Layout
    pub padding_x: u32,
    pub padding_y: u32,
    pub spacing_x: u32,
    pub spacing_y: u32,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, ValueEnum)]
pub enum SpriteNaming {
    Camel,
    Pascal,
    Snake,
    ShoutySnake,
    Kebab,
    ShoutyKebab,
    Title,
    Train,
}

#[derive(Clone, Debug, Parser)]
struct CliArgs {
    input: Vec<PathBuf>,

    #[arg(short, long, default_value = "atlas.png")]
    output: PathBuf,

    #[arg(long, value_enum, default_value_t = SpriteNaming::Camel)]
    sprite_naming: SpriteNaming,

    #[arg(short, long, default_value_t = 1)]
    padding: u32,

    #[arg(long)]
    padding_x: Option<u32>,

    #[arg(long)]
    padding_y: Option<u32>,

    #[arg(short, long, default_value_t = 1)]
    spacing: u32,

    #[arg(long)]
    spacing_x: Option<u32>,

    #[arg(long)]
    spacing_y: Option<u32>,
}

impl TryFrom<CliArgs> for Config {
    type Error = AppError;

    fn try_from(args: CliArgs) -> Result<Self, Self::Error> {
        let output_stem = args.output.file_stem().ok_or_else(|| {
            AppError {
                path: args.output.clone(),
                kind: AppErrorKind::SpriteName,
            }
        })?;

        let mut output_json_path = args.output.clone();
        output_json_path.set_file_name(output_stem);
        output_json_path.set_extension("json");

        let image_format = ImageFormat::from_extension(
            args.output.extension().unwrap_or_default(),
        )
        .ok_or_else(|| {
            AppError {
                path: args.output.clone(),
                kind: AppErrorKind::OutputFormat,
            }
        })?;

        Ok(Self {
            // Paths
            input_paths: args.input,
            output_json_path,
            output_image_path: args.output,

            // Format
            sprite_naming: args.sprite_naming,
            image_format,

            // Spacing
            padding_x: args.padding_x.unwrap_or(args.padding),
            padding_y: args.padding_y.unwrap_or(args.padding),
            spacing_x: args.padding_x.unwrap_or(args.spacing),
            spacing_y: args.padding_y.unwrap_or(args.spacing),
        })
    }
}
