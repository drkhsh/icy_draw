use std::{
    path::{Path, PathBuf},
    sync::Arc,
};

use eframe::egui::{self, RichText};
use icy_engine::{
    editor::UndoState, BitFont, Buffer, EngineResult, FontGlyph, Layer, Size, TextPane, TheDrawFont,
};

use crate::{
    model::Tool, AnsiEditor, BitFontEditor, ClipboardHandler, Document, DocumentOptions,
    DrawGlyphStyle, Message, TerminalResult,
};

pub struct CharFontEditor {
    font: BitFont,
    selected_char_opt: Option<char>,
    old_selected_char_opt: Option<char>,
    file_name: Option<PathBuf>,
    ansi_editor: AnsiEditor,
    is_dirty: bool,
    selected_font: usize,
    fonts: Vec<TheDrawFont>,
}

impl ClipboardHandler for CharFontEditor {
    fn can_cut(&self) -> bool {
        self.ansi_editor.can_cut()
    }
    fn cut(&mut self) -> EngineResult<()> {
        self.ansi_editor.cut()
    }

    fn can_copy(&self) -> bool {
        self.ansi_editor.can_copy()
    }

    fn copy(&mut self) -> EngineResult<()> {
        self.ansi_editor.copy()
    }

    fn can_paste(&self) -> bool {
        self.ansi_editor.can_paste()
    }

    fn paste(&mut self) -> EngineResult<()> {
        self.ansi_editor.paste()
    }
}

impl UndoState for CharFontEditor {
    fn undo_description(&self) -> Option<String> {
        self.ansi_editor.undo_description()
    }

    fn can_undo(&self) -> bool {
        self.ansi_editor.can_undo()
    }

    fn undo(&mut self) -> EngineResult<()> {
        self.ansi_editor.undo()
    }

    fn redo_description(&self) -> Option<String> {
        self.ansi_editor.redo_description()
    }

    fn can_redo(&self) -> bool {
        self.ansi_editor.can_redo()
    }

    fn redo(&mut self) -> EngineResult<()> {
        self.ansi_editor.redo()
    }
}

impl Document for CharFontEditor {
    fn get_title(&self) -> String {
        if let Some(file_name) = &self.file_name {
            file_name.file_name().unwrap().to_str().unwrap().to_string()
        } else {
            "Untitled".to_string()
        }
    }

    fn is_dirty(&self) -> bool {
        self.is_dirty
    }

    fn undo_stack_len(&self) -> usize {
        self.ansi_editor.undo_stack_len()
    }

    fn get_bytes(&mut self, _path: &Path) -> TerminalResult<Vec<u8>> {
        self.save_old_selected_char();
        TheDrawFont::create_font_bundle(&self.fonts)
    }

    fn show_ui(
        &mut self,
        ui: &mut egui::Ui,
        cur_tool: &mut Box<dyn Tool>,
        selected_tool: usize,
        options: &DocumentOptions,
    ) -> Option<Message> {
        egui::ComboBox::from_id_source("combobox1")
            .selected_text(RichText::new(
                self.fonts[self.selected_font].name.to_string(),
            ))
            .show_ui(ui, |ui| {
                let mut changed = false;
                let mut select_font = 0;
                for (i, font) in self.fonts.iter().enumerate() {
                    if ui
                        .selectable_value(&mut self.selected_font, i, &font.name)
                        .clicked()
                    {
                        select_font = i;
                        changed = true;
                    }
                }
                if changed {
                    self.save_old_selected_char();
                    self.selected_font = select_font;
                    self.old_selected_char_opt = None;
                    self.selected_char_opt = None;
                    self.show_selected_char();
                }
            });

        self.show_char_selector(ui);
        let attr = self
            .ansi_editor
            .buffer_view
            .lock()
            .get_edit_state()
            .get_caret()
            .get_attribute();

        for layer in &mut self
            .ansi_editor
            .buffer_view
            .lock()
            .get_edit_state_mut()
            .get_buffer_mut()
            .layers
        {
            match self.fonts[self.selected_font].font_type {
                icy_engine::FontType::Outline | icy_engine::FontType::Block => {
                    layer.forced_output_attribute = Some(attr);
                }
                icy_engine::FontType::Color => {
                    layer.forced_output_attribute = None;
                }
            }
        }

        self.ansi_editor
            .show_ui(ui, cur_tool, selected_tool, options);
        None
    }

    fn get_ansi_editor_mut(&mut self) -> Option<&mut AnsiEditor> {
        self.ansi_editor.get_ansi_editor_mut()
    }

    fn get_ansi_editor(&self) -> Option<&AnsiEditor> {
        self.ansi_editor.get_ansi_editor()
    }

