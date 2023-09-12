use std::{
    cell::RefCell,
    fs,
    path::Path,
    rc::Rc,
    sync::{Arc, Mutex},
    time::Duration,
};

use crate::{
    add_child, model::Tool, util::autosave, AnsiEditor, AskCloseFileDialog, BitFontEditor,
    CharFontEditor, CharTableToolWindow, Commands, Document, DocumentBehavior, DocumentTab,
    LayerToolWindow, Message, MinimapToolWindow, ModalDialog, Settings, ToolBehavior, ToolTab,
    TopBar,
};
use eframe::{
    egui::{self, Key, Response, SidePanel, TextStyle, Ui},
    epaint::{pos2, FontId},
};
use egui_tiles::{Container, TileId};
use glow::Context;
use i18n_embed_fl::fl;
use icy_engine::{
    BitFont, Buffer, EngineResult, Palette, Position, TextAttribute, TextPane, TheDrawFont,
};

pub struct MainWindow {
    pub document_tree: egui_tiles::Tree<DocumentTab>,
    pub tool_tree: egui_tiles::Tree<ToolTab>,
    pub toasts: egui_notify::Toasts,

    pub document_behavior: DocumentBehavior,
    pub tool_behavior: ToolBehavior,
    pub gl: Arc<Context>,

    dialog_open: bool,
    modal_dialog: Option<Box<dyn ModalDialog>>,
    id: usize,
    palette_mode: usize,
    pub is_closed: bool,
    pub top_bar: TopBar,
    pub left_panel: bool,
    pub right_panel: bool,
    pub bottom_panel: bool,

    pub commands: Commands,
    pub is_fullscreen: bool,
}

pub const PASTE_TOOL: usize = 0;
pub const FIRST_TOOL: usize = 1;
pub const BRUSH_TOOL: usize = 3;

impl MainWindow {
    pub fn create_id(&mut self) -> usize {
        self.id += 1;
        self.id
    }

    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let mut fnt = crate::model::font_imp::FontTool {
            selected_font: Arc::new(Mutex::new(0)),
            fonts: Arc::new(Mutex::new(Vec::new())),
            sizes: Vec::new(),
        };
        fnt.load_fonts();
        fnt.install_watcher();

        let tools: Vec<Box<dyn Tool>> = vec![
            Box::<crate::model::paste_tool::PasteTool>::default(),
            Box::<crate::model::click_imp::ClickTool>::default(),
            Box::<crate::model::select_imp::SelectTool>::default(),
            Box::new(crate::model::pencil_imp::PencilTool {
                use_back: true,
                use_fore: true,
                undo_op: None,
                brush_type: crate::model::pencil_imp::PencilType::Shade,
                char_code: Rc::new(RefCell::new('\u{00B0}')),
                last_pos: Position::default(),
            }),
            Box::new(crate::model::brush_imp::BrushTool {
                size: 3,
                use_back: true,
                use_fore: true,
                undo_op: None,
                custom_brush: None,
                image: None,
                brush_type: crate::model::brush_imp::BrushType::Shade,
                char_code: Rc::new(RefCell::new('\u{00B0}')),
            }),
            Box::new(crate::model::erase_imp::EraseTool {
                size: 3,
                brush_type: crate::model::erase_imp::EraseType::Shade,
                undo_op: None,
            }),
            Box::new(crate::model::pipette_imp::PipetteTool {}),
            Box::new(crate::model::line_imp::LineTool {
                draw_mode: crate::model::DrawMode::Line,
                use_fore: true,
                use_back: true,
                attr: icy_engine::TextAttribute::default(),
                char_code: Rc::new(RefCell::new('\u{00B0}')),
                old_pos: icy_engine::Position { x: 0, y: 0 },
            }),
            Box::new(crate::model::draw_rectangle_imp::DrawRectangleTool {
                draw_mode: crate::model::DrawMode::Line,
                use_fore: true,
                use_back: true,
                attr: icy_engine::TextAttribute::default(),
                char_code: Rc::new(RefCell::new('\u{00B0}')),
            }),
            Box::new(
                crate::model::draw_rectangle_filled_imp::DrawRectangleFilledTool {
                    draw_mode: crate::model::DrawMode::Line,
                    use_fore: true,
                    use_back: true,
                    attr: icy_engine::TextAttribute::default(),
                    char_code: Rc::new(RefCell::new('\u{00B0}')),
                },
            ),
            Box::new(crate::model::draw_ellipse_imp::DrawEllipseTool {
                draw_mode: crate::model::DrawMode::Line,
                use_fore: true,
                use_back: true,
                attr: icy_engine::TextAttribute::default(),
                char_code: Rc::new(RefCell::new('\u{00B0}')),
            }),
            Box::new(
                crate::model::draw_ellipse_filled_imp::DrawEllipseFilledTool {
                    draw_mode: crate::model::DrawMode::Line,
                    use_fore: true,
                    use_back: true,
                    attr: icy_engine::TextAttribute::default(),
                    char_code: Rc::new(RefCell::new('\u{00B0}')),
                },
            ),
            Box::new(crate::model::fill_imp::FillTool {
                use_fore: true,
                use_back: true,
                char_code: Rc::new(RefCell::new('\u{00B0}')),
                fill_type: crate::model::fill_imp::FillType::Character,
                attr: icy_engine::TextAttribute::default(),
            }),
            Box::new(fnt),
            Box::<crate::model::move_layer_imp::MoveLayer>::default(),
        ];

