# Gattai

CLI tool to combine multiple images into a single sprite sheet.

## Installation

Gattai can be installed with `cargo` using:

```shell
cargo install --locked gattai
```

## Usage

Gattai has a simple and easy-to-use CLI interface powered by
[`clap`](https://github.com/clap-rs/clap).

```shell
gattai -o sprite-sheet.png images/**/*.png
```

This command will output two files:

- `sprite-sheet.png`: the sprite sheet containing all images.
- `sprite-sheet.json`: the positions and sizes of all used sprites.

## CLI Options

Gattai provides several options to configure its output, such as:

- `-o`, `--output-file`: configure the name and format of the sprite sheet.
- `-m`, `--output-mode`: configure the format of the sprite data JSON file.
  - `map`: output sprite data as a JSON map:
  ```json
  {
    "sprites": {
      "path/to/image1.png": {
        "x": 0,
        "y": 0,
        "w": 16,
        "h": 16
      },
      "path/to/image2.png": {
        "x": 16,
        "y": 16,
        "w": 16,
        "h": 16
      }
    }
  }
  ```
  - `array`: output sprite data as a JSON array:
  ```json
  {
    "sprites": [
      {
        "path": "path/to/image1.png",
        "bounds": {
          "x": 0,
          "y": 0,
          "w": 16,
          "h": 16
        }
      },
      {
        "path": "path/to/image2.png",
        "bounds": {
          "x": 16,
          "y": 16,
          "w": 16,
          "h": 16
        }
      }
    ]
  }
  ```
- `-p`, `--padding`: configure the padding between the sprites and the border of
  the image.
- `-s`, `--spacing`: configure the spacing between the sprites.

## Supported Image Formats

Gattai is able to use all image formats supported by
[`image`](https://github.com/image-rs/image), the only restriction being that
output formats must have an alpha channel.
