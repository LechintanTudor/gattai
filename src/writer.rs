use crate::config::Config;
use crate::encoder::EncoderResult;
use crate::error::{AppError, AppErrorKind, AppResult};
use image::ImageError;
use std::fs;

pub fn run(config: &Config, encoder_result: &EncoderResult) -> AppResult {
    let sprites_json = serde_json::to_string_pretty(&encoder_result.sprites)
        .map_err(|_| {
            AppError {
                path: config.output_json_path.clone(),
                kind: AppErrorKind::FileEncode,
            }
        })?;

    fs::write(&config.output_json_path, &sprites_json).map_err(|_| {
        AppError {
            path: config.output_json_path.clone(),
            kind: AppErrorKind::FileWrite,
        }
    })?;

    encoder_result
        .image
        .save_with_format(&config.output_image_path, config.image_format)
        .map_err(|e| {
            let kind = if matches!(e, ImageError::IoError(_)) {
                AppErrorKind::FileWrite
            } else {
                AppErrorKind::FileEncode
            };

            AppError {
                path: config.output_json_path.clone(),
                kind,
            }
        })
}
