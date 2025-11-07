use std::path::{Path, PathBuf};

use anyhow::Result;

pub struct Loader {
    pub path: PathBuf,
    pub data: Vec<u8>,
}

impl Loader {
    /// Creates a new `BinFile` instance from the specified path.
    ///
    /// # Arguments
    ///
    /// * `path` - A path to the binary file to be read.
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing the `BinFile` instance if successful,
    /// or an `std::io::Error` if the file cannot be read.
    pub fn from_path<P: AsRef<Path>>(path: P) -> Result<Self> {
        let path = path.as_ref().to_path_buf();
        let data = std::fs::read(&path)?;

        Ok(Self { path, data })
    }
}