    fn destroy(&self, gl: &glow::Context) {
        self.ansi_editor.destroy(gl);
    }
}

impl CharFontEditor {
    pub fn new(
        gl: &Arc<glow::Context>,
        file_name: Option<PathBuf>,
        id: usize,
        fonts: Vec<TheDrawFont>,
    ) -> Self {
        let mut buffer = Buffer::new(Size::new(30, 12));
        set_up_layers(&mut buffer);
        let ansi_editor = AnsiEditor::new(gl, id, buffer);

        Self {
            font: BitFont::default(),
            file_name,
            ansi_editor,
            is_dirty: false,
            selected_char_opt: None,
            old_selected_char_opt: None,
            fonts,
            selected_font: 0,
        }
    }

    pub fn show_char_selector(&mut self, ui: &mut egui::Ui) {
        egui::ScrollArea::vertical().show(ui, |ui| {
            ui.horizontal_wrapped(|ui| {
                for i in b'!'..=b'~' {
                    let ch = unsafe { char::from_u32_unchecked(i as u32) };
                    let mut style = DrawGlyphStyle::Normal;
                    if !self.fonts[self.selected_font].has_char(i) {
                        style = DrawGlyphStyle::GrayOut
                    }
                    if let Some(ch2) = self.selected_char_opt {
                        if ch == ch2 {
                            style = DrawGlyphStyle::Selected
                        }
                    }
                    let response = BitFontEditor::draw_glyph(ui, &self.font, style, ch);
                    if response.clicked() {
                        self.selected_char_opt = Some(ch);
                        self.show_selected_char();
                    }
                }
            });
        });
    }

    fn show_selected_char(&mut self) {
        self.save_old_selected_char();

        let lock = &mut self.ansi_editor.buffer_view.lock();
        let edit_state = &mut lock.get_edit_state_mut();
        set_up_layers(edit_state.get_buffer_mut());
        edit_state.set_current_layer(1);
        edit_state.get_caret_mut().set_position((0, 0).into());
        if let Some(ch) = self.selected_char_opt {
            let font = &self.fonts[self.selected_font];
            font.render(edit_state, ch as u8);
        }
        edit_state.get_undo_stack().lock().unwrap().clear();
        self.old_selected_char_opt = self.selected_char_opt;
    }

    fn save_old_selected_char(&mut self) {
        if let Some(font) = self.fonts.get_mut(self.selected_font) {
            if let Some(ch) = self.old_selected_char_opt {
                match font.font_type {
                    icy_engine::FontType::Outline => {
                        log::warn!("TODO: save old selected char for outline fonts");
                    }
                    icy_engine::FontType::Block => {
                        let lock = &mut self.ansi_editor.buffer_view.lock();
                        let buf = lock.get_buffer();
                        let mut data = Vec::new();
                        let mut w = 0;
                        let mut h = 0;
                        for y in 0..buf.get_line_count() {
                            if y > 0 {
                                data.push(13);
                            }
                            let lw = buf.get_line_length(y);
                            for x in 0..lw {
                                let ch = buf.get_char((x, y));
                                data.push(ch.ch as u8);
                            }
                            w = w.max(lw);
                            h = y;
                        }

                        font.set_glyph(
                            ch,
                            FontGlyph {
                                size: Size::new(w, h),
                                data,
                            },
                        );
                    }
                    icy_engine::FontType::Color => {
                        let lock = &mut self.ansi_editor.buffer_view.lock();
                        let buf = lock.get_buffer();
                        let mut data = Vec::new();
                        let mut w = 0;
                        let mut h = 0;
                        for y in 0..buf.get_line_count() {
                            if y > 0 {
                                data.push(13);
                            }
                            let lw = buf.get_line_length(y);
                            for x in 0..lw {
                                let ch = buf.get_char((x, y));
                                data.push(ch.ch as u8);
                                data.push(ch.attribute.as_u8(icy_engine::BufferType::LegacyIce));
                            }
                            w = w.max(lw);
                            h = y;
                        }

                        font.set_glyph(
                            ch,
                            FontGlyph {
                                size: Size::new(w, h),
                                data,
                            },
                        );
                    }
                }
            }
        }
    }
}

fn set_up_layers(buffer: &mut Buffer) {
    buffer.layers.clear();

    let mut new_layer = Layer::new("background", Size::new(30, 12));
    new_layer.has_alpha_channel = false;
    new_layer.is_locked = true;
    new_layer.is_position_locked = true;
    buffer.layers.push(new_layer);

    let mut new_layer = Layer::new("edit layer", Size::new(30, 12));
    new_layer.has_alpha_channel = true;
    new_layer.is_position_locked = true;
    buffer.layers.push(new_layer);
}
