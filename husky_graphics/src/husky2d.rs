use std::sync::{Arc, Mutex};

use image::{DynamicImage, Rgba, RgbaImage};
use rusttype::{Font, Scale, point};

use crate::gl_wrapper::gl_types::f32_f32;
use crate::gl_wrapper::gl_types::Texture;
use crate::gl_wrapper::mesh::{Vertex, Mesh};
use crate::gl_wrapper::shader::{Shader, ShaderProgram};

#[derive(Clone)]
pub struct Renderer2D {
    active_fontobj: String,
    font_program: ShaderProgram,
    font_mesh: Mesh,
    font_image: Arc<Mutex<RgbaImage>>,
    font_texture: Texture,
}

lazy_static! {
    static ref MAX_IMAGE_DIMENSION: u32 = {
        let mut value = 0;
        unsafe { gl::GetIntegerv(gl::MAX_TEXTURE_SIZE, &mut value) };
        value as u32
    };
}

impl Renderer2D {
    pub fn new(active_fontobj: String) -> Self {
        let font_vs = Shader::from_source(include_str!("../../shaders/vs_text.glsl"), gl::VERTEX_SHADER).expect("Failed to compile font vertex shader!");
        let font_fs = Shader::from_source(include_str!("../../shaders/fs_text.glsl"), gl::FRAGMENT_SHADER).expect("Failed to compile font fragment shader!");
        let font_program = ShaderProgram::from_shaders(vec![&font_vs, &font_fs]);

        let font_mesh_verts: Vec<Vertex> = vec![
            Vertex {
                pos: (0.0, 0.0, 0.0).into(),
                uv: (0.0, 0.0).into(),
                rgba: (1.0, 1.0, 1.0, 1.0).into(),
            },
            Vertex {
                pos: (1.0, 0.0, 0.0).into(),
                uv: (1.0, 0.0).into(),
                rgba: (1.0, 1.0, 1.0, 1.0).into(),
            },
            Vertex {
                pos: (1.0, 1.0, 0.0).into(),
                uv: (1.0, 1.0).into(),
                rgba: (1.0, 1.0, 1.0, 1.0).into(),
            },

            Vertex {
                pos: (0.0, 1.0, 0.0).into(),
                uv: (0.0, 1.0).into(),
                rgba: (1.0, 1.0, 1.0, 1.0).into(),
            },
            Vertex {
                pos: (0.0, 0.0, 0.0).into(),
                uv: (0.0, 0.0).into(),
                rgba: (1.0, 1.0, 1.0, 1.0).into(),
            },
            Vertex {
                pos: (1.0, 1.0, 0.0).into(),
                uv: (1.0, 1.0).into(),
                rgba: (1.0, 1.0, 1.0, 1.0).into(),
            },
        ];
        let font_mesh = Mesh::from_vertices(&font_mesh_verts);

        let image = DynamicImage::new_rgba8(1280, 720).to_rgba8();
        let texture = Texture::from_ptr((1280, 720), std::ptr::null(), gl::RGBA as i32, gl::RGBA, gl::UNSIGNED_BYTE);

        Self {
            active_fontobj: active_fontobj,
            font_program: font_program,
            font_mesh: font_mesh,
            font_image: Arc::new(Mutex::new(image)),
            font_texture: texture,
        }
    }

    pub fn finish_frame(&self) {
        let win_size: (u32, u32) = {
            let raw_win_size = crate::WINDOW_SIZE.lock().unwrap();
            (raw_win_size.0, raw_win_size.1)
        };

        let ortho_matrix = glam::Mat4::orthographic_rh_gl(0.0, win_size.0 as f32, win_size.1 as f32, 0.0, -1.0, 1.0);

        {
            let image_lock = self.font_image.lock().unwrap();
            self.font_texture.bind();
            self.font_texture.data(image_lock.as_raw());
            self.font_texture.unbind();
            // Texture::new((1280, 720), image_lock.as_raw(), gl::RGBA as i32, gl::RGBA, gl::UNSIGNED_BYTE)
        }

        //Render textured quad with the above image
        self.font_program.bind();
        self.font_texture.bind();
        self.font_program.uniform("offset", f32_f32::from((0f32, 0f32)));
        self.font_program.uniform("ortho", ortho_matrix);
        self.font_program.uniform("scale", f32_f32::from( (win_size.0 as f32, win_size.1 as f32) ));
        self.font_mesh.draw();
        self.font_texture.unbind();
        self.font_program.unbind();
    }

    pub fn gfx_print(&self, font: &Font, text: &str, xoff: f32, yoff: f32) {
        let font_size = 24.0; //TODO: Don't hardcode this???? Also copy the line below to wherever you start passing this to calculate it properly
        // let scale = (font_size * window_ctx.window().scale_factor() as f32).round();
        let scale = Scale::uniform(font_size);

        let font_colour = (255, 255, 255);

        let v_metrics = font.v_metrics(scale);

        let glyphs: Vec<_> = font.layout(text, scale, point(0f32, v_metrics.ascent)).collect();
        let glyphs_height = (v_metrics.ascent - v_metrics.descent).ceil() as u32;
        let glyphs_width = {
            // let min_x = glyphs
            //     .first()
            //     .map(|g| g.pixel_bounding_box().unwrap().min.x)
            //     .unwrap();
            let max_x = glyphs
                .last()
                .map(|g| g.pixel_bounding_box().unwrap().max.x)
                .unwrap();
            // (max_x - min_x) as u32
            max_x as u32
        };

        {
            let mut image_lock = self.font_image.lock().unwrap();
            for glyph in glyphs {
                if let Some(bounding_box) = glyph.pixel_bounding_box() {
                    // Draw the glyph into the image per-pixel by using the draw closure
                    glyph.draw(|x, y, v| {
                        if (x + xoff as u32 + bounding_box.min.x as u32) < image_lock.width() && (y + yoff as u32 + bounding_box.min.y as u32) < image_lock.height() {
                            image_lock.put_pixel(
                                // Offset the position by the glyph bounding box
                                x + xoff as u32 + bounding_box.min.x as u32,
                                y + yoff as u32 + bounding_box.min.y as u32,
                                // Turn the coverage into an alpha value
                                Rgba([font_colour.0, font_colour.1, font_colour.2, (v * 255.0) as u8]),
                            )
                        }
                    });
                }
            }
        }

    }
}
