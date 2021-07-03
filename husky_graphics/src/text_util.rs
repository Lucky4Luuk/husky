use std::mem;
use std::ffi::CString;

use gl::types::*;

use glyph_brush::{ab_glyph::*, *};

use crate::gl_wrapper::gl_types::Texture;
use crate::gl_wrapper::shader::{Shader, ShaderProgram};

pub type Vertex = [f32; 13];

#[inline]
pub fn to_vertex(
    glyph_brush::GlyphVertex {
        mut tex_coords,
        pixel_coords,
        bounds,
        extra,
    }: glyph_brush::GlyphVertex,
) -> Vertex {
    let gl_bounds = bounds;

    let mut gl_rect = Rect {
        min: point(pixel_coords.min.x as f32, pixel_coords.min.y as f32),
        max: point(pixel_coords.max.x as f32, pixel_coords.max.y as f32),
    };

    // handle overlapping bounds, modify uv_rect to preserve texture aspect
    if gl_rect.max.x > gl_bounds.max.x {
        let old_width = gl_rect.width();
        gl_rect.max.x = gl_bounds.max.x;
        tex_coords.max.x = tex_coords.min.x + tex_coords.width() * gl_rect.width() / old_width;
    }
    if gl_rect.min.x < gl_bounds.min.x {
        let old_width = gl_rect.width();
        gl_rect.min.x = gl_bounds.min.x;
        tex_coords.min.x = tex_coords.max.x - tex_coords.width() * gl_rect.width() / old_width;
    }
    if gl_rect.max.y > gl_bounds.max.y {
        let old_height = gl_rect.height();
        gl_rect.max.y = gl_bounds.max.y;
        tex_coords.max.y = tex_coords.min.y + tex_coords.height() * gl_rect.height() / old_height;
    }
    if gl_rect.min.y < gl_bounds.min.y {
        let old_height = gl_rect.height();
        gl_rect.min.y = gl_bounds.min.y;
        tex_coords.min.y = tex_coords.max.y - tex_coords.height() * gl_rect.height() / old_height;
    }

    [
        gl_rect.min.x,
        gl_rect.max.y,
        extra.z,
        gl_rect.max.x,
        gl_rect.min.y,
        tex_coords.min.x,
        tex_coords.max.y,
        tex_coords.max.x,
        tex_coords.min.y,
        extra.color[0],
        extra.color[1],
        extra.color[2],
        extra.color[3],
    ]
}

#[rustfmt::skip]
pub fn ortho(left: f32, right: f32, bottom: f32, top: f32, near: f32, far: f32) -> [f32; 16] {
    let tx = -(right + left) / (right - left);
    let ty = -(top + bottom) / (top - bottom);
    let tz = -(far + near) / (far - near);
    [
        2.0 / (right - left), 0.0, 0.0, 0.0,
        0.0, 2.0 / (top - bottom), 0.0, 0.0,
        0.0, 0.0, -2.0 / (far - near), 0.0,
        tx, ty, tz, 1.0,
    ]
}

#[derive(Clone)]
pub struct GlGlyphTexture {
    pub texture: Texture,
}

impl GlGlyphTexture {
    pub fn new((width, height): (u32, u32)) -> Self {
        unsafe {
            gl::PixelStorei(gl::UNPACK_ALIGNMENT, 1);
        }
        let texture = Texture::from_ptr((width as i32, height as i32), std::ptr::null(), gl::RED as i32, gl::RED, gl::UNSIGNED_BYTE);
        unsafe {
            texture.bind();
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as _);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as _);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as _);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as _);
            texture.unbind();
        }
        Self {
            texture: texture
        }
    }

    pub fn clear(&self) {
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, self.texture.id);
            gl::ClearTexImage(
                self.texture.id,
                0,
                gl::RED,
                gl::UNSIGNED_BYTE,
                [12_u8].as_ptr() as _,
            );
        }
    }
}

pub struct GlTextPipe {
    program: ShaderProgram,
    vao: GLuint,
    vbo: GLuint,
    transform_uniform: GLint,
    vertex_count: usize,
    vertex_buffer_len: usize,
}

