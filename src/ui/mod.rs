pub mod ansi_editor;

mod main_window;
use std::error::Error;

use eframe::egui;
pub use main_window::*;

mod document;
pub use document::*;

mod font_editor;
pub use font_editor::*;

mod palette_editor;
pub use palette_editor::*;

mod tool_switcher;
pub use tool_switcher::*;

mod char_table;
pub use char_table::*;

mod icons;
pub use icons::*;

mod layer_view;
pub use layer_view::*;

mod settings;
pub use settings::*;

mod dialogs;
pub use dialogs::*;

pub type TerminalResult<T> = Result<T, Box<dyn Error>>;

pub trait ModalDialog {
    fn show(&mut self, ctx: &egui::Context) -> bool;

    fn should_commit(&self) -> bool;

    fn commit(&self, editor: &mut crate::model::Editor) -> TerminalResult<bool>;
}
