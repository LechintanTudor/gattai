mod bounds;
mod cli_args;
mod encoder;
mod packer;
mod reader;
mod writer;

use crate::cli_args::CliArgs;
use clap::Parser as _;
use std::process::ExitCode;

fn main() -> anyhow::Result<ExitCode> {
    let cli_args = CliArgs::parse();
    let mut success = true;

    // Reader.
    let images = {
        println!("[Reader]");

        if cli_args.input_files.is_empty() {
            println!(" - No images were supplied. Exiting...");
            return Ok(ExitCode::FAILURE);
        }

        let (images, image_errors) = reader::run(&cli_args);

        if !image_errors.is_empty() {
            success = false;

            for image_error in &image_errors {
                println!(" - {image_error}");
            }
        }

        if images.is_empty() {
            println!(" - No images could be read. Exiting...");
            return Ok(ExitCode::FAILURE);
        }

        println!(
            " - Successfully read {} out of {} images",
            images.len(),
            cli_args.input_files.len(),
        );

        println!();
        images
    };

    // Packer.
    let packer_result = {
        println!("[Packer]");
        let packer_result = packer::run(&cli_args, images);

        println!(
            " - Packed {} images in a {}x{} sprite sheet",
            packer_result.images.len(),
            packer_result.size.w,
            packer_result.size.h,
        );

        println!();
        packer_result
    };

    // Encoder.
    let encoder_result = {
        println!("[Encoder]");
        let encoder_result = encoder::run(&cli_args, &packer_result);

        println!(" - Successfully encoded sprite sheet and sprite data");

        println!();
        encoder_result
    };

    // Writer
    {
        println!("[Writer]");

        match writer::run(&cli_args, &encoder_result) {
            Ok(result) => {
                println!(
                    " - Sprite sheet written to '{}'",
                    result.image_file_name.display(),
                );
                println!(
                    " - Sprite data written to '{}'",
                    result.sprites_file_name.display(),
                );
            }
            Err(error) => {
                println!(" - {error}");
            }
        }
    }

    let exit_code = if success {
        ExitCode::SUCCESS
    } else {
        ExitCode::FAILURE
    };

    Ok(exit_code)
}