        let ctx: &egui::Context = &cc.egui_ctx;

        // try to detect dark vs light mode from the host system; default to dark
        ctx.set_visuals(if dark_light::detect() == dark_light::Mode::Light {
            egui::Visuals::light()
        } else {
            egui::Visuals::dark()
        });

        let mut style: egui::Style = (*ctx.style()).clone();
        style.spacing.window_margin = egui::Margin::same(8.0);
        use egui::FontFamily::Proportional;
        use egui::TextStyle::{Body, Button, Heading, Monospace, Small};
        style.text_styles = [
            (Heading, FontId::new(24.0, Proportional)),
            (Body, FontId::new(18.0, Proportional)),
            (Monospace, FontId::new(18.0, egui::FontFamily::Monospace)),
            (Button, FontId::new(18.0, Proportional)),
            (Small, FontId::new(14.0, Proportional)),
        ]
        .into();
        ctx.set_style(style);

        let gl = cc.gl.clone().unwrap();

        let mut tool_tree = egui_tiles::Tree::<ToolTab>::empty("tool_tree");
        let layers = tool_tree
            .tiles
            .insert_pane(ToolTab::new(LayerToolWindow::default()));
        let minimap = tool_tree
            .tiles
            .insert_pane(ToolTab::new(MinimapToolWindow::new(gl.clone())));
        let char_table = tool_tree
            .tiles
            .insert_pane(ToolTab::new(CharTableToolWindow::new()));

        let tab = tool_tree.tiles.insert_tab_tile(vec![minimap, char_table]);
        let vert_id = tool_tree.tiles.insert_vertical_tile(vec![tab, layers]);
        if let Some(egui_tiles::Tile::Container(Container::Linear(linear))) =
            tool_tree.tiles.get_mut(vert_id)
        {
            linear.shares.set_share(tab, 3.0);
            linear.shares.set_share(layers, 1.25);
        }

        tool_tree.root = Some(vert_id);

