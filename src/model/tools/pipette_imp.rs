use std::sync::{Arc, Mutex};

use eframe::egui;

use crate::ansi_editor::BufferView;

use super::{Event, Position, Tool, ToolUiResult};
pub struct PipetteTool {}

impl Tool for PipetteTool {
    fn get_icon_name(&self) -> &'static egui_extras::RetainedImage {
        &super::icons::DROPPER_SVG
    }
    fn use_caret(&self) -> bool {
        false
    }
    fn use_selection(&self) -> bool {
        false
    }
    fn show_ui(
        &mut self,
        _ctx: &egui::Context,
        _ui: &mut egui::Ui,
        _buffer_opt: Option<std::sync::Arc<std::sync::Mutex<BufferView>>>,
    ) -> ToolUiResult {
        ToolUiResult::default()
    }

    fn handle_click(
        &mut self,
        buffer_view: Arc<Mutex<BufferView>>,
        button: i32,
        pos: Position,
    ) -> Event {
        if button == 1 {
            let editor = &mut buffer_view.lock().editor;
            let ch = editor.get_char(pos);
            editor.set_caret_attribute(ch.attribute);
        }
        Event::None
    }
}