impl GlTextPipe {
    pub fn new(window_size: glutin::dpi::PhysicalSize<u32>) -> Self {
        let (w, h) = (window_size.width as f32, window_size.height as f32);

        let vs = Shader::from_source(include_str!("../../shaders/vs_text.glsl"), gl::VERTEX_SHADER).expect("Failed to compile font shader!");
        let fs = Shader::from_source(include_str!("../../shaders/fs_text.glsl"), gl::FRAGMENT_SHADER).expect("Failed to compile font shader!");
        let program = ShaderProgram::from_shaders(vec![&vs, &fs]);

        let mut vao = 0;
        let mut vbo = 0;

        let transform_uniform;

        unsafe {
            // Create Vertex Array Object
            gl::GenVertexArrays(1, &mut vao);
            gl::BindVertexArray(vao);

            // Create a Vertex Buffer Object
            gl::GenBuffers(1, &mut vbo);
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);

            // Specify the layout of the vertex data
            transform_uniform = gl::GetUniformLocation(program.id, CString::new("transform").expect("Failed to create cstring!").as_ptr());
            if transform_uniform < 0 {
                panic!("{}", format!("GetUniformLocation(\"transform\") -> {}", transform_uniform));
            }
            let transform = ortho(0.0, w, 0.0, h, 1.0, -1.0);
            gl::UniformMatrix4fv(transform_uniform, 1, 0, transform.as_ptr());

            let mut offset = 0;
            for (v_field, float_count) in &[
                ("left_top", 3),
                ("right_bottom", 2),
                ("tex_left_top", 2),
                ("tex_right_bottom", 2),
                ("color", 4),
            ] {
                let attr = gl::GetAttribLocation(program.id, CString::new(*v_field).expect("Failed to create cstring!").as_ptr());
                if attr < 0 {
                    panic!("{}", format!("{} GetAttribLocation -> {}", v_field, attr));
                }
                gl::VertexAttribPointer(
                    attr as _,
                    *float_count,
                    gl::FLOAT,
                    gl::FALSE as _,
                    mem::size_of::<Vertex>() as _,
                    offset as _,
                );
                gl::EnableVertexAttribArray(attr as _);
                gl::VertexAttribDivisor(attr as _, 1);

                offset += float_count * 4;
            }

            // Enabled alpha blending
            gl::Enable(gl::BLEND);
            gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
            // Use srgb for consistency with other examples
            gl::Enable(gl::FRAMEBUFFER_SRGB);
            gl::ClearColor(0.02, 0.02, 0.02, 1.0);
        };

        Self {
            program: program,
            vao: vao,
            vbo: vbo,
            transform_uniform: transform_uniform,
            vertex_count: 0,
            vertex_buffer_len: 0,
        }
    }

    pub fn upload_vertices(&mut self, vertices: &[Vertex]) {
        // Draw new vertices
        self.vertex_count = vertices.len();

        unsafe {
            gl::BindVertexArray(self.vao);
            gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo);
            if self.vertex_buffer_len < self.vertex_count {
                gl::BufferData(
                    gl::ARRAY_BUFFER,
                    (self.vertex_count * mem::size_of::<Vertex>()) as GLsizeiptr,
                    vertices.as_ptr() as _,
                    gl::DYNAMIC_DRAW,
                );
                self.vertex_buffer_len = self.vertex_count;
            } else {
                gl::BufferSubData(
                    gl::ARRAY_BUFFER,
                    0,
                    (self.vertex_count * mem::size_of::<Vertex>()) as GLsizeiptr,
                    vertices.as_ptr() as _,
                );
            }
        }
    }

    pub fn update_geometry(&self, window_size: glutin::dpi::PhysicalSize<u32>) {
        let (w, h) = (window_size.width as f32, window_size.height as f32);
        let transform = ortho(0.0, w, 0.0, h, 1.0, -1.0);

        unsafe {
            gl::UseProgram(self.program.id);
            gl::UniformMatrix4fv(self.transform_uniform, 1, 0, transform.as_ptr());
        }
    }

    pub fn draw(&self) {
        unsafe {
            gl::UseProgram(self.program.id);
            gl::BindVertexArray(self.vao);
            gl::DrawArraysInstanced(gl::TRIANGLE_STRIP, 0, 4, self.vertex_count as _);
        }
    }
}

impl Drop for GlTextPipe {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteBuffers(1, &self.vbo);
            gl::DeleteVertexArrays(1, &self.vao);
        }
    }
}