        MainWindow {
            document_behavior: DocumentBehavior::new(Arc::new(Mutex::new(tools))),
            tool_behavior: ToolBehavior::default(),
            toasts: egui_notify::Toasts::default(),
            document_tree: egui_tiles::Tree::<DocumentTab>::empty("document_tree"),
            tool_tree,
            gl,
            dialog_open: false,
            modal_dialog: None,
            id: 0,
            left_panel: true,
            right_panel: true,
            bottom_panel: false,
            palette_mode: 0,
            top_bar: TopBar::new(&cc.egui_ctx),
            commands: Commands::default(),
            is_closed: false,
            is_fullscreen: false,
        }
    }

    pub fn open_file(&mut self, path: &Path, load_autosave: bool) {
        let mut already_open = None;
        self.enumerate_documents(|id, pane| {
            if let Some(doc_path) = pane.get_path() {
                if doc_path == *path {
                    already_open = Some(id);
                }
            }
        });
        if let Some(id) = already_open {
            self.enumerate_tabs(|_, tab| {
                if tab.children.contains(&id) {
                    tab.active = Some(id);
                }
            });
            return;
        }

        let full_path = path.to_path_buf();
        let load_path = if load_autosave {
            autosave::get_autosave_file(path)
        } else {
            path.to_path_buf()
        };
        Settings::add_recent_file(path);

        if let Some(ext) = path.extension() {
            let ext = ext.to_str().unwrap().to_ascii_lowercase();
            if "psf" == ext || "f16" == ext || "f14" == ext || "f8" == ext || "fon" == ext {
                if let Ok(data) = fs::read(&load_path) {
                    let file_name = path.file_name();
                    if file_name.is_none() {
                        return;
                    }
                    let file_name_str = file_name.unwrap().to_str().unwrap().to_string();
                    if let Ok(font) = BitFont::from_bytes(&file_name_str, &data) {
                        add_child(
                            &mut self.document_tree,
                            Some(full_path),
                            Box::new(BitFontEditor::new(font)),
                        );
                        return;
                    }
                }
            }

            if "tdf" == ext {
                if let Ok(data) = fs::read(&load_path) {
                    let file_name = path.file_name();
                    if file_name.is_none() {
                        return;
                    }
                    if let Ok(fonts) = TheDrawFont::from_tdf_bytes(&data) {
                        let id = self.create_id();
                        add_child(
                            &mut self.document_tree,
                            Some(full_path),
                            Box::new(CharFontEditor::new(&self.gl, id, fonts)),
                        );
                        return;
                    }
                }
            }
        }
        if let Ok(data) = fs::read(&load_path) {
            match Buffer::from_bytes(path, true, &data) {
                Ok(mut buf) => {
                    let id = self.create_id();
                    buf.is_terminal_buffer = false;
                    buf.set_height(buf.get_line_count());
                    let editor = AnsiEditor::new(&self.gl, id, buf);
                    add_child(&mut self.document_tree, Some(full_path), Box::new(editor));
                }
                Err(err) => {
                    log::error!("Error loading file: {}", err);
                    self.toasts
                        .error(fl!(
                            crate::LANGUAGE_LOADER,
                            "error-load-file",
                            error = err.to_string()
                        ))
                        .set_duration(Some(Duration::from_secs(5)));
                }
            }
        }
    }

    pub fn get_active_pane(&mut self) -> Option<&mut DocumentTab> {
        let mut stack = vec![];

        if let Some(root) = self.document_tree.root {
            stack.push(root);
        }
        while let Some(id) = stack.pop() {
            match self.document_tree.tiles.get(id) {
                Some(egui_tiles::Tile::Pane(_)) => {
                    if let Some(egui_tiles::Tile::Pane(p)) = self.document_tree.tiles.get_mut(id) {
                        return Some(p);
                    } else {
                        return None;
                    }
                }
                Some(egui_tiles::Tile::Container(container)) => match container {
                    egui_tiles::Container::Tabs(tabs) => {
                        if let Some(active) = tabs.active {
                            stack.push(active);
                        }
                    }
                    egui_tiles::Container::Linear(l) => {
                        for child in l.children.iter() {
                            stack.push(*child);
                        }
                    }
                    egui_tiles::Container::Grid(g) => {
                        for child in g.children() {
                            stack.push(*child);
                        }
                    }
                },
                None => {}
            }
        }

        None
    }

    pub fn enumerate_documents<F>(&mut self, mut callback: F)
    where
        F: FnMut(TileId, &mut DocumentTab),
    {
        let mut stack = vec![];

        if let Some(root) = self.document_tree.root {
            stack.push(root);
        }
        while let Some(id) = stack.pop() {
            match self.document_tree.tiles.get(id) {
                Some(egui_tiles::Tile::Pane(_)) => {
                    if let Some(egui_tiles::Tile::Pane(p)) = self.document_tree.tiles.get_mut(id) {
                        callback(id, p);
                    }
                }
                Some(egui_tiles::Tile::Container(container)) => match container {
                    egui_tiles::Container::Tabs(tabs) => {
                        for child in &tabs.children {
                            stack.push(*child);
                        }
                    }
                    egui_tiles::Container::Linear(l) => {
                        for child in l.children.iter() {
                            stack.push(*child);
                        }
                    }
                    egui_tiles::Container::Grid(g) => {
                        for child in g.children() {
                            stack.push(*child);
                        }
                    }
                },
                None => {}
            }
        }
    }

    pub fn enumerate_tabs<F>(&mut self, mut callback: F)
    where
        F: FnMut(TileId, &mut egui_tiles::Tabs),
    {
        let mut stack = vec![];

        if let Some(root) = self.document_tree.root {
            stack.push(root);
        }
        while let Some(id) = stack.pop() {
            match self.document_tree.tiles.get_mut(id) {
                Some(egui_tiles::Tile::Pane(_)) => {}
                Some(egui_tiles::Tile::Container(container)) => match container {
                    egui_tiles::Container::Tabs(tabs) => {
                        callback(id, tabs);

                        for child in &tabs.children {
                            stack.push(*child);
                        }
                    }
                    egui_tiles::Container::Linear(l) => {
                        for child in l.children.iter() {
                            stack.push(*child);
                        }
                    }
                    egui_tiles::Container::Grid(g) => {
                        for child in g.children() {
                            stack.push(*child);
                        }
                    }
                },
                None => {}
            }
        }
    }

    pub fn get_active_document(&mut self) -> Option<Arc<Mutex<Box<dyn Document>>>> {
        if let Some(pane) = self.get_active_pane() {
            return Some(pane.doc.clone());
        }
        None
    }

    pub(crate) fn open_dialog<T: ModalDialog + 'static>(&mut self, dialog: T) {
        self.modal_dialog = Some(Box::new(dialog));
    }

    pub(crate) fn run_editor_command<T>(
        &mut self,
        param: T,
        func: fn(&mut MainWindow, &mut AnsiEditor, T) -> Option<Message>,
    ) {
        let mut msg = None;
        if let Some(doc) = self.get_active_document() {
            if let Ok(mut doc) = doc.lock() {
                if let Some(editor) = doc.get_ansi_editor_mut() {
                    msg = func(self, editor, param);
                }
            }
        }
        self.handle_message(msg);
    }

    pub(crate) fn handle_result<T>(&mut self, result: EngineResult<T>) {
        if let Err(err) = result {
            log::error!("Error: {}", err);
            self.toasts
                .error(fl!(
                    crate::LANGUAGE_LOADER,
                    "error-load-file",
                    error = err.to_string()
                ))
                .set_duration(Some(Duration::from_secs(5)));
        }
    }

    fn request_close_tab(&mut self, close_id: TileId) -> bool {
        if let Some(egui_tiles::Tile::Pane(pane)) = self.document_tree.tiles.get(close_id) {
            if !pane.is_dirty() {
                self.document_tree.tiles.remove(close_id);
                return true;
            } else {
                self.open_dialog(AskCloseFileDialog::new(pane.get_path(), close_id));
                return false;
            }
        }
        true
    }
}

