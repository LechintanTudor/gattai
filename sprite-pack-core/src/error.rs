use derive_more::{Display, Error, From};
use image::ImageError;
use std::fmt;
use std::io::Error as IoError;
use std::path::PathBuf;

pub type SpriteCoreResult<T = ()> = Result<T, SpriteCoreError>;

#[derive(From, Error, Debug, Display)]
pub enum SpriteCoreErrorKind {
    Io(IoError),

    #[from(ignore)]
    Image(ImageError),

    #[display("serialization error")]
    Serialization,
}

impl From<ImageError> for SpriteCoreErrorKind {
    fn from(error: ImageError) -> Self {
        match error {
            ImageError::IoError(error) => Self::Io(error),
            _ => Self::Image(error),
        }
    }
}

#[derive(Error, Debug)]
pub struct SpriteCoreError {
    pub path: Option<PathBuf>,

    #[error(source)]
    pub kind: SpriteCoreErrorKind,
}

impl SpriteCoreError {
    pub fn new<E>(error: E) -> Self
    where
        E: Into<SpriteCoreErrorKind>,
    {
        Self {
            path: None,
            kind: error.into(),
        }
    }

    pub fn new_with_path<E, P>(error: E, path: P) -> Self
    where
        E: Into<SpriteCoreErrorKind>,
        P: Into<PathBuf>,
    {
        Self {
            path: Some(path.into()),
            kind: error.into(),
        }
    }
}

impl fmt::Display for SpriteCoreError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.path.as_ref() {
            Some(path) => write!(f, "Error in file '{}': {}", path.display(), self.kind),
            None => fmt::Display::fmt(&self.kind, f),
        }
    }
}
