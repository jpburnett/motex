use crate::file_buffer::FileBuffer;
use anyhow::Result;
use std::fs;
use std::path::PathBuf;

#[derive(Default)]
pub struct PaletteManager {
    external_palette: Option<Vec<u8>>,
    external_palette_path: Option<PathBuf>,
}

impl PaletteManager {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn load_external_palette(&mut self, path: PathBuf) -> Result<()> {
        let data = fs::read(&path)?;
        self.external_palette = Some(data);
        self.external_palette_path = Some(path);
        Ok(())
    }

    pub fn clear_external_palette(&mut self) {
        self.external_palette = None;
        self.external_palette_path = None;
    }

    pub fn has_external_palette(&self) -> bool {
        self.external_palette.is_some()
    }

    pub fn external_palette_filename(&self) -> Option<String> {
        self.external_palette_path.as_ref().and_then(|p| {
            p.file_name()
                .and_then(|n| n.to_str())
                .map(|s| s.to_string())
        })
    }

    pub fn get_palette_data(
        &self,
        buffer: &FileBuffer,
        palette_offset: usize,
        split_palette: bool,
        split_offset: usize,
    ) -> Vec<u8> {
        // If external palette is loaded, use it
        if let Some(ext_pal) = &self.external_palette {
            return ext_pal.clone();
        }

        // Otherwise, extract from buffer
        let palette_size = 256 * 2; // RGBA16 palette (256 colors, 2 bytes each)

        if split_palette {
            // Split palette mode: first half from one location, second half from another
            let half_size = palette_size / 2;
            let mut palette = Vec::with_capacity(palette_size);

            if let Some(first_half) = buffer.get_slice(palette_offset, half_size) {
                palette.extend_from_slice(first_half);
            } else {
                palette.resize(half_size, 0);
            }

            if let Some(second_half) = buffer.get_slice(split_offset, half_size) {
                palette.extend_from_slice(second_half);
            } else {
                palette.resize(palette_size, 0);
            }

            palette
        } else {
            // Normal mode: extract contiguous palette
            if let Some(pal_data) = buffer.get_slice(palette_offset, palette_size) {
                pal_data.to_vec()
            } else {
                vec![0; palette_size]
            }
        }
    }

    pub fn save_palette_to_file(
        &self,
        buffer: &FileBuffer,
        palette_offset: usize,
        split_palette: bool,
        split_offset: usize,
    ) -> Result<()> {
        if let Some(path) = &self.external_palette_path {
            let palette_data =
                self.get_palette_data(buffer, palette_offset, split_palette, split_offset);
            fs::write(path, palette_data)?;
        }
        Ok(())
    }
}
