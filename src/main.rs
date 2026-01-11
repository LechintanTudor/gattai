mod config;
mod encoder;
mod error;
mod math;
mod packer;
mod reader;
mod writer;

use std::process::ExitCode;

fn main() -> ExitCode {
    println!("[Config]");
    let config = match config::read_from_cli_args() {
        Ok(config) => {
            println!(" - Config read and parsed successfully");
            config
        }
        Err(e) => {
            println!(" - {e}");
            return ExitCode::from(1);
        }
    };

    println!("\n[Reader]");
    let sprites = {
        let (sprites, errs) = reader::run(&config);

        for e in &errs {
            println!(" - {e}");
        }

        println!(
            " - Read and decoded {} out of {} images",
            sprites.len(),
            config.input_paths.len(),
        );

        if !errs.is_empty() {
            return ExitCode::from(2);
        }

        sprites
    };

    println!("\n[Packer]");
    let packer_result = packer::run(&config, sprites);

    println!(
        " - Packed {} sprites in a {}x{} atlas",
        packer_result.sprites.len(),
        packer_result.size.w,
        packer_result.size.h,
    );

    let encoder_result = encoder::run(packer_result);

    println!("\n[Writer]");
    match writer::run(&config, &encoder_result) {
        Ok(()) => {
            println!(
                " - Image written to '{}'",
                config.output_image_path.display(),
            );

            println!(
                " - Data written to '{}'",
                config.output_json_path.display(),
            );
        }
        Err(e) => {
            println!(" - {e}");
            return ExitCode::from(3);
        }
    }

    ExitCode::SUCCESS
}
