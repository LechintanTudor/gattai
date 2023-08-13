mod args;

use crate::args::{Args, SpriteSheetKind};
use clap::Parser;
use sprite_pack_core::image::{Color, Image};
use sprite_pack_core::pack::{pack, PackOptions, Sprite};
use sprite_pack_core::sheet::{save_sprite_sheet, GenericSpriteSheetData, SpriteSheetData};

fn main() {
    let args = Args::parse();

    let fill_color = args
        .background
        .map(|hex| Color::from_hex_str(&hex))
        .transpose()
        .unwrap()
        .unwrap_or(Color::TRANSPARENT);

    let mut images = Vec::new();
    let mut errors = Vec::new();

    for path in args.images {
        match Image::open_png(&path) {
            Ok(image) => images.push((path, image)),
            Err(error) => errors.push((path, error)),
        }
    }

    let sprites = images
        .iter()
        .enumerate()
        .map(|(id, (_, image))| {
            Sprite {
                id,
                size: image.size(),
            }
        })
        .collect::<Vec<_>>();

    let pack_result = pack(
        sprites,
        &PackOptions {
            spacing: (args.spacing_x, args.spacing_y),
        },
    );

    let mut sprite_sheet = Image::filled(pack_result.size, fill_color);
    let mut sprite_sheet_data = match args.kind {
        SpriteSheetKind::Array => GenericSpriteSheetData::new_vec(),
        SpriteSheetKind::Map => GenericSpriteSheetData::new_map(),
    };

    pack_result
        .sprites
        .iter()
        .map(|sprite| (sprite.bounds, &images[sprite.id].0, &images[sprite.id].1))
        .for_each(|(bounds, name, image)| {
            sprite_sheet.copy_from(image, bounds.position()).unwrap();
            sprite_sheet_data.add_sprite(name.clone(), bounds);
        });

    sprite_sheet
        .save_png(format!("{}.png", args.output))
        .unwrap();

    let format = args.format.to_core_format();

    save_sprite_sheet(
        &sprite_sheet_data,
        format!("{}.{}", args.output, format.extension()),
        format,
    )
    .unwrap();
}
