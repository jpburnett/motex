use crate::actions::AppAction;
use crate::file_buffer::FileBuffer;
use crate::palette_manager::PaletteManager;
use crate::texture_view::{ScrollMode, TextureViewState};
use anyhow::Result;
use eframe::egui;

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct Texture64App {
    #[serde(skip)]
    pub file_buffer: FileBuffer,

    pub texture_view: TextureViewState,

    #[serde(skip)]
    pub palette_manager: PaletteManager,

    // UI state
    #[serde(with = "color32_serde")]
    pub background_color: egui::Color32,
    pub scale: f32,
    pub show_pixel_info: bool,

    #[serde(skip)]
    pub hovered_pixel: Option<(usize, usize, egui::Color32)>,

    #[serde(skip)]
    pub texture_handle: Option<egui::TextureHandle>,

    #[serde(skip)]
    show_save_confirmation: bool,

    #[serde(skip)]
    pending_action: Option<PendingAction>,
}

// Custom serialization for egui::Color32
mod color32_serde {
    use eframe::egui;
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    pub fn serialize<S>(color: &egui::Color32, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        (color.r(), color.g(), color.b(), color.a()).serialize(serializer)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<egui::Color32, D::Error>
    where
        D: Deserializer<'de>,
    {
        let (r, g, b, a) = <(u8, u8, u8, u8)>::deserialize(deserializer)?;
        Ok(egui::Color32::from_rgba_unmultiplied(r, g, b, a))
    }
}

#[derive(Clone)]
enum PendingAction {
    OpenFile(std::path::PathBuf),
}

impl Default for Texture64App {
    fn default() -> Self {
        Self {
            file_buffer: FileBuffer::default(),
            texture_view: TextureViewState::default(),
            palette_manager: PaletteManager::default(),
            background_color: egui::Color32::from_rgb(40, 40, 40), // Modern Dark Gray Default
            scale: 2.0,                                            // Easier to see by default
            show_pixel_info: true,
            hovered_pixel: None,
            texture_handle: None,
            show_save_confirmation: false,
            pending_action: None,
        }
    }
}

impl Texture64App {
    pub fn process_action(&mut self, action: AppAction) {
        match action {
            AppAction::OpenFileDialog => self.ui_open_file(),
            AppAction::SaveFile => {
                let _ = self.save_file();
            }
            AppAction::InsertImageDialog => {
                let _ = self.insert_image();
            }
            AppAction::ExportTextureDialog => {
                let _ = self.export_texture();
            }
            AppAction::UpdateTexture => self.update_texture(),
            AppAction::CopyToClipboard => {
                let _ = self.copy_to_clipboard();
            }
            AppAction::AdjustOffset { delta, mode } => {
                self.texture_view
                    .adjust_offset(delta, self.file_buffer.len(), mode);
                self.update_texture();
            }
            AppAction::Quit => {
                // eframe handles close via viewport events usually
            }
        }
    }

    pub fn open_file(&mut self, path: std::path::PathBuf) -> Result<()> {
        self.file_buffer.load_file(path)?;
        self.texture_view.offset = 0;
        self.texture_handle = None;
        self.update_texture();
        Ok(())
    }

    pub fn ui_open_file(&mut self) {
        if let Some(path) = rfd::FileDialog::new()
            .add_filter("All files", &["*"])
            .pick_file()
        {
            if self.file_buffer.is_modified() {
                self.pending_action = Some(PendingAction::OpenFile(path));
                self.show_save_confirmation = true;
            } else {
                let _ = self.open_file(path);
            }
        }
    }

    pub fn save_file(&mut self) -> Result<()> {
        self.file_buffer.save()?;
        Ok(())
    }

    pub fn insert_image(&mut self) -> Result<()> {
        if let Some(path) = rfd::FileDialog::new()
            .add_filter("Image files", &["png", "jpg"])
            .pick_file()
        {
            let img = image::open(path)?;
            let rgba = img.to_rgba8();
            let n64_data = self.texture_view.encode_to_n64(&rgba)?;
            self.file_buffer
                .insert_data(self.texture_view.offset, &n64_data)?;
            self.update_texture();
        }
        Ok(())
    }

    pub fn export_texture(&mut self) -> Result<()> {
        if let Some(path) = rfd::FileDialog::new()
            .add_filter("PNG", &["png"])
            .save_file()
        {
            if let Some(rgba_data) = self.texture_view.get_current_rgba() {
                let img = image::RgbaImage::from_raw(
                    self.texture_view.width,
                    self.texture_view.height,
                    rgba_data,
                )
                .ok_or_else(|| anyhow::anyhow!("Failed to create image"))?;
                img.save(path)?;
            }
        }
        Ok(())
    }

    pub fn copy_to_clipboard(&mut self) -> Result<()> {
        if let Some(rgba_data) = self.texture_view.get_current_rgba() {
            let img = image::RgbaImage::from_raw(
                self.texture_view.width,
                self.texture_view.height,
                rgba_data,
            )
            .ok_or_else(|| anyhow::anyhow!("Bad image"))?;
            if let Ok(mut cb) = arboard::Clipboard::new() {
                let _ = cb.set_image(arboard::ImageData {
                    width: img.width() as usize,
                    height: img.height() as usize,
                    bytes: img.as_raw().into(),
                });
            }
        }
        Ok(())
    }

