// src/lib.rs

pub mod files;

// Re-export items for convenient access
pub use files::bin_handler::{read_file_bytes, BinFile};
