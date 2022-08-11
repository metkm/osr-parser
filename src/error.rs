use std::io;

#[derive(Debug)]
pub enum ReplayError {
    IoError(io::Error),

    #[cfg(feature = "lzma")]
    LzmaError(lzma::Error),

    #[cfg(feature = "lzma")]
    LzmaParseError(std::string::FromUtf8Error),
}

impl From<io::Error> for ReplayError {
    fn from(err: io::Error) -> Self {
        Self::IoError(err)
    }
}

#[cfg(feature="lzma")]
impl From<lzma::Error> for ReplayError {
    fn from(err: lzma::Error) -> Self {
        Self::LzmaError(err)
    }
}

#[cfg(feature="lzma")]
impl From<std::string::FromUtf8Error> for ReplayError {
    fn from(err: std::string::FromUtf8Error) -> Self {
        Self::LzmaParseError(err)
    }
}