    pub fn update_texture(&mut self) {
        let palette_data = if self.texture_view.format.is_ci() {
            Some(self.palette_manager.get_palette_data(
                &self.file_buffer,
                self.texture_view.palette_offset,
                self.texture_view.split_palette_enabled,
                self.texture_view.split_palette_offset,
            ))
        } else {
            None
        };

        self.texture_view
            .update_from_buffer(&self.file_buffer, palette_data.as_deref());
    }

    fn handle_file_drop(&mut self, ctx: &egui::Context) {
        ctx.input(|i| {
            if !i.raw.dropped_files.is_empty() {
                if let Some(dropped_file) = i.raw.dropped_files.first() {
                    if let Some(path) = &dropped_file.path {
                        let _ = self.open_file(path.clone());
                    }
                }
            }
        });
    }

    // Keyboard Shortcuts for Navigation
    fn check_input(&mut self, ctx: &egui::Context) -> Vec<AppAction> {
        let mut actions = Vec::new();

        // File Shortcuts
        if ctx.input_mut(|i| i.consume_key(egui::Modifiers::CTRL, egui::Key::O)) {
            actions.push(AppAction::OpenFileDialog);
        }
        if ctx.input_mut(|i| i.consume_key(egui::Modifiers::CTRL, egui::Key::S)) {
            if self.file_buffer.path().is_some() {
                actions.push(AppAction::SaveFile);
            }
        }

        // Navigation Shortcuts
        if ctx.input(|i| i.key_pressed(egui::Key::ArrowDown)) {
            let mode = if ctx.input(|i| i.modifiers.shift) {
                ScrollMode::Pixel
            } else {
                ScrollMode::Row
            };
            actions.push(AppAction::AdjustOffset { delta: 1, mode });
        }
        if ctx.input(|i| i.key_pressed(egui::Key::ArrowUp)) {
            let mode = if ctx.input(|i| i.modifiers.shift) {
                ScrollMode::Pixel
            } else {
                ScrollMode::Row
            };
            actions.push(AppAction::AdjustOffset { delta: -1, mode });
        }
        if ctx.input(|i| i.key_pressed(egui::Key::PageDown)) {
            actions.push(AppAction::AdjustOffset {
                delta: 1,
                mode: ScrollMode::Image,
            });
        }
        if ctx.input(|i| i.key_pressed(egui::Key::PageUp)) {
            actions.push(AppAction::AdjustOffset {
                delta: -1,
                mode: ScrollMode::Image,
            });
        }

        actions
    }

    fn ui_save_confirmation(&mut self, ctx: &egui::Context) {
        if self.show_save_confirmation {
            egui::Window::new("Unsaved Changes").show(ctx, |ui| {
                ui.label("Save changes?");
                if ui.button("Save").clicked() {
                    let _ = self.save_file();
                    self.execute_pending();
                    self.show_save_confirmation = false;
                }
                if ui.button("Discard").clicked() {
                    self.execute_pending();
                    self.show_save_confirmation = false;
                }
                if ui.button("Cancel").clicked() {
                    self.pending_action = None;
                    self.show_save_confirmation = false;
                }
            });
        }
    }

    fn execute_pending(&mut self) {
        if let Some(action) = self.pending_action.take() {
            match action {
                PendingAction::OpenFile(p) => {
                    let _ = self.open_file(p);
                }
            }
        }
    }

    // Decode Palette for Visualization
    fn get_preview_palette(&self) -> Option<Vec<egui::Color32>> {
        if !self.texture_view.format.is_ci() {
            return None;
        }

        let raw_bytes = self.palette_manager.get_palette_data(
            &self.file_buffer,
            self.texture_view.palette_offset,
            self.texture_view.split_palette_enabled,
            self.texture_view.split_palette_offset,
        );

        let mut colors = Vec::new();
        for chunk in raw_bytes.chunks(2) {
            if chunk.len() == 2 {
                let val = u16::from_be_bytes([chunk[0], chunk[1]]);

                let r5 = (val >> 11) & 0x1F;
                let g5 = (val >> 6) & 0x1F;
                let b5 = (val >> 1) & 0x1F;
                let a1 = val & 0x01;

                let r8 = (r5 << 3) | (r5 >> 2);
                let g8 = (g5 << 3) | (g5 >> 2);
                let b8 = (b5 << 3) | (b5 >> 2);
                let a8 = if a1 == 1 { 255 } else { 0 };

                colors.push(egui::Color32::from_rgba_unmultiplied(
                    r8 as u8, g8 as u8, b8 as u8, a8,
                ));
            }
        }
        Some(colors)
    }
}

impl eframe::App for Texture64App {
    fn save(&mut self, _storage: &mut dyn eframe::Storage) {}

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let mut actions = self.check_input(ctx);
        self.handle_file_drop(ctx);
        self.ui_save_confirmation(ctx);

        // Calculate palette for UI
        let palette_preview = self.get_preview_palette();

        // Pass calculated palette to render_ui
        let ui_actions = crate::ui::render_ui(self, ctx, palette_preview.as_deref());
        actions.extend(ui_actions);

        for action in actions {
            self.process_action(action);
        }
    }
}
