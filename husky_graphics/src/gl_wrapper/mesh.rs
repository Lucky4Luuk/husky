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
    index_count: i32,
    // vao: GLuint,
    // vbo: GLuint,
    // ebo: GLuint,
    vao: VertexArray,
    vbo: ArrayBuffer,
    ebo: ElementArrayBuffer,
}

impl Mesh {
    /// Vertex format:
    /// x,y,z   - f32, f32, f32
    /// u,v     - f32, f32
    /// r,g,b,a - f32, f32, f32, f32
    pub fn from_vertices(vertices: &Vec<Vertex>, indices: &Vec<u32>) -> Self {
        trace!("New mesh!");
        let vcount = vertices.len() as i32;
        let index_count = indices.len() as i32;

        let vao = VertexArray::new();
        let vbo = ArrayBuffer::new();
        let ebo = ElementArrayBuffer::new();

        vao.bind();

        vbo.bind();
        vbo.data(vertices, gl::STATIC_DRAW);

        ebo.bind();
        ebo.data(vertices, gl::STATIC_DRAW);

        vao.attrib_pointers();

        vao.unbind();

        Self {
            vert_count: vcount,
            index_count: index_count,
            vao: vao,
            ebo: ebo,
            vbo: vbo,
        }
    }

    /// Make sure to bind a shader first!
    pub fn draw(&self) {
        unsafe {
            self.vao.bind();
            gl::DrawElements(
                gl::TRIANGLES, // mode
                self.index_count, // number of indices to be rendered,
                gl::UNSIGNED_INT,
                0 as *const GLvoid,
            );
            gl_assert_ok!();
            self.vao.unbind();
        }
    }
}
