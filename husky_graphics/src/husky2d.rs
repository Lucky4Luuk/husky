use std::sync::Mutex;

use crate::gl_wrapper;
use crate::text_util::{GlGlyphTexture, GlTextPipe, to_vertex, Vertex};

use glyph_brush::{ab_glyph::*, *};

use mlua::prelude::*;
use mlua::Table;

pub fn load_husky2d_api(lua: &Lua, api: &Table) -> LuaResult<()> {
    let gfx_print_func = lua.create_function(|_, (text, x, y): (String, f32, f32)| {
        gfx_print(&text, x, y);
        Ok(())
    })?;
    api.set("print", gfx_print_func)?;
    Ok(())
}

lazy_static! {
    static ref MAX_IMAGE_DIMENSION: u32 = {
        let mut value = 0;
        unsafe { gl::GetIntegerv(gl::MAX_TEXTURE_SIZE, &mut value) };
        value as u32
    };

    static ref DEFAULT_FONT: Mutex<GlyphBrush<Vertex>> = {
        let roboto = FontArc::try_from_slice(include_bytes!("../../fonts/RobotoMono-Regular.ttf")).expect("Failed to load font!");
        trace!("Loaded default font!");
        Mutex::new(GlyphBrushBuilder::using_font(roboto).build())
    };

    static ref DEFAULT_FONT_TEXTURE: Mutex<GlGlyphTexture> = {
        let tex = GlGlyphTexture::new(DEFAULT_FONT.lock().unwrap().texture_dimensions());
        trace!("GlGlyphTexture created!");
        Mutex::new(tex)
    };

    static ref TEXT_PIPE: Mutex<GlTextPipe> = Mutex::new(GlTextPipe::new(*crate::WINDOW_SIZE.lock().unwrap()));
}

pub fn gfx_print(text: &str, x: f32, y: f32) {
    let font_size = 18.0; //TODO: Don't hardcode this???? Also copy the line below to wherever you start passing this to calculate it properly
    // let scale = (font_size * window_ctx.window().scale_factor() as f32).round();
    let text = Text::new(&text).with_scale(font_size);
    {
        let mut font = DEFAULT_FONT.lock().expect("Failed to acquire lock on font mutex!");
        let texture: &mut GlGlyphTexture = &mut *DEFAULT_FONT_TEXTURE.lock().unwrap();

        font.queue(Section::default().add_text(text));

        let mut brush_action;
        trace!("About to enter loop!");
        loop {
            brush_action = font.process_queued(
                |rect, tex_data| unsafe {
                    // Update part of gpu texture with new glyph alpha values
                    gl::BindTexture(gl::TEXTURE_2D, texture.texture.id);
                    gl::TexSubImage2D(
                        gl::TEXTURE_2D,
                        0,
                        rect.min[0] as _,
                        rect.min[1] as _,
                        rect.width() as _,
                        rect.height() as _,
                        gl::RED,
                        gl::UNSIGNED_BYTE,
                        tex_data.as_ptr() as _,
                    );
                },
                to_vertex,
            );

            match brush_action {
                Ok(_) => break,
                Err(BrushError::TextureTooSmall { suggested, .. }) => {
                    let (new_width, new_height) = if (suggested.0 > *MAX_IMAGE_DIMENSION
                        || suggested.1 > *MAX_IMAGE_DIMENSION)
                        && (font.texture_dimensions().0 < *MAX_IMAGE_DIMENSION
                            || font.texture_dimensions().1 < *MAX_IMAGE_DIMENSION)
                    {
                        (*MAX_IMAGE_DIMENSION, *MAX_IMAGE_DIMENSION)
                    } else {
                        suggested
                    };
                    eprint!("\r                            \r");
                    eprintln!("Resizing glyph texture -> {}x{}", new_width, new_height);

                    // Recreate texture as a larger size to fit more
                    *texture = GlGlyphTexture::new((new_width, new_height));

                    font.resize_texture(new_width, new_height);
                }
            }
        }
        trace!("Loop done!");
        match brush_action.unwrap() {
            BrushAction::Draw(vertices) => TEXT_PIPE.lock().unwrap().upload_vertices(&vertices),
            BrushAction::ReDraw => {}
        }
    }
}
