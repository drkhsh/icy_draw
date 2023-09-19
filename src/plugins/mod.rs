use std::{fs, path::Path, sync::Arc};

use i18n_embed_fl::fl;
use icy_engine::{AttributedChar, Position, TextPane};
use mlua::{Lua, UserData};
use regex::Regex;
use walkdir::WalkDir;

use crate::{model::font_imp::FontTool, Settings, PLUGINS};

pub struct Plugin {
    pub title: String,
    pub text: String,
}

impl Plugin {
    pub fn load(path: &Path) -> anyhow::Result<Self> {
        let text = fs::read_to_string(path).unwrap();

        let re = Regex::new(r"--\s*Title:\s*(.*)")?;

        if let Some(cap) = re.captures(&text) {
            let title = cap.get(1).unwrap().as_str().to_string();
            return Ok(Self { title, text });
        }
        Err(anyhow::anyhow!("No plugin file"))
    }

    pub(crate) fn run_plugin(
        &self,
        _window: &mut crate::MainWindow,
        editor: &crate::AnsiEditor,
    ) -> anyhow::Result<()> {
        let lua = Lua::new();
        let globals = lua.globals();

        globals
            .set(
                "log",
                lua.create_function(move |_lua, txt: String| {
                    log::info!("{txt}");
                    Ok(())
                })?,
            )
            .unwrap();

        globals.set(
            "buf",
            LuaBufferView {
                buffer_view: editor.buffer_view.clone(),
            },
        )?;

        let sel = editor.buffer_view.lock().get_selection();

        let rect = if let Some(l) = editor.buffer_view.lock().get_edit_state().get_cur_layer() {
            l.get_rectangle()
        } else {
            return Err(anyhow::anyhow!("No layer selected"));
        };

        if let Some(sel) = sel {
            let mut selected_rect = sel.as_rectangle().intersect(&rect);
            selected_rect -= rect.start;

            globals.set("start_x", selected_rect.left())?;
            globals.set("end_x", selected_rect.right() - 1)?;
            globals.set("start_y", selected_rect.top())?;
            globals.set("end_y", selected_rect.bottom() - 1)?;
        } else {
            globals.set("start_x", 0)?;
            globals.set("end_x", rect.get_width())?;
            globals.set("start_y", 0)?;
            globals.set("end_y", rect.get_height())?;
        }
        let _undo = editor
            .buffer_view
            .lock()
            .get_edit_state_mut()
            .begin_atomic_undo(fl!(
                crate::LANGUAGE_LOADER,
                "undo-plugin",
                title = self.title.clone()
            ));
        lua.load(&self.text).exec()?;
        Ok(())
    }

    pub fn read_plugin_directory() {
        let walker = WalkDir::new(Settings::get_plugin_directory().unwrap()).into_iter();
        for entry in walker.filter_entry(|e| !FontTool::is_hidden(e)) {
            match entry {
                Ok(entry) => {
                    if entry.file_type().is_dir() {
                        continue;
                    }
                    unsafe {
                        match Plugin::load(entry.path()) {
                            Ok(plugin) => {
                                PLUGINS.push(plugin);
                            }
                            Err(err) => log::error!("Error loading plugin: {err}"),
                        }
                    }
                }
                Err(err) => log::error!("Error loading plugin: {err}"),
            }
        }
    }
}

struct LuaBufferView {
    buffer_view: Arc<eframe::epaint::mutex::Mutex<icy_engine_egui::BufferView>>,
}

