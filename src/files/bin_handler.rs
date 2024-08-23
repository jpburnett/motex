use std::{
    fs::File,
    io::{BufReader, Read},
    path::{Path, PathBuf},
};

use anyhow::{Context, Result};

/// A simple struct to hold the path and data of a binary file.
#[derive(Debug, Default)]
pub struct BinFile {
    /// The file path as a `PathBuf`.
    pub path: PathBuf,
    /// The raw bytes of the file.
    pub data: Vec<u8>,
}

/// Opens a file specified by the given path.
///
/// ### Arguments
///
/// * `path` - The path to the file to be opened.
///
/// ### Errors
///
/// Returns an error if the file fails to open.
pub fn read_file_bytes<P: AsRef<Path>>(path: P) -> Result<Vec<u8>> {
    let file = File::open(path.as_ref())
        .with_context(|| format!("Failed to open file: {}", path.as_ref().display()))?;

    let mut buf_reader = BufReader::new(file);
    let mut buffer = Vec::new();

    buf_reader
        .read_to_end(&mut buffer)
        .with_context(|| format!("Failed to read file: {}", path.as_ref().display()))?;

    Ok(buffer)
}

impl BinFile {
    pub fn from_path<P: AsRef<Path>>(path: P) -> Result<Self> {
        let path = path.as_ref().to_path_buf();
        let data = read_file_bytes(&path)?;

        Ok(Self { path, data })
    }
}
