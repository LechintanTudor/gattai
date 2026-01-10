use crate::math::Bounds;
use crate::packer::PackerResult;
use image::{GenericImage, RgbaImage};
use serde::Serialize;
use std::collections::BTreeMap;

pub type EncodedSpriteMap = BTreeMap<String, EncodedSprite>;

pub struct EncoderResult {
    pub sprites: EncodedSpriteMap,
    pub image: RgbaImage,
}

#[derive(Serialize)]
#[serde(untagged)]
pub enum EncodedSprite {
    Single(Bounds),
    Multi(Vec<Bounds>),
}

#[must_use]
pub fn run(mut packer_result: PackerResult) -> EncoderResult {
    packer_result.sprites.sort_by(|s1, s2| {
        (s1.name.cmp(&s2.name)).then_with(|| s1.index.cmp(&s2.index))
    });

    let mut sprites = BTreeMap::<String, EncodedSprite>::new();
    let mut image = RgbaImage::new(packer_result.size.w, packer_result.size.h);

    let mut sprites_iter = packer_result.sprites.into_iter().peekable();

    while let Some(sprite) = sprites_iter.next() {
        let sprite_bounds = sprite.bounds();

        let encoded_sprite = if sprite.is_single() {
            EncodedSprite::Single(sprite_bounds)
        } else {
            let mut bounds = vec![sprite_bounds];

            while let Some(sprite) = sprites_iter
                .next_if(|next_sprite| next_sprite.name == sprite.name)
            {
                bounds.push(sprite.bounds());

                image
                    .copy_from(
                        &sprite.image,
                        sprite.position.x,
                        sprite.position.y,
                    )
                    .unwrap();
            }

            EncodedSprite::Multi(bounds)
        };

        sprites.insert(sprite.name, encoded_sprite);

        image
            .copy_from(&sprite.image, sprite.position.x, sprite.position.y)
            .unwrap();
    }

    EncoderResult { sprites, image }
}
