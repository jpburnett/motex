use std::io;
use std::path::{Path, PathBuf};

/// File information after loading
#[derive(Debug, Clone)]
pub struct LoadedFile {
    pub path: PathBuf,
    pub name: String,
    pub data: Vec<u8>,
    pub size: u64,
}

/// Load a file from disk
pub fn load_file(path: impl AsRef<Path>) -> io::Result<LoadedFile> {
    let path = path.as_ref();
    log::info!("Loading file: {:?}", path);

    let data = std::fs::read(path)?;
    let size = data.len() as u64;
    let name = path
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("Unknown")
        .to_string();

    Ok(LoadedFile {
        path: path.to_path_buf(),
        name,
        data,
        size,
    })
}

/// Open a file dialog and load the selected file
pub fn open_file_dialog() -> Result<LoadedFile, String> {
    let file = rfd::FileDialog::new()
        .add_filter("All Files", &["*"])
        .set_title("Open Texture File")
        .pick_file()
        .ok_or("No file selected")?;

    load_file(&file).map_err(|e| format!("Failed to load: {}", e))
}
