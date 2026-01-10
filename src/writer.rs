use crate::config::Config;
use crate::encoder::EncoderResult;
use std::fs;

pub fn run(config: &Config, encoder_result: &EncoderResult) {
    let sprites_json =
        serde_json::to_string_pretty(&encoder_result.sprites).unwrap();

    fs::write(&config.output_json_path, &sprites_json).unwrap();

    encoder_result
        .image
        .save_with_format(&config.output_image_path, config.image_format)
        .unwrap();
}
