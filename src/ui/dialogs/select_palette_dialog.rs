use std::{fs, io::Read, path::Path};

use eframe::{
    egui::{self, Response, Sense, TextEdit, WidgetText},
    epaint::{Color32, FontFamily, FontId, Pos2, Rect, Rounding, Stroke, Vec2},
};
use egui_file::FileDialog;
use egui_modal::Modal;
use i18n_embed_fl::fl;
use icy_engine::{
    Palette, PaletteFormat, C64_DEFAULT_PALETTE, DOS_DEFAULT_PALETTE, EGA_PALETTE,
    VIEWDATA_PALETTE, XTERM_256_PALETTE,
};
use walkdir::WalkDir;

use crate::{to_message, AnsiEditor, Message, Settings, TerminalResult};

enum PaletteSource {
    BuiltIn,
    Library,
    File,
}

pub struct SelectPaletteDialog {
    palettes: Vec<(Palette, PaletteSource)>,
    selected_palette: i32,

    filter: String,
    show_builtin: bool,
    show_library: bool,
    show_file: bool,

    do_select: bool,
    edit_selected_font: bool,

    export_dialog: Option<FileDialog>,
}

impl SelectPaletteDialog {
    pub fn new(editor: &AnsiEditor) -> anyhow::Result<Self> {
        let mut palettes = Vec::new();

        let mut dos = Palette::from_colors(DOS_DEFAULT_PALETTE.to_vec());
        dos.title = fl!(
            crate::LANGUAGE_LOADER,
            "palette_selector-dos_default_palette"
        );
        palettes.push((dos, PaletteSource::BuiltIn));

        let mut dos = Palette::from_colors(C64_DEFAULT_PALETTE.to_vec());
        dos.title = fl!(
            crate::LANGUAGE_LOADER,
            "palette_selector-c64_default_palette"
        );
        palettes.push((dos, PaletteSource::BuiltIn));

        let mut dos = Palette::from_colors(EGA_PALETTE.to_vec());
        dos.title = fl!(
            crate::LANGUAGE_LOADER,
            "palette_selector-ega_default_palette"
        );
        palettes.push((dos, PaletteSource::BuiltIn));

        let mut dos = Palette::from_colors(XTERM_256_PALETTE.map(|p| p.1).to_vec());
        dos.title = fl!(
            crate::LANGUAGE_LOADER,
            "palette_selector-xterm_default_palette"
        );
        palettes.push((dos, PaletteSource::BuiltIn));

        let mut dos = Palette::from_colors(VIEWDATA_PALETTE[0..8].to_vec());
        dos.title = fl!(
            crate::LANGUAGE_LOADER,
            "palette_selector-viewdata_default_palette"
        );
        palettes.push((dos, PaletteSource::BuiltIn));

        let mut pal = editor.buffer_view.lock().get_buffer().palette.clone();
        let mut selected_palette = 0;
        if let Some(idx) = palettes.iter().position(|p| p.0.are_colors_equal(&pal)) {
            selected_palette = idx as i32;
        } else {
            if pal.title.is_empty() {
                pal.title = fl!(
                    crate::LANGUAGE_LOADER,
                    "palette_selector-extracted_from_buffer_default_label"
                );
            }
            palettes.insert(0, (pal, PaletteSource::File));
        }
        if let Ok(palette_dir) = Settings::get_palettes_diretory() {
            for palette in SelectPaletteDialog::load_palettes(palette_dir.as_path())? {
                palettes.push((palette, PaletteSource::Library));
            }
        }

        Ok(Self {
            do_select: false,
            palettes,
            selected_palette,
            filter: String::new(),
            show_builtin: true,
            show_library: true,
            show_file: true,
            edit_selected_font: false,
            export_dialog: None,
        })
    }

    fn load_palettes(tdf_dir: &Path) -> anyhow::Result<Vec<Palette>> {
        let mut palettes = Vec::new();
        let walker = WalkDir::new(tdf_dir).into_iter();
        for entry in walker.filter_entry(|e| !crate::model::font_imp::FontTool::is_hidden(e)) {
            if let Err(e) = entry {
                log::error!("Can't load palette library: {e}");
                break;
            }
            let entry = entry.unwrap();
            let path = entry.path();

            if path.is_dir() {
                continue;
            }
            let extension = path.extension();
            if extension.is_none() {
                continue;
            }
            let extension = extension.unwrap().to_str();
            if extension.is_none() {
                continue;
            }

            if let Ok(palette) = Palette::import_palette(path, &fs::read(path)?) {
                palettes.push(palette);
            }
            let ext = extension.unwrap().to_lowercase();
            if ext == "zip" {
                match fs::File::open(path) {
                    Ok(mut file) => {
                        let mut data = Vec::new();
                        file.read_to_end(&mut data).unwrap_or_default();
                        SelectPaletteDialog::read_zip_archive(data, &mut palettes);
                    }

                    Err(err) => {
                        log::error!("Failed to open zip file: {}", err);
                        return Err(err.into());
                    }
                }
            }
        }
        Ok(palettes)
    }