pub fn button_with_shortcut(
    ui: &mut Ui,
    enabled: bool,
    label: impl Into<String>,
    shortcut: impl Into<String>,
) -> Response {
    ui.set_width(280.0);
    let btn_re = ui.add_enabled(enabled, egui::Button::new(label.into()));
    let font_id = TextStyle::Body.resolve(ui.style());
    let color = ui.style().visuals.noninteractive().fg_stroke.color;

    let galley = ui.fonts(|f| {
        f.layout_job(egui::text::LayoutJob::simple_singleline(
            shortcut.into(),
            font_id,
            color,
        ))
    });

    ui.painter().galley(
        pos2(
            btn_re.rect.right() - galley.size().x - 2.0,
            btn_re.rect.top() + 2.0,
        ),
        galley,
    );

    btn_re
}

impl eframe::App for MainWindow {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        let msg = self.show_top_bar(ctx, frame);
        self.handle_message(msg);
        if self.is_closed {
            frame.close();
        }
        SidePanel::left("left_panel")
            .exact_width(240.0)
            .resizable(false)
            .frame(egui::Frame {
                fill: ctx.style().visuals.panel_fill,
                ..Default::default()
            })
            .show_animated(ctx, self.left_panel, |ui| {
                ui.add_space(8.0);
                let mut msg = None;
                let mut palette_mode: usize = self.palette_mode;

                let mut caret_attr = TextAttribute::default();
                let mut palette = Palette::default();

                if let Some(doc) = self.get_active_document() {
                    if let Some(editor) = doc.lock().unwrap().get_ansi_editor() {
                        caret_attr = editor.buffer_view.lock().get_caret().get_attribute();
                        palette = editor.buffer_view.lock().get_buffer().palette.clone();
                    }
                }

                /*
                   let caret_attr = editor.buffer_view.lock().get_caret().get_attribute();
                   let palette = editor.buffer_view.lock().get_buffer().palette.clone();
                */
                ui.vertical_centered(|ui| {
                    msg = crate::palette_switcher(ctx, ui, &caret_attr, &palette);
                });
                ui.add_space(8.0);
                ui.horizontal(|ui| {
                    ui.add_space(8.0);
                    if ui
                        .selectable_label(
                            palette_mode == 0,
                            fl!(crate::LANGUAGE_LOADER, "color-dos"),
                        )
                        .clicked()
                    {
                        palette_mode = 0;
                    }
                    if ui
                        .selectable_label(
                            palette_mode == 1,
                            fl!(crate::LANGUAGE_LOADER, "color-ext"),
                        )
                        .clicked()
                    {
                        palette_mode = 1;
                    }
                    if ui
                        .selectable_label(
                            palette_mode == 2,
                            fl!(crate::LANGUAGE_LOADER, "color-custom"),
                        )
                        .clicked()
                    {
                        palette_mode = 2;
                    }
                });
                ui.separator();
                let msg2 = match palette_mode {
                    0 => crate::palette_editor_16(ui, &caret_attr, &palette),
                    1 => crate::show_extended_palette(ui),
                    _ => crate::show_custom_palette(ui),
                };

                if msg.is_none() {
                    msg = msg2;
                }
                ui.separator();

                self.palette_mode = palette_mode;
                self.handle_message(msg);

                crate::add_tool_switcher(ctx, ui, self);
                let mut tool_result = None;
                if let Some(tool) = self
                    .document_behavior
                    .tools
                    .clone()
                    .lock()
                    .unwrap()
                    .get_mut(self.document_behavior.selected_tool)
                {
                    ui.horizontal(|ui| {
                        ui.add_space(4.0);
                        ui.vertical(|ui| {
                            let mut shown = false;
                            if let Some(doc) = self.get_active_document() {
                                if let Some(editor) = doc.lock().unwrap().get_ansi_editor() {
                                    shown = true;
                                    tool_result = tool.show_ui(ctx, ui, Some(editor))
                                }
                            }
                            if !shown {
                                tool_result = tool.show_ui(ctx, ui, None);
                            }
                        });
                    });
                }
                // can't handle message inside the lock
                self.handle_message(tool_result);
            });

