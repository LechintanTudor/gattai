use crate::error::{AppError, AppErrorKind, AppResult};
use clap::{Parser, ValueEnum};
use image::{ImageFormat, Rgba};
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
    pub image_background: Rgba<u8>,
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

/// Spritesheet generator.
#[derive(Clone, Debug, Parser)]
#[command(version)]
struct CliArgs {
    /// Input image files.
    input: Vec<PathBuf>,

    /// Output image file.
    #[arg(short, long, default_value = "spritesheet.png")]
    output: PathBuf,

    /// Sprite naming scheme.
    #[arg(long, value_enum, default_value_t = SpriteNaming::Camel)]
    sprite_naming: SpriteNaming,

    /// Output image background color.
    #[arg(long, value_parser = parse_hex_color, default_value = "000000")]
    background: Rgba<u8>,

    /// Image padding.
    #[arg(short, long, default_value_t = 1)]
    padding: u32,

    /// Horizontal image padding.
    #[arg(long)]
    padding_x: Option<u32>,

    /// Vertical image padding.
    #[arg(long)]
    padding_y: Option<u32>,

    /// Sprite spacing.
    #[arg(short, long, default_value_t = 1)]
    spacing: u32,

    /// Horizontal sprite spacing.
    #[arg(long)]
    spacing_x: Option<u32>,

    /// Vertical sprite spacing.
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
            image_background: args.background,
            image_format,

            // Spacing
            padding_x: args.padding_x.unwrap_or(args.padding),
            padding_y: args.padding_y.unwrap_or(args.padding),
            spacing_x: args.spacing_x.unwrap_or(args.spacing),
            spacing_y: args.spacing_y.unwrap_or(args.spacing),
        })
    }
}

fn parse_hex_color(input: &str) -> Result<Rgba<u8>, &'static str> {
    let input = input.strip_prefix('#').unwrap_or(input);

    if input.len() != 6 && input.len() != 8 {
        return Err("invalid color length");
    }

    let mut digit_iter = input.chars();

    #[allow(clippy::cast_possible_truncation)]
    let mut next_channel = || -> Result<u8, &'static str> {
        let mut digits = [0; 2];

        for digit in &mut digits {
            *digit = digit_iter
                .next()
                .unwrap()
                .to_digit(16)
                .ok_or("invalid hex digit")?;
        }

        Ok(((digits[0] << 4) + digits[1]) as u8)
    };

    let r = next_channel()?;
    let g = next_channel()?;
    let b = next_channel()?;

    let a = if input.len() == 8 {
        next_channel()?
    } else {
        u8::MAX
    };

    Ok(Rgba([r, g, b, a]))
}
