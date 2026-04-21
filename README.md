# Gattai

CLI tool to combine multiple images into a single spritesheet.

## Installation

Gattai can be installed with `cargo` using:

```shell
cargo install --locked gattai
```

## Usage

Gattai has a simple and easy-to-use CLI interface powered by
[`clap`](https://github.com/clap-rs/clap).

```shell
gattai -o spritesheet.png images/**/*.png
```

This command will output two files:

- `spritesheet.png`: the sprite sheet containing all images.
- `spritesheet.json`: the positions and sizes of all sprites.

## Features

Features supported by Gattai.

### Automatic Sprite Grouping

Sprites that end with a number are grouped automatically.

For example, these files...

```
- sprite.png
- animated_sprite_0.png
- animated_sprite_1.png
```

...would end up generating this JSON file:

```json
{
  "sprite": {
    "x": 1,
    "y": 1,
    "w": 16,
    "h": 16
  },
  "animatedSprite": [
    {
      "x": 18,
      "y": 1,
      "w": 16,
      "h": 16
    },
    {
      "x": 1,
      "y": 18,
      "w": 16,
      "h": 16
    }
  ]
}
```

### Supported Image Formats

Gattai is able to use all image formats supported by
[`image`](https://github.com/image-rs/image), the only restriction being that
output formats must have an alpha channel.

## License

Gattai is dual-licensed under either

- MIT License ([LICENSE-MIT](LICENSE-MIT) or
  [https://opensource.org/license/mit/](https://opensource.org/license/mit/))

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  [https://www.apache.org/licenses/LICENSE-2.0](https://www.apache.org/licenses/LICENSE-2.0))

at your option.

<br />

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above without any additional terms or conditions.
