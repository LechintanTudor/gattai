use crate::cli_args::CliArgs;
use anyhow::Context;
use image::DynamicImage;
use rayon::iter::{
    IndexedParallelIterator, IntoParallelIterator, ParallelIterator,
};
use std::path::PathBuf;

#[derive(Clone, Debug)]
pub struct Image {
    pub path: PathBuf,
    pub image: DynamicImage,
}

pub fn run(cli_args: &CliArgs) -> (Vec<Image>, Vec<anyhow::Error>) {
    let mut image_results = Vec::new();

    cli_args
        .input_files
        .as_slice()
        .into_par_iter()
        .map(|path| {
            image::open(path.as_path())
                .map(|image| {
                    Image {
                        path: path.clone(),
                        image,
                    }
                })
                .with_context(|| {
                    format!(
                        "Failed to open or decode image '{}'",
                        path.display(),
                    )
                })
        })
        .collect_into_vec(&mut image_results);

    let mut images = Vec::new();
    let mut errors = Vec::new();

    for image_result in image_results {
        match image_result {
            Ok(image) => images.push(image),
            Err(error) => errors.push(error),
        }
    }

    (images, errors)
}
