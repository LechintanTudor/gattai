use crate::config::{Config, SpriteNaming};
use crate::error::{AppError, AppErrorKind, AppResult};
use crate::math::{Bounds, Position};
use image::{DynamicImage, ImageError};
use rayon::iter::{Either, IntoParallelIterator, ParallelIterator};
use std::fmt;
use std::path::Path;

const DEFAULT_INDEX: isize = -1;

pub struct Sprite {
    pub name: String,
    pub index: isize,
    pub image: DynamicImage,
    pub position: Position,
}

#[must_use]
pub fn run(config: &Config) -> (Vec<Sprite>, Vec<AppError>) {
    config
        .input_paths
        .as_slice()
        .into_par_iter()
        .map(|path| read_sprite(path, config.sprite_naming))
        .partition_map(|result| {
            match result {
                Ok(sprite) => Either::Left(sprite),
                Err(error) => Either::Right(error),
            }
        })
}

impl Sprite {
    #[must_use]
    pub fn is_single(&self) -> bool {
        self.index == DEFAULT_INDEX
    }

    #[must_use]
    pub fn bounds(&self) -> Bounds {
        Bounds {
            x: self.position.x,
            y: self.position.y,
            w: self.image.width(),
            h: self.image.height(),
        }
    }
}

impl fmt::Debug for Sprite {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct(stringify!(Sprite))
            .field("name", &self.name)
            .field("index", &self.index)
            .field("x", &self.position.x)
            .field("y", &self.position.y)
            .field("w", &self.image.width())
            .field("h", &self.image.height())
            .finish_non_exhaustive()
    }
}

fn read_sprite(path: &Path, naming: SpriteNaming) -> AppResult<Sprite> {
    let (name, index) = {
        let Some(stem) = path.file_stem().and_then(|stem| stem.to_str()) else {
            return Err(AppError {
                path: path.to_owned(),
                kind: AppErrorKind::SpriteName,
            });
        };

        let is_animation =
            stem.chars().last().is_some_and(|c| c.is_ascii_digit());

        if is_animation {
            let index_len =
                stem.chars().rev().take_while(char::is_ascii_digit).count();

            let index_start = stem.len() - index_len;

            let name = fmt_name(&stem[..index_start], naming);

            let Ok(index) = stem[index_start..].parse::<isize>() else {
                return Err(AppError {
                    path: path.to_owned(),
                    kind: AppErrorKind::SpriteName,
                });
            };

            (name, index)
        } else {
            (fmt_name(stem, naming), DEFAULT_INDEX)
        }
    };

    let image = match image::open(path) {
        Ok(image) => image,
        Err(e) => {
            let kind = if matches!(e, ImageError::IoError(_)) {
                AppErrorKind::FileRead
            } else {
                AppErrorKind::FileDecode
            };

            return Err(AppError {
                path: path.to_owned(),
                kind,
            });
        }
    };

    Ok(Sprite {
        name,
        index,
        image,
        position: Position::default(),
    })
}

#[must_use]
fn fmt_name(stem: &str, naming: SpriteNaming) -> String {
    use heck::*;

    match naming {
        SpriteNaming::Camel => stem.to_lower_camel_case(),
        SpriteNaming::Pascal => stem.to_pascal_case(),
        SpriteNaming::Snake => stem.to_snake_case(),
        SpriteNaming::ShoutySnake => stem.to_shouty_snake_case(),
        SpriteNaming::Kebab => stem.to_kebab_case(),
        SpriteNaming::ShoutyKebab => stem.to_shouty_kebab_case(),
        SpriteNaming::Title => stem.to_title_case(),
        SpriteNaming::Train => stem.to_train_case(),
    }
}
