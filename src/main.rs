mod config;
mod encoder;
mod math;
mod packer;
mod reader;
mod writer;

fn main() -> anyhow::Result<()> {
    let config = config::read_from_cli_args()?;
    let (sprites, _) = reader::run(&config);
    let packer_result = packer::run(&config, sprites);
    let encoder_result = encoder::run(packer_result);
    writer::run(&config, &encoder_result);
    Ok(())
}
