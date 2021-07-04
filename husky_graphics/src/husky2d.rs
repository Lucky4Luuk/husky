use image::{DynamicImage, Rgba};
use rusttype::{Font, Scale, point};

use crate::gl_wrapper::gl_types::f32_f32;
use crate::gl_wrapper::mesh::{Vertex, Mesh};
use crate::gl_wrapper::shader::{Shader, ShaderProgram};

#[derive(Clone)]
pub struct Renderer2D {
    active_fontobj: String,
    font_program: ShaderProgram,
    font_mesh: Mesh,
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
        ];
        let font_mesh_indices: Vec<u32> = vec![0,1,2];
        let font_mesh = Mesh::from_vertices(&font_mesh_verts, &font_mesh_indices);

        Self {
            active_fontobj: active_fontobj,
            font_program: font_program,
            font_mesh: font_mesh,
        }
    }

    pub fn gfx_print(&self, font: &Font, text: &str, x: f32, y: f32) {
        let font_size = 18.0; //TODO: Don't hardcode this???? Also copy the line below to wherever you start passing this to calculate it properly
        // let scale = (font_size * window_ctx.window().scale_factor() as f32).round();
        let scale = Scale::uniform(font_size);

        let font_colour = (255, 255, 255);

        let v_metrics = font.v_metrics(scale);

        let glyphs: Vec<_> = font.layout(text, scale, point(x, y + v_metrics.ascent)).collect();
        let glyphs_height = (v_metrics.ascent - v_metrics.descent).ceil() as u32;
        let glyphs_width = {
            let min_x = glyphs
                .first()
                .map(|g| g.pixel_bounding_box().unwrap().min.x)
                .unwrap();
            let max_x = glyphs
                .last()
                .map(|g| g.pixel_bounding_box().unwrap().max.x)
                .unwrap();
            (max_x - min_x) as u32
        };

        let mut image = DynamicImage::new_rgba8(glyphs_width + 40, glyphs_height + 40).to_rgba8();

        for glyph in glyphs {
            if let Some(bounding_box) = glyph.pixel_bounding_box() {
                // Draw the glyph into the image per-pixel by using the draw closure
                glyph.draw(|x, y, v| {
                    image.put_pixel(
                        // Offset the position by the glyph bounding box
                        x + bounding_box.min.x as u32,
                        y + bounding_box.min.y as u32,
                        // Turn the coverage into an alpha value
                        Rgba([font_colour.0, font_colour.1, font_colour.2, (v * 255.0) as u8]),
                    )
                });
            }
        }

        //Render textured quad with the above image
        self.font_program.bind();
        // self.font_program.uniform("scale", f32_f32::from( (glyphs_width as f32, glyphs_height as f32) ));
        self.font_program.uniform("scale", f32_f32::from( (16.0, 16.0) ));
        self.font_mesh.draw();
        self.font_program.unbind();
    }
}
