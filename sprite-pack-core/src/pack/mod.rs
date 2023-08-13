mod bounds;
mod node;
mod sprite;

pub use self::bounds::*;
pub use self::node::*;
pub use self::sprite::*;
use std::cmp::{self, Reverse};

#[derive(Debug)]
pub struct PackOptions {
    pub spacing: (u32, u32),
}

impl Default for PackOptions {
    fn default() -> Self {
        Self { spacing: (1, 1) }
    }
}

#[derive(Debug)]
pub struct PackResult<I> {
    pub size: (u32, u32),
    pub sprites: Vec<PackedSprite<I>>,
}

pub fn pack<I>(mut sprites: Vec<Sprite<I>>, options: &PackOptions) -> PackResult<I> {
    sprites.sort_by_key(|sprite| Reverse(cmp::max(sprite.size.0, sprite.size.1)));

    let mut sprite_iter = sprites.drain(..);
    let Some(root_sprite) = sprite_iter.next() else {
        return PackResult {
            size: (0, 0),
            sprites: vec![],
        };
    };

    let mut root = Node::root(root_sprite, options.spacing);

    for sprite in sprite_iter {
        root.insert(sprite, options.spacing);
    }

    PackResult {
        size: (
            root.bounds.w - options.spacing.0,
            root.bounds.h - options.spacing.1,
        ),
        sprites: root.into_packed_sprites().collect(),
    }
}