    fn read_zip_archive(data: Vec<u8>, palettes: &mut Vec<Palette>) {
        let file = std::io::Cursor::new(data);
        match zip::ZipArchive::new(file) {
            Ok(mut archive) => {
                for i in 0..archive.len() {
                    match archive.by_index(i) {
                        Ok(mut file) => {
                            if let Some(name) = file.enclosed_name() {
                                let file_name_buf = name.to_path_buf();
                                let file_name =
                                    file_name_buf.to_string_lossy().to_ascii_lowercase();
                                let mut data = Vec::new();
                                file.read_to_end(&mut data).unwrap_or_default();

                                if file_name.ends_with(".zip") {
                                    SelectPaletteDialog::read_zip_archive(data, palettes);
                                } else if let Ok(palette) =
                                    Palette::import_palette(&file_name_buf, &data)
                                {
                                    palettes.push(palette);
                                }
                            }
                        }
                        Err(err) => {
                            log::error!("Error reading zip file: {}", err);
                        }
                    }
                }
            }
            Err(err) => {
                log::error!("Error reading zip archive: {}", err);
            }
        }
    }

    pub fn draw_palette_row(
        &mut self,
        ui: &mut egui::Ui,
        cur_pal: usize,
        row_height: f32,
        is_selected: bool,
    ) -> Response {
        let palette = &self.palettes[cur_pal];
        let (id, rect) = ui.allocate_space([ui.available_width(), row_height].into());
        let response = ui.interact(rect, id, Sense::click());

        if response.hovered() {
            ui.painter().rect_filled(
                rect.expand(1.0),
                Rounding::same(4.0),
                ui.style().visuals.widgets.active.bg_fill,
            );
        } else if is_selected {
            ui.painter().rect_filled(
                rect.expand(1.0),
                Rounding::same(4.0),
                ui.style().visuals.extreme_bg_color,
            );
        }

        let text_color = if is_selected {
            ui.style().visuals.strong_text_color()
        } else {
            ui.style().visuals.text_color()
        };

        let font_id = FontId::new(14.0, FontFamily::Proportional);
        let text: WidgetText = palette.0.title.clone().into();
        let galley = text.into_galley(ui, Some(false), f32::INFINITY, font_id);
        ui.painter().galley_with_color(
            egui::Align2::LEFT_TOP
                .align_size_within_rect(galley.size(), rect.shrink(4.0))
                .min,
            galley.galley,
            text_color,
        );

        let mut color_rect = rect;
        color_rect.set_top(rect.top() + 22.0);
        color_rect.set_height(rect.height() - 32.0);

        let mut num_colors = 8;
        while (palette.0.len() as f32 / num_colors as f32).ceil() > 6.0 {
            num_colors += 8;
        }
        // paint palette preview
        let w = color_rect.width() / num_colors as f32;
        let h = color_rect.height() / (palette.0.len() as f32 / num_colors as f32).ceil().max(1.0);

        for i in 0..palette.0.len() {
            let (r, g, b) = palette.0.get_rgb(i);
            let rect = Rect::from_min_size(
                Pos2::new(
                    color_rect.left() + (i % num_colors) as f32 * w,
                    color_rect.top() + (i / num_colors) as f32 * h,
                ),
                Vec2::new(w, h),
            );
            ui.painter()
                .rect_filled(rect, Rounding::none(), Color32::from_rgb(r, g, b))
        }

        // paint palette tag
        let font_type = match palette.1 {
            PaletteSource::BuiltIn => {
                fl!(crate::LANGUAGE_LOADER, "font_selector-builtin_font")
            }
            PaletteSource::Library => {
                fl!(crate::LANGUAGE_LOADER, "font_selector-library_font")
            }
            PaletteSource::File => {
                fl!(crate::LANGUAGE_LOADER, "font_selector-file_font")
            }
        };

        let font_id = FontId::new(12.0, FontFamily::Proportional);
        let text: WidgetText = font_type.into();
        let galley = text.into_galley(ui, Some(false), f32::INFINITY, font_id);

        let rect = Rect::from_min_size(
            Pos2::new(
                (rect.right() - galley.size().x - 10.0).floor(),
                (rect.top() + 8.0).floor(),
            ),
            galley.size(),
        );
        ui.painter().rect_filled(
            rect.expand(2.0),
            Rounding::same(4.0),
            ui.style().visuals.widgets.active.bg_fill,
        );

        ui.painter()
            .rect_stroke(rect.expand(2.0), 4.0, Stroke::new(1.0, text_color));

        ui.painter().galley_with_color(
            egui::Align2::CENTER_CENTER
                .align_size_within_rect(galley.size(), rect)
                .min,
            galley.galley,
            text_color,
        );

        if palette.0.description.is_empty() {
            response
        } else {
            response.on_hover_ui(|ui| {
                ui.small(palette.0.description.clone());
            })
        }
    }
}

impl crate::ModalDialog for SelectPaletteDialog {
    fn show(&mut self, ctx: &egui::Context) -> bool {
        if let Some(ed) = &mut self.export_dialog {
            if ed.show(ctx).selected() {
                if let Some(res) = ed.path() {
                    let s = self.selected_palette as usize;
                    if s < self.palettes.len() {
                        let res = res.with_extension("gpl");
                        let data = self.palettes[s].0.export_palette(&PaletteFormat::Gpl);
                        if let Err(err) = fs::write(res, data) {
                            log::error!("Error exporting palette: {err}");
                        }
                    }
                }
                self.export_dialog = None
            } else {
                return false;
            }
        }

        let mut result = false;
        let modal = Modal::new(ctx, "select_font_dialog2");
        let palette_count = self.palettes.len();
        modal.show(|ui| {
            modal.title(
                ui,
                fl!(
                    crate::LANGUAGE_LOADER,
                    "select-font-dialog-title",
                    fontcount = palette_count
                ),
            );
            modal.frame(ui, |ui| {
                let row_height = 200.0 / 2.0;
                ui.horizontal(|ui: &mut egui::Ui| {
                    ui.add_sized(
                        [250.0, 20.0],
                        TextEdit::singleline(&mut self.filter).hint_text(fl!(
                            crate::LANGUAGE_LOADER,
                            "select-font-dialog-filter-text"
                        )),
                    );
                    let response = ui.button("🗙");
                    if response.clicked() {
                        self.filter.clear();
                    }

                    let response = ui.selectable_label(
                        self.show_library,
                        fl!(crate::LANGUAGE_LOADER, "font_selector-library_font"),
                    );
                    if response.clicked() {
                        self.show_library = !self.show_library;
                    }

                    let response = ui.selectable_label(
                        self.show_builtin,
                        fl!(crate::LANGUAGE_LOADER, "font_selector-builtin_font"),
                    );
                    if response.clicked() {
                        self.show_builtin = !self.show_builtin;
                    }
                });
                ui.add_space(4.0);

                let mut filtered_fonts = Vec::new();

                for i in 0..palette_count {
                    let palette = &self.palettes[i];
                    let match_filter = match palette.1 {
                        PaletteSource::BuiltIn => self.show_builtin,
                        PaletteSource::Library => self.show_library,
                        PaletteSource::File => self.show_file,
                    };

                    if palette
                        .0
                        .title
                        .to_lowercase()
                        .contains(&self.filter.to_lowercase())
                        && match_filter
                    {
                        filtered_fonts.push(i);
                    }
                }
                if filtered_fonts.is_empty() {
                    if palette_count == 0 {
                        ui.label(fl!(
                            crate::LANGUAGE_LOADER,
                            "select-font-dialog-no-fonts-installed"
                        ));
                    } else {
                        ui.label(fl!(crate::LANGUAGE_LOADER, "select-font-dialog-no-fonts"));
                    }
                } else {
                    egui::ScrollArea::vertical().max_height(300.).show_rows(
                        ui,
                        row_height,
                        filtered_fonts.len(),
                        |ui, range| {
                            for row in range {
                                let is_selected =
                                    self.selected_palette == filtered_fonts[row] as i32;
                                let response = self.draw_palette_row(
                                    ui,
                                    filtered_fonts[row],
                                    row_height,
                                    is_selected,
                                );

                                if response.clicked() {
                                    self.selected_palette = filtered_fonts[row] as i32;
                                }
                                if response.double_clicked() {
                                    self.selected_palette = filtered_fonts[row] as i32;
                                    self.do_select = true;
                                    result = true;
                                }
                            }
                        },
                    );
                }
            });

            modal.buttons(ui, |ui| {
                if ui
                    .button(fl!(crate::LANGUAGE_LOADER, "select-font-dialog-select"))
                    .clicked()
                {
                    self.do_select = true;
                    result = true;
                }
                if ui
                    .button(fl!(crate::LANGUAGE_LOADER, "new-file-cancel"))
                    .clicked()
                {
                    result = true;
                }

                if ui
                    .button(fl!(crate::LANGUAGE_LOADER, "export-button-title"))
                    .clicked()
                {
                    let mut initial_path = None;
                    crate::set_default_initial_directory_opt(&mut initial_path);
                    let mut dialog = FileDialog::save_file(initial_path);
                    dialog.open();
                    self.export_dialog = Some(dialog);
                }
            });
        });
        modal.open();
        result
    }

    fn should_commit(&self) -> bool {
        self.do_select || self.edit_selected_font
    }

    fn commit(&self, editor: &mut AnsiEditor) -> TerminalResult<Option<Message>> {
        if let Some((palette, _)) = self.palettes.get(self.selected_palette as usize) {
            Ok(to_message(
                editor
                    .buffer_view
                    .lock()
                    .get_edit_state_mut()
                    .switch_to_palette(palette.clone()),
            ))
        } else {
            Ok(None)
        }
    }
}