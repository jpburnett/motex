// src/lib.rs

pub mod n64_graphics;
pub mod files;

// Re-export items for convenient access
pub use n64_graphics::textures::{N64Codec, codec_name, Color};
pub use files::bin_handler::{BinFile, read_file_bytes};