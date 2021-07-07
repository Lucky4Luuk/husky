use super::gl_types::{
    f32_f32, f32_f32_f32, f32_f32_f32_f32,
    ArrayBuffer, ElementArrayBuffer, VertexArray,
};

use gl::types::*;

pub fn gl_err_to_str(err: u32) -> &'static str {
    match err {
        gl::INVALID_ENUM => "INVALID_ENUM",
        gl::INVALID_VALUE => "INVALID_VALUE",
        gl::INVALID_OPERATION => "INVALID_OPERATION",
        gl::INVALID_FRAMEBUFFER_OPERATION => "INVALID_FRAMEBUFFER_OPERATION",
        gl::OUT_OF_MEMORY => "OUT_OF_MEMORY",
        gl::STACK_UNDERFLOW => "STACK_UNDERFLOW",
        gl::STACK_OVERFLOW => "STACK_OVERFLOW",
        _ => "Unknown error",
    }
}

macro_rules! gl_assert_ok {
    () => {{
        let err = gl::GetError();
        assert_eq!(err, gl::NO_ERROR, "{}", gl_err_to_str(err));
    }};
}

#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct Vertex {
    pub pos: f32_f32_f32,
    pub uv: f32_f32,
    pub rgba: f32_f32_f32_f32,
}

#[derive(Clone)]
pub struct Mesh {
    vert_count: i32,
    _vbo: ArrayBuffer,
    vao: VertexArray,
}

impl Mesh {
    /// Vertex format:
    /// x,y,z   - f32, f32, f32
    /// u,v     - f32, f32
    /// r,g,b,a - f32, f32, f32, f32
    pub fn from_vertices(vertices: &Vec<Vertex>) -> Self {
        let vcount = vertices.len() as i32;
        let vbo = ArrayBuffer::new();
        vbo.bind();
        vbo.static_draw_data(&vertices);
        vbo.unbind();

        let vao = VertexArray::new();
        vao.bind();
        vbo.bind();
        vao.attrib_pointers();
        vbo.unbind();
        vao.unbind();

        Self {
            vert_count: vcount,
            _vbo: vbo,
            vao: vao,
        }
    }

    /// Make sure to bind a shader first!
    pub fn draw(&self) {
        unsafe {
            self.vao.bind();
            gl::DrawArrays(
                gl::TRIANGLES, // mode
                0, // starting index in the enabled arrays
                self.vert_count // number of indices to be rendered
            );
            self.vao.unbind();
        }
    }

    ///Make sure to bind a shader first!
    pub fn draw_wireframe(&self) {
        unsafe {
            self.vao.bind();
            gl::DrawArrays(
                gl::LINES, // mode
                0, // starting index in the enabled arrays
                self.vert_count
            );
            self.vao.unbind();
        }
    }
}
