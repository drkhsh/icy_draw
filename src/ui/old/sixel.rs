#![allow(clippy::many_single_char_names, clippy::collapsible_match)]
use glow::{HasContext, NativeTexture};
use icy_engine::Position;

use super::BufferView;

pub struct SixelCacheEntry {
    pub pos: Position,
    pub size: icy_engine::Size<i32>,
    pub x_scale: i32,
    pub y_scale: i32,

    pub texture: NativeTexture,
}

impl BufferView {
    pub fn update_sixels(&mut self, gl: &glow::Context) -> bool {
        let buffer = &mut self.editor.buf;
        buffer.update_sixel_threads();
        if buffer.layers[0].sixels.is_empty() {
            for sx in &self.sixel_cache {
                unsafe {
                    gl.delete_texture(sx.texture);
                }
            }
            self.sixel_cache.clear();
            return false;
        }
        if !buffer.layers[0].updated_sixels {
            return false;
        }
        for sx in &self.sixel_cache {
            unsafe {
                gl.delete_texture(sx.texture);
            }
        }
        self.sixel_cache.clear();

        let sixel_len = buffer.layers[0].sixels.lock().unwrap().len();
        //   if sixel_len == 0 {
        //     return false;
        //  }
        let mut i = 0;
        while i < sixel_len {
            let sixel = &buffer.layers[0].sixels.lock().unwrap()[i];

            unsafe {
                let texture = gl.create_texture().unwrap();
                gl.active_texture(glow::TEXTURE0 + 6);
                gl.bind_texture(glow::TEXTURE_2D, Some(texture));
                gl.tex_parameter_i32(
                    glow::TEXTURE_2D,
                    glow::TEXTURE_MIN_FILTER,
                    glow::NEAREST as i32,
                );
                gl.tex_parameter_i32(
                    glow::TEXTURE_2D,
                    glow::TEXTURE_MAG_FILTER,
                    glow::NEAREST as i32,
                );
                gl.tex_parameter_i32(
                    glow::TEXTURE_2D,
                    glow::TEXTURE_WRAP_S,
                    glow::CLAMP_TO_EDGE as i32,
                );
                gl.tex_parameter_i32(
                    glow::TEXTURE_2D,
                    glow::TEXTURE_WRAP_T,
                    glow::CLAMP_TO_EDGE as i32,
                );
                gl.tex_image_2d(
                    glow::TEXTURE_2D,
                    0,
                    glow::RGB as i32,
                    sixel.width() as i32,
                    sixel.height() as i32,
                    0,
                    glow::RGBA,
                    glow::UNSIGNED_BYTE,
                    Some(&sixel.picture_data),
                );

                let new_entry = SixelCacheEntry {
                    pos: sixel.position,
                    x_scale: sixel.horizontal_scale,
                    y_scale: sixel.vertical_scale,
                    size: icy_engine::Size {
                        width: sixel.width() as i32,
                        height: sixel.height() as i32,
                    },
                    texture,
                };
                self.sixel_cache.push(new_entry);
            }
            i += 1;
        }
        false
    }
}
