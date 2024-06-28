use crate::cli_args::CliArgs;
use crate::encoder::EncoderResult;
use anyhow::{anyhow, Context};
use image::ImageFormat;
use std::ffi::OsStr;
use std::fs::File;
use std::io::BufWriter;
use std::path::{Path, PathBuf};

#[derive(Clone, Debug)]
pub struct WriterResult {
    pub image_file_name: PathBuf,
    pub sprites_file_name: PathBuf,
}

pub fn run(
    cli_args: &CliArgs,
    encoder_result: &EncoderResult,
) -> anyhow::Result<WriterResult> {
    let (image_extension, image_format) =
        get_image_extension_and_format(&cli_args.output_file)?;

    let image_file_name = cli_args.output_file.with_extension(image_extension);

    // Write image file.
    encoder_result
        .image
        .save_with_format(image_file_name.as_path(), image_format)
        .with_context(|| {
            format!(
                "Failed to write image file to '{}'",
                image_file_name.display(),
            )
        })?;

    let sprites_file_name = cli_args.output_file.with_extension("json");

    // Write sprites file.
    {
        let sprites_writer = File::create(sprites_file_name.as_path())
            .map(BufWriter::new)
            .with_context(|| {
                format!(
                    "Failed to open sprites file '{}'",
                    sprites_file_name.display(),
                )
            })?;

        serde_json::to_writer_pretty(sprites_writer, &encoder_result.sprites)
            .with_context(|| {
            format!(
                "Failed to write sprites file to '{}'",
                sprites_file_name.display(),
            )
        })?;
    }

    Ok(WriterResult {
        image_file_name,
        sprites_file_name,
    })
}

fn get_image_extension_and_format(
    output_file: &Path,
) -> anyhow::Result<(&OsStr, ImageFormat)> {
    let result = match output_file.extension() {
        Some(extension) => {
            let format =
                ImageFormat::from_extension(extension).ok_or_else(|| {
                    anyhow!(
                        "Cannot deduce image format from extension '{}'",
                        extension.to_string_lossy(),
                    )
                })?;

            (extension, format)
        }
        None => (OsStr::new("png"), ImageFormat::Png),
    };

    Ok(result)
}
