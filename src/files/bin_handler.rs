use std::{
    fs::File,
    io::{BufReader, Read},
    path::{Path, PathBuf},
};

use anyhow::{Context, Result};

#[derive(Debug, Default)]
pub struct BinFile {
    pub path: PathBuf,
    pub data: Vec<u8>,
}

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
