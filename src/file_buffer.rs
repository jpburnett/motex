// file_buffer.rs

use anyhow::{Context, Result};
use std::fs;
use std::path::PathBuf;

#[derive(Default, Clone)]
pub struct FileBuffer {
    data: Vec<u8>,
    path: Option<PathBuf>,
    modified: bool,
}

impl FileBuffer {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn load_file(&mut self, path: PathBuf) -> Result<()> {
        let data = fs::read(&path).with_context(|| format!("Failed to read file: {:?}", path))?;

        self.data = data;
        self.path = Some(path);
        self.modified = false;

        Ok(())
    }

    pub fn save(&mut self) -> Result<()> {
        if let Some(path) = &self.path {
            fs::write(path, &self.data)
                .with_context(|| format!("Failed to write file: {:?}", path))?;
            self.modified = false;
            Ok(())
        } else {
            Err(anyhow::anyhow!("No file path set"))
        }
    }

    pub fn save_as(&mut self, path: PathBuf) -> Result<()> {
        fs::write(&path, &self.data)
            .with_context(|| format!("Failed to write file: {:?}", path))?;
        self.path = Some(path);
        self.modified = false;
        Ok(())
    }

    pub fn insert_data(&mut self, offset: usize, data: &[u8]) -> Result<()> {
        // Extend buffer if necessary
        if offset + data.len() > self.data.len() {
            self.data.resize(offset + data.len(), 0);
        }

        self.data[offset..offset + data.len()].copy_from_slice(data);
        self.modified = true;

        Ok(())
    }

    pub fn get_data(&self) -> &[u8] {
        &self.data
    }

    pub fn get_slice(&self, offset: usize, len: usize) -> Option<&[u8]> {
        if offset + len <= self.data.len() {
            Some(&self.data[offset..offset + len])
        } else if offset < self.data.len() {
            Some(&self.data[offset..])
        } else {
            None
        }
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    pub fn is_modified(&self) -> bool {
        self.modified
    }

    pub fn path(&self) -> Option<&PathBuf> {
        self.path.as_ref()
    }

    pub fn filename(&self) -> Option<String> {
        self.path.as_ref().and_then(|p| {
            p.file_name()
                .and_then(|n| n.to_str())
                .map(|s| s.to_string())
        })
    }
}