impl UserData for LuaBufferView {
    fn add_fields<'lua, F: mlua::UserDataFields<'lua, Self>>(fields: &mut F) {
        fields.add_field_method_get("height", |_, this| {
            Ok(this.buffer_view.lock().get_buffer_mut().get_height())
        });
        fields.add_field_method_set("height", |_, this, val| {
            this.buffer_view.lock().get_buffer_mut().set_height(val);
            Ok(())
        });
        fields.add_field_method_get("width", |_, this| {
            Ok(this.buffer_view.lock().get_buffer_mut().get_width())
        });
        fields.add_field_method_set("width", |_, this, val| {
            this.buffer_view.lock().get_buffer_mut().set_width(val);
            Ok(())
        });

        fields.add_field_method_get("font_page", |_, this| {
            Ok(this.buffer_view.lock().get_caret_mut().get_font_page())
        });
        fields.add_field_method_set("font_page", |_, this, val| {
            this.buffer_view.lock().get_caret_mut().set_font_page(val);
            Ok(())
        });

        fields.add_field_method_get("layer", |_, this| {
            Ok(this
                .buffer_view
                .lock()
                .get_edit_state_mut()
                .get_current_layer())
        });
        fields.add_field_method_set("layer", |_, this, val| {
            if val < this.buffer_view.lock().get_buffer_mut().layers.len() {
                this.buffer_view
                    .lock()
                    .get_edit_state_mut()
                    .set_current_layer(val);
                Ok(())
            } else {
                Err(mlua::Error::SyntaxError {
                    message: format!(
                        "Layer {} out of range (0..<{})",
                        val,
                        this.buffer_view.lock().get_buffer_mut().layers.len()
                    ),
                    incomplete_input: false,
                })
            }
        });

        fields.add_field_method_get("fg", |_, this| {
            Ok(this
                .buffer_view
                .lock()
                .get_caret_mut()
                .get_attribute()
                .get_foreground())
        });
        fields.add_field_method_set("fg", |_, this, val| {
            let mut attr = this.buffer_view.lock().get_caret_mut().get_attribute();
            attr.set_foreground(val);
            this.buffer_view.lock().get_caret_mut().set_attr(attr);
            Ok(())
        });

        fields.add_field_method_get("bg", |_, this| {
            Ok(this
                .buffer_view
                .lock()
                .get_caret_mut()
                .get_attribute()
                .get_background())
        });
        fields.add_field_method_set("bg", |_, this, val| {
            let mut attr = this.buffer_view.lock().get_caret_mut().get_attribute();
            attr.set_background(val);
            this.buffer_view.lock().get_caret_mut().set_attr(attr);
            Ok(())
        });

        fields.add_field_method_get("x", |_, this| {
            Ok(this.buffer_view.lock().get_caret_mut().get_position().x)
        });
        fields.add_field_method_set("x", |_, this, val| {
            this.buffer_view.lock().get_caret_mut().set_x_position(val);
            Ok(())
        });

        fields.add_field_method_get("y", |_, this| {
            Ok(this.buffer_view.lock().get_caret_mut().get_position().y)
        });
        fields.add_field_method_set("y", |_, this, val| {
            this.buffer_view.lock().get_caret_mut().set_y_position(val);
            Ok(())
        });

        fields.add_field_method_get("layer_count", |_, this| {
            Ok(this.buffer_view.lock().get_buffer_mut().layers.len())
        });
    }

