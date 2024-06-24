use anyhow::Context;
use image::ImageFormat;

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

    Ok(())
}
