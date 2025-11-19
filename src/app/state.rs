use crate::formats::adapter::TextureFormat;
use crate::io::loader::LoadedFile;
use crate::texture::texture::Texture;

/// Centralized application state
/// This is the "single source of truth" for the app
/// Orthogonal: No UI code, just data
#[derive(Clone)]
pub struct AppState {
    pub current_file: Option<LoadedFile>,
    pub decoded_texture: Option<Texture>,
    pub selected_format: TextureFormat,
    pub texture_width: u32,
    pub texture_height: u32,
    pub zoom_level: f32,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            current_file: None,
            decoded_texture: None,
            selected_format: TextureFormat::RGBA16,
            texture_width: 64,
            texture_height: 64,
            zoom_level: 1.0,
        }
    }

    #[allow(dead_code)] // Used for conditional UI rendering
    pub fn has_file(&self) -> bool {
        self.current_file.is_some()
    }

    #[allow(dead_code)] // Will be used for export functionality
    pub fn has_texture(&self) -> bool {
        self.decoded_texture.is_some()
    }

    pub fn file_name(&self) -> String {
        self.current_file
            .as_ref()
            .map(|f| f.name.clone())
            .unwrap_or_else(|| "Texture Viewer".to_string())
    }
}

