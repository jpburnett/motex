// src/lib.rs

pub mod n64_graphics;

// Re-export items for convenient access
pub use n64_graphics::textures::{N64Codec, codec_name, Color};