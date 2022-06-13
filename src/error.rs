use std::io;

#[derive(Debug)]
pub enum ReplayError {
    IoError(io::Error),

    #[cfg(feature = "lzma")]
    LzmaError(lzma::Error),
}

impl From<io::Error> for ReplayError {
    fn from(err: io::Error) -> Self {
        Self::IoError(err)
    }
}

impl From<lzma::Error> for ReplayError {
    fn from(err: lzma::Error) -> Self {
        Self::LzmaError(err)
    }
}
