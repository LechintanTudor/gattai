mod bounds;
mod cli_args;
mod encoder;
mod packer;
mod reader;
mod writer;

use crate::cli_args::CliArgs;
use clap::Parser as _;

fn main() -> anyhow::Result<()> {
    let cli_args = CliArgs::parse();
    let (images, _) = reader::run(&cli_args);
    let packer_result = packer::run(&cli_args, images);
    let encoder_result = encoder::run(&cli_args, &packer_result);

    println!("{:#?}", encoder_result.sprites);
    writer::run(&cli_args, &encoder_result)
}