        let panel_frame = egui::Frame {
            fill: ctx.style().visuals.panel_fill,
            ..Default::default()
        };

        egui::SidePanel::right("right_panel")
            .frame(panel_frame)
            .exact_width(260.0)
            .resizable(false)
            .show_animated(ctx, self.right_panel, |ui| {
                self.tool_behavior.active_document = self.get_active_document();
                self.tool_tree.ui(&mut self.tool_behavior, ui);
                self.tool_behavior.active_document = None;
                let msg = self.tool_behavior.message.take();
                self.handle_message(msg);
            });

        egui::CentralPanel::default()
            .frame(egui::Frame {
                fill: ctx.style().visuals.panel_fill,
                ..Default::default()
            })
            .show(ctx, |ui| {
                ui.set_width(ui.available_width() - 250.0);
                self.document_tree.ui(&mut self.document_behavior, ui);

                if let Some(doc) = self.get_active_document() {
                    if let Some(editor) = doc.lock().unwrap().get_ansi_editor() {
                        let lock = &mut editor.buffer_view.lock();
                        let last = lock.get_buffer().layers.len().saturating_sub(1);
                        if let Some(layer) = lock.get_buffer().layers.last() {
                            if layer.role.is_paste()
                                && self.document_behavior.selected_tool != PASTE_TOOL
                            {
                                self.document_behavior.tools.lock().unwrap()[PASTE_TOOL] =
                                    Box::new(crate::model::paste_tool::PasteTool::new(
                                        self.document_behavior.selected_tool,
                                    ));
                                self.document_behavior.selected_tool = PASTE_TOOL;

                                lock.get_edit_state_mut().set_current_layer(last);
                            }
                        }
                    }
                }
            });
        self.dialog_open = false;
        let mut dialog_message = None;
        if self.modal_dialog.is_some() {
            self.dialog_open = true;
            if self.modal_dialog.as_mut().unwrap().show(ctx) {
                let modal_dialog = self.modal_dialog.take().unwrap();
                if modal_dialog.should_commit() {
                    if let Some(doc) = self.get_active_document() {
                        if let Some(editor) = doc.lock().unwrap().get_ansi_editor_mut() {
                            match modal_dialog.commit(editor) {
                                Ok(msg) => {
                                    dialog_message = msg;
                                }
                                Err(err) => {
                                    log::error!("Error: {}", err);
                                    self.toasts
                                        .error(format!("{err}"))
                                        .set_duration(Some(Duration::from_secs(5)));
                                }
                            }
                        }
                    }
                    match modal_dialog.commit_self(self) {
                        Ok(msg) => {
                            if dialog_message.is_none() {
                                dialog_message = msg;
                            }
                        }
                        Err(err) => {
                            log::error!("Error: {}", err);
                            self.toasts
                                .error(format!("{err}"))
                                .set_duration(Some(Duration::from_secs(5)));
                        }
                    }
                }
            }
            if self.modal_dialog.is_some() && ctx.input(|i| i.key_pressed(Key::Escape)) {
                self.modal_dialog = None;
            }
        }
        self.handle_message(dialog_message);

