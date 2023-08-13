use clap::{Parser, ValueEnum};
use sprite_pack_core::sheet::SpriteSheetFormat as CoreSpriteSheetFormat;

#[derive(Clone, Copy, Debug, ValueEnum)]
pub enum SpriteSheetKind {
    Array,
    Map,
}

#[derive(Clone, Copy, Debug, ValueEnum)]
pub enum SpriteSheetFormat {
    Json,
    Ron,
    Yaml,
}

impl SpriteSheetFormat {
    #[inline]
    #[must_use]
    pub fn to_core_format(&self) -> CoreSpriteSheetFormat {
        match self {
            Self::Json => CoreSpriteSheetFormat::Json,
            Self::Ron => CoreSpriteSheetFormat::Ron,
            Self::Yaml => CoreSpriteSheetFormat::Yaml,
        }
    }
}

#[derive(Clone, Debug, Parser)]
pub struct Args {
    #[arg(value_name = "IMAGE", help = "Images to pack")]
    pub images: Vec<String>,

    #[arg(long, default_value = "array", help = "Set sprite sheet kind")]
    pub kind: SpriteSheetKind,

    #[arg(long, default_value = "json", help = "Set sprite sheet format")]
    pub format: SpriteSheetFormat,

    #[arg(long, short, help = "Set sprite sheet background color")]
    pub background: Option<String>,

    #[arg(
        long,
        default_value = "0",
        help = "Set minimum horizontal spacing between sprites"
    )]
    pub spacing_x: u32,

    #[arg(
        long,
        default_value = "0",
        help = "Set minimum vertical spacing between sprites"
    )]
    pub spacing_y: u32,

    #[arg(long, short, help = "Set output file name")]
    pub output: String,
}
