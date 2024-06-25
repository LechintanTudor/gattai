use anyhow::Context;
use image::ImageFormat;
use std::fs::File;
use std::io::BufWriter;

use crate::cli_args::CliArgs;
use crate::encoder::EncoderResult;

pub fn run(
    _cli_args: &CliArgs,
    encoder_result: &EncoderResult,
) -> anyhow::Result<()> {
    encoder_result
        .image
        .save_with_format("sprite-sheet.png", ImageFormat::Png)
        .context("Failed to write sprite sheet")?;

    let sprites_writer = File::create("sprite-sheet.json")
        .map(BufWriter::new)
        .context("Failed to open sprites file")?;

    serde_json::to_writer_pretty(sprites_writer, &encoder_result.sprites)
        .context("Failed to write sprites")?;

    Ok(())
}