        self.toasts.show(ctx);
        if let Some(close_id) = self.document_behavior.request_close.take() {
            self.request_close_tab(close_id);
        }

        if let Some(close_id) = self.document_behavior.request_close_all.take() {
            let mut open_tab = Vec::new();
            self.enumerate_tabs(|_, tab| {
                if tab.children.contains(&close_id) {
                    open_tab = tab.children.clone();
                }
            });
            for t in open_tab {
                if !self.request_close_tab(t) {
                    break;
                }
            }
        }

        if let Some(close_id) = self.document_behavior.request_close_others.take() {
            let mut open_tab = Vec::new();
            self.enumerate_tabs(|_, tab| {
                if tab.children.contains(&close_id) {
                    open_tab = tab.children.clone();
                }
            });
            for t in open_tab {
                if t != close_id && !self.request_close_tab(t) {
                    break;
                }
            }
        }

        let mut msg = self.document_behavior.message.take();
        self.commands.check(ctx, &mut msg);
        self.handle_message(msg);
        self.handle_message(read_outline_keys(ctx));

        ctx.input(|i| {
            for f in &i.raw.dropped_files {
                if let Some(path) = &f.path {
                    self.open_file(path, false);
                }
            }
            for evt in &i.events.clone() {
                if let eframe::egui::Event::Zoom(vec) = evt {
                    let scale = self.document_behavior.document_options.get_scale() * *vec;
                    self.document_behavior.document_options.set_scale(scale);
                }
            }
        });

