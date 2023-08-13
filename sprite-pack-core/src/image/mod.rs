mod color;

pub use self::color::*;
use crate::error::{SpriteCoreError, SpriteCoreResult};
use image::io::Reader as ImageReader;
use image::{GenericImage, ImageFormat, Rgba, RgbaImage};
use std::path::Path;

#[derive(Clone, Debug)]
pub struct Image(RgbaImage);

impl Image {
    #[inline]
    pub fn filled((w, h): (u32, u32), color: Color) -> Self {
        Self(RgbaImage::from_pixel(w, h, Rgba(color.to_array())))
    }

    #[inline]
    pub fn fill(&mut self, color: Color) {
        let pixel = Rgba(color.to_array());

        for p in self.0.pixels_mut() {
            *p = pixel;
        }
    }

    #[inline]
    pub fn copy_from(&mut self, image: &Image, (x, y): (u32, u32)) -> SpriteCoreResult {
        self.0
            .copy_from(&image.0, x, y)
            .map_err(SpriteCoreError::new)
    }

    pub fn open_png<P>(path: P) -> SpriteCoreResult<Self>
    where
        P: AsRef<Path>,
    {
        let path = path.as_ref();

        let mut reader =
            ImageReader::open(path).map_err(|error| SpriteCoreError::new_with_path(error, path))?;

        reader.set_format(ImageFormat::Png);

        reader
            .decode()
            .map_err(|error| SpriteCoreError::new_with_path(error, path))
            .map(|image| Self(image.to_rgba8()))
    }

    pub fn save_png<P>(&self, path: P) -> SpriteCoreResult
    where
        P: AsRef<Path>,
    {
        let path = path.as_ref();

        self.0
            .save_with_format(path, ImageFormat::Png)
            .map_err(|error| SpriteCoreError::new_with_path(error, path))
    }

    #[inline]
    #[must_use]
    pub fn size(&self) -> (u32, u32) {
        self.0.dimensions()
    }

    #[inline]
    #[must_use]
    pub fn data(&self) -> &[u8] {
        self.0.as_raw().as_slice()
    }
}
