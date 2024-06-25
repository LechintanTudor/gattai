use crate::bounds::Bounds;
use crate::cli_args::{CliArgs, OutputMode};
use crate::packer::{PackerResult, Sprite};
use image::{GenericImage, GenericImageView, Rgba, RgbaImage};
use serde::Serialize;
use std::collections::BTreeMap;
use std::path::PathBuf;

#[must_use]
#[derive(Clone, Debug)]
pub struct EncoderResult {
    pub image: RgbaImage,
    pub sprites: EncodedSprites,
}

#[derive(Clone, Debug, Serialize)]
#[serde(untagged)]
pub enum EncodedSprites {
    Map(BTreeMap<PathBuf, Bounds>),
    Array(Vec<EncodedSprite>),
}

#[derive(Clone, Debug, Serialize)]
pub struct EncodedSprite {
    pub path: PathBuf,
    pub bounds: Bounds,
}

pub fn run(cli_args: &CliArgs, packer_result: &PackerResult) -> EncoderResult {
    EncoderResult {
        image: build_sprite_sheet(cli_args, packer_result),
        sprites: build_sprite_sheet_data(cli_args, packer_result),
    }
}

fn build_sprite_sheet(
    _cli_args: &CliArgs,
    packer_result: &PackerResult,
) -> RgbaImage {
    let mut image = RgbaImage::from_pixel(
        packer_result.size.w,
        packer_result.size.h,
        Rgba([0, 0, 0, 0]),
    );

    for sprite in &packer_result.sprites {
        image
            .copy_from(
                &packer_result.images[sprite.image_index].image,
                sprite.position.x,
                sprite.position.y,
            )
            .expect("Failed to copy image to sprite sheet");
    }

    image
}

fn build_sprite_sheet_data(
    cli_args: &CliArgs,
    packer_result: &PackerResult,
) -> EncodedSprites {
    let get_image_path_and_bounds = |sprite: &Sprite| {
        let image = &packer_result.images[sprite.image_index];
        let path = image.path.clone();

        let bounds = {
            let (w, h) = image.image.dimensions();
            Bounds::new(sprite.position.x, sprite.position.y, w, h)
        };

        (path, bounds)
    };

    match cli_args.output_mode {
        OutputMode::Map => {
            let sprites = packer_result
                .sprites
                .iter()
                .map(get_image_path_and_bounds)
                .collect();

            EncodedSprites::Map(sprites)
        }
        OutputMode::Array => {
            let mut sprites = packer_result
                .sprites
                .iter()
                .map(|sprite| {
                    let (path, bounds) = get_image_path_and_bounds(sprite);
                    EncodedSprite { path, bounds }
                })
                .collect::<Vec<_>>();

            sprites.sort_unstable_by(|s1, s2| s1.path.cmp(&s2.path));
            EncodedSprites::Array(sprites)
        }
    }
}