        frame.set_fullscreen(self.is_fullscreen);

        ctx.request_repaint_after(Duration::from_millis(150));
    }

    fn on_exit(&mut self, _gl: Option<&glow::Context>) {
        /* TODO

        self.enumerate_documents( move |doc| {
            doc.destroy(gl);
        });*/
    }
}

fn read_outline_keys(ctx: &egui::Context) -> Option<Message> {
    let mut result = None;

    if ctx.input(|i| i.key_pressed(Key::F1) && check_base_f_key_modifier(i)) {
        result = Some(Message::SelectOutline(0));
    }
    if ctx.input(|i| i.key_pressed(Key::F2) && check_base_f_key_modifier(i)) {
        result = Some(Message::SelectOutline(1));
    }
    if ctx.input(|i| i.key_pressed(Key::F3) && check_base_f_key_modifier(i)) {
        result = Some(Message::SelectOutline(2));
    }
    if ctx.input(|i| i.key_pressed(Key::F4) && check_base_f_key_modifier(i)) {
        result = Some(Message::SelectOutline(3));
    }
    if ctx.input(|i| i.key_pressed(Key::F5) && check_base_f_key_modifier(i)) {
        result = Some(Message::SelectOutline(4));
    }
    if ctx.input(|i| i.key_pressed(Key::F6) && check_base_f_key_modifier(i)) {
        result = Some(Message::SelectOutline(5));
    }
    if ctx.input(|i| i.key_pressed(Key::F7) && check_base_f_key_modifier(i)) {
        result = Some(Message::SelectOutline(6));
    }
    if ctx.input(|i| i.key_pressed(Key::F8) && check_base_f_key_modifier(i)) {
        result = Some(Message::SelectOutline(7));
    }
    if ctx.input(|i| i.key_pressed(Key::F9) && check_base_f_key_modifier(i)) {
        result = Some(Message::SelectOutline(8));
    }
    if ctx.input(|i| i.key_pressed(Key::F10) && check_base_f_key_modifier(i)) {
        result = Some(Message::SelectOutline(9));
    }
    if ctx.input(|i| i.key_pressed(Key::F11) && check_base_f_key_modifier(i)) {
        result = Some(Message::SelectOutline(10));
    }
    if ctx.input(|i| i.key_pressed(Key::F12) && check_base_f_key_modifier(i)) {
        result = Some(Message::SelectOutline(11));
    }

    if ctx.input(|i| {
        i.key_pressed(Key::F1) && i.modifiers.shift && (i.modifiers.ctrl || i.modifiers.alt)
    }) {
        result = Some(Message::SelectOutline(12));
    }
    if ctx.input(|i| {
        i.key_pressed(Key::F2) && i.modifiers.shift && (i.modifiers.ctrl || i.modifiers.alt)
    }) {
        result = Some(Message::SelectOutline(13));
    }
    if ctx.input(|i| {
        i.key_pressed(Key::F3) && i.modifiers.shift && (i.modifiers.ctrl || i.modifiers.alt)
    }) {
        result = Some(Message::SelectOutline(14));
    }

    result
}

fn check_base_f_key_modifier(i: &egui::InputState) -> bool {
    i.modifiers.command_only() || (i.modifiers.alt && !i.modifiers.shift && !i.modifiers.ctrl)
}