    fn add_methods<'lua, M: mlua::UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method_mut("fg_rgb", |_, this, (r, g, b): (u8, u8, u8)| {
            let color = this
                .buffer_view
                .lock()
                .get_buffer_mut()
                .palette
                .insert_color_rgb(r, g, b);
            this.buffer_view
                .lock()
                .get_caret_mut()
                .set_foreground(color);
            Ok(color)
        });

        methods.add_method_mut("bg_rgb", |_, this, (r, g, b): (u8, u8, u8)| {
            let color = this
                .buffer_view
                .lock()
                .get_buffer_mut()
                .palette
                .insert_color_rgb(r, g, b);
            this.buffer_view
                .lock()
                .get_caret_mut()
                .set_background(color);
            Ok(color)
        });

        methods.add_method_mut("set_char", |_, this, (x, y, ch): (i32, i32, u32)| {
            let cur_layer = this
                .buffer_view
                .lock()
                .get_edit_state_mut()
                .get_current_layer();
            let layer_len = this.buffer_view.lock().get_buffer_mut().layers.len();
            if cur_layer >= layer_len {
                return Err(mlua::Error::SyntaxError {
                    message: format!(
                        "Current layer {} out of range (0..<{})",
                        cur_layer, layer_len
                    ),
                    incomplete_input: false,
                });
            }
            let attr = this.buffer_view.lock().get_caret_mut().get_attribute();

            this.buffer_view
                .lock()
                .get_edit_state_mut()
                .set_char(
                    (x, y),
                    AttributedChar::new(unsafe { std::char::from_u32_unchecked(ch) }, attr),
                )
                .unwrap();
            Ok(())
        });

        methods.add_method_mut("get_char", |_, this, (x, y): (i32, i32)| {
            let cur_layer = this
                .buffer_view
                .lock()
                .get_edit_state_mut()
                .get_current_layer();
            let layer_len = this.buffer_view.lock().get_buffer_mut().layers.len();
            if cur_layer >= layer_len {
                return Err(mlua::Error::SyntaxError {
                    message: format!(
                        "Current layer {} out of range (0..<{})",
                        cur_layer, layer_len
                    ),
                    incomplete_input: false,
                });
            }

            let ch = this.buffer_view.lock().get_buffer_mut().layers[cur_layer].get_char((x, y));
            Ok(ch.ch as u32)
        });

        methods.add_method_mut("set_fg", |_, this, (x, y, col): (i32, i32, u32)| {
            let cur_layer = this
                .buffer_view
                .lock()
                .get_edit_state_mut()
                .get_current_layer();
            let layer_len = this.buffer_view.lock().get_buffer_mut().layers.len();
            if cur_layer >= layer_len {
                return Err(mlua::Error::SyntaxError {
                    message: format!(
                        "Current layer {} out of range (0..<{})",
                        cur_layer, layer_len
                    ),
                    incomplete_input: false,
                });
            }
            let mut ch =
                this.buffer_view.lock().get_buffer_mut().layers[cur_layer].get_char((x, y));
            ch.attribute.set_foreground(col);
            this.buffer_view.lock().get_buffer_mut().layers[cur_layer].set_char((x, y), ch);
            Ok(())
        });

        methods.add_method_mut("get_fg", |_, this, (x, y): (i32, i32)| {
            let cur_layer = this
                .buffer_view
                .lock()
                .get_edit_state_mut()
                .get_current_layer();
            let layer_len = this.buffer_view.lock().get_buffer_mut().layers.len();
            if cur_layer >= layer_len {
                return Err(mlua::Error::SyntaxError {
                    message: format!(
                        "Current layer {} out of range (0..<{})",
                        cur_layer, layer_len
                    ),
                    incomplete_input: false,
                });
            }

            let ch = this.buffer_view.lock().get_buffer_mut().layers[cur_layer].get_char((x, y));
            Ok(ch.attribute.get_foreground())
        });

        methods.add_method_mut("set_bg", |_, this, (x, y, col): (i32, i32, u32)| {
            let cur_layer = this
                .buffer_view
                .lock()
                .get_edit_state_mut()
                .get_current_layer();
            let layer_len = this.buffer_view.lock().get_buffer_mut().layers.len();
            if cur_layer >= layer_len {
                return Err(mlua::Error::SyntaxError {
                    message: format!(
                        "Current layer {} out of range (0..<{})",
                        cur_layer, layer_len
                    ),
                    incomplete_input: false,
                });
            }
            let mut ch =
                this.buffer_view.lock().get_buffer_mut().layers[cur_layer].get_char((x, y));
            ch.attribute.set_background(col);
            this.buffer_view.lock().get_buffer_mut().layers[cur_layer].set_char((x, y), ch);
            Ok(())
        });

        methods.add_method_mut("get_bg", |_, this, (x, y): (i32, i32)| {
            let cur_layer = this
                .buffer_view
                .lock()
                .get_edit_state_mut()
                .get_current_layer();
            let layer_len = this.buffer_view.lock().get_buffer_mut().layers.len();
            if cur_layer >= layer_len {
                return Err(mlua::Error::SyntaxError {
                    message: format!(
                        "Current layer {} out of range (0..<{})",
                        cur_layer, layer_len
                    ),
                    incomplete_input: false,
                });
            }
            let ch = this.buffer_view.lock().get_buffer_mut().layers[cur_layer].get_char((x, y));
            Ok(ch.attribute.get_background())
        });

        methods.add_method_mut("print", |_, this, str: String| {
            for c in str.chars() {
                let mut pos = this.buffer_view.lock().get_caret_mut().get_position();
                let _ = this.buffer_view.lock().get_edit_state_mut().set_char(
                    pos,
                    AttributedChar::new(c, this.buffer_view.lock().get_caret_mut().get_attribute()),
                );
                pos.x += 1;
                this.buffer_view.lock().get_caret_mut().set_position(pos);
            }
            Ok(())
        });

        methods.add_method_mut("gotoxy", |_, this, (x, y): (i32, i32)| {
            this.buffer_view
                .lock()
                .get_caret_mut()
                .set_position(Position::new(x, y));
            Ok(())
        });

        methods.add_method_mut(
            "set_layer_position",
            |_, this, (layer, x, y): (usize, i32, i32)| {
                if layer < this.buffer_view.lock().get_buffer_mut().layers.len() {
                    let _ = this
                        .buffer_view
                        .lock()
                        .get_edit_state_mut()
                        .move_layer(Position::new(x, y));
                    Ok(())
                } else {
                    Err(mlua::Error::SyntaxError {
                        message: format!(
                            "Layer {} out of range (0..<{})",
                            layer,
                            this.buffer_view.lock().get_buffer_mut().layers.len()
                        ),
                        incomplete_input: false,
                    })
                }
            },
        );
        methods.add_method_mut("get_layer_position", |_, this, layer: usize| {
            if layer < this.buffer_view.lock().get_buffer_mut().layers.len() {
                let pos = this.buffer_view.lock().get_buffer_mut().layers[layer].get_offset();
                Ok((pos.x, pos.y))
            } else {
                Err(mlua::Error::SyntaxError {
                    message: format!(
                        "Layer {} out of range (0..<{})",
                        layer,
                        this.buffer_view.lock().get_buffer_mut().layers.len()
                    ),
                    incomplete_input: false,
                })
            }
        });

        methods.add_method_mut(
            "set_layer_visible",
            |_, this, (layer, is_visible): (i32, bool)| {
                let layer = layer as usize;
                if layer < this.buffer_view.lock().get_buffer_mut().layers.len() {
                    // todo
                    this.buffer_view.lock().get_buffer_mut().layers[layer].is_visible = is_visible;
                    Ok(())
                } else {
                    Err(mlua::Error::SyntaxError {
                        message: format!(
                            "Layer {} out of range (0..<{})",
                            layer,
                            this.buffer_view.lock().get_buffer_mut().layers.len()
                        ),
                        incomplete_input: false,
                    })
                }
            },
        );

        methods.add_method_mut("get_layer_visible", |_, this, layer: usize| {
            if layer < this.buffer_view.lock().get_buffer_mut().layers.len() {
                Ok(this.buffer_view.lock().get_buffer_mut().layers[layer].is_visible)
            } else {
                Err(mlua::Error::SyntaxError {
                    message: format!(
                        "Layer {} out of range (0..<{})",
                        layer,
                        this.buffer_view.lock().get_buffer_mut().layers.len()
                    ),
                    incomplete_input: false,
                })
            }
        });

        methods.add_method_mut("clear", |_, this, ()| {
            this.buffer_view.lock().get_buffer_mut().reset_terminal();
            Ok(())
        });
    }
}