mod data;

pub use self::data::*;
use crate::error::{SpriteCoreError, SpriteCoreErrorKind, SpriteCoreResult};
use std::fs::File;
use std::io::BufWriter;
use std::path::Path;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum SpriteSheetFormat {
    Json,
    Ron,
    Yaml,
}

impl SpriteSheetFormat {
    #[inline]
    #[must_use]
    pub fn extension(&self) -> &'static str {
        match self {
            Self::Json => "json",
            Self::Ron => "ron",
            Self::Yaml => "yml",
        }
    }
}

pub fn save_sprite_sheet<D, P>(data: &D, path: P, format: SpriteSheetFormat) -> SpriteCoreResult
where
    D: SpriteSheetData,
    P: AsRef<Path>,
{
    let path = path.as_ref();

    let writer = File::create(path)
        .map_err(|error| SpriteCoreError::new_with_path(error, path))
        .map(BufWriter::new)?;

    match format {
        SpriteSheetFormat::Json => {
            serde_json::to_writer_pretty(writer, data).map_err(|_| {
                SpriteCoreError::new_with_path(SpriteCoreErrorKind::Serialization, path)
            })
        }
        SpriteSheetFormat::Ron => {
            ron::Options::default()
                .to_writer_pretty(writer, data, Default::default())
                .map_err(|_| {
                    SpriteCoreError::new_with_path(SpriteCoreErrorKind::Serialization, path)
                })
        }
        SpriteSheetFormat::Yaml => {
            serde_yaml::to_writer(writer, data).map_err(|_| {
                SpriteCoreError::new_with_path(SpriteCoreErrorKind::Serialization, path)
            })
        }
    }
}
