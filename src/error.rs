use std::error::Error as StdError;
use std::fmt;
use std::path::PathBuf;

pub type AppResult<T = ()> = Result<T, AppError>;

#[derive(Clone, Debug)]
pub struct AppError {
    pub path: PathBuf,
    pub kind: AppErrorKind,
}

#[derive(Clone, Copy, Debug)]
pub enum AppErrorKind {
    SpriteName,
    OutputFormat,
    FileRead,
    FileWrite,
    FileDecode,
    FileEncode,
}

impl StdError for AppError {
    // Empty
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "'{}': {}", self.path.display(), self.kind)
    }
}

impl fmt::Display for AppErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let display_str = match self {
            Self::SpriteName => "failed to extract sprite name",
            Self::OutputFormat => "failed to deduce output file format",
            Self::FileRead => "failed to read file",
            Self::FileWrite => "failed to write file",
            Self::FileDecode => "failed to decode file",
            Self::FileEncode => "failed to encode file",
        };

        write!(f, "{display_str}")
    }
}
