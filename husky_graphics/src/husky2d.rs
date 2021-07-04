use std::sync::Arc;

use crate::gl_wrapper;
use crate::text_util::{FontObject, GlGlyphTexture, GlTextPipe, to_vertex, Vertex};

use glyph_brush::{ab_glyph::*, *};

#[derive(Clone)]
pub struct Renderer2D {
    text_pipe: GlTextPipe,
    active_fontobj: String,
}

lazy_static! {
    static ref MAX_IMAGE_DIMENSION: u32 = {
        let mut value = 0;
        unsafe { gl::GetIntegerv(gl::MAX_TEXTURE_SIZE, &mut value) };
        value as u32
    };
}

impl Renderer2D {
    pub fn new(window_size: glutin::dpi::PhysicalSize<u32>, active_fontobj: String) -> Self {
        Self {
            text_pipe: GlTextPipe::new(window_size),
            active_fontobj: active_fontobj,
        }
    }

    pub fn gfx_print(&mut self, fontobj: &mut FontObject, text: &str, x: f32, y: f32) {
        let font_size = 18.0; //TODO: Don't hardcode this???? Also copy the line below to wherever you start passing this to calculate it properly
        // let scale = (font_size * window_ctx.window().scale_factor() as f32).round();
        let text = Text::new(&text).with_scale(font_size);

        let glyph_brush = &mut fontobj.glyph_brush;
        let glyph_texture = &mut fontobj.glyph_texture;

        glyph_brush.queue(Section::default().add_text(text.with_color([0.0, 0.0, 0.0, 1.0]))); //TODO: Don't hardcode this

        let mut brush_action;
        loop {
            brush_action = glyph_brush.process_queued(
                |rect, tex_data| unsafe {
                    // Update part of gpu texture with new glyph alpha values
                    gl::BindTexture(gl::TEXTURE_2D, glyph_texture.texture.id);
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
                        && (glyph_brush.texture_dimensions().0 < *MAX_IMAGE_DIMENSION
                            || glyph_brush.texture_dimensions().1 < *MAX_IMAGE_DIMENSION)
                    {
                        (*MAX_IMAGE_DIMENSION, *MAX_IMAGE_DIMENSION)
                    } else {
                        suggested
                    };
                    debug!("Resizing glyph texture -> {}x{}", new_width, new_height);

                    // Recreate texture as a larger size to fit more
                    *glyph_texture = GlGlyphTexture::new((new_width, new_height));

                    glyph_brush.resize_texture(new_width, new_height);
                }
            }
        }
        match brush_action.unwrap() {
            BrushAction::Draw(vertices) => self.text_pipe.upload_vertices(&vertices),
            BrushAction::ReDraw => {}
        }

        self.text_pipe.draw();

        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, 0);
        }
    }
}
