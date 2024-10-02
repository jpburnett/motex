use std::path::{Path, PathBuf};

use anyhow::Result;

/// A simple struct to hold the path and data of a binary file.
#[derive(Debug, Default, PartialEq)]
pub struct BinFile {
    /// The file path as a `PathBuf`.
    pub path: PathBuf,
    /// The raw bytes of the file.
    pub data: Vec<u8>,
}

impl BinFile {
    pub fn from_path<P: AsRef<Path>>(path: P) -> Result<Self> {
        let path = path.as_ref().to_path_buf();
        let data = std::fs::read(&path).unwrap();

        Ok(Self { path, data })
    }
}
