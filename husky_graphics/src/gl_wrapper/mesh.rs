use super::gl_types::{
    f32_f32, f32_f32_f32, f32_f32_f32_f32,
    ArrayBuffer, ElementArrayBuffer, VertexArray,
};

#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct Vertex {
    pub pos: f32_f32_f32,
    pub uv: f32_f32,
    pub rgba: f32_f32_f32_f32,
}

pub struct Mesh {
    vert_count: i32,
    vbo: ArrayBuffer,
    ebo: ElementArrayBuffer,
    vao: VertexArray,
}

impl Mesh {
    /// Vertex format:
    /// x,y,z   - f32, f32, f32
    /// u,v     - f32, f32
    /// r,g,b,a - f32, f32, f32, f32
    pub fn from_vertices(vertices: &Vec<Vertex>, indices: &Vec<u32>) -> Self {
        let vcount = vertices.len() as i32;
        let vbo = ArrayBuffer::new();
        vbo.bind();
        vbo.static_draw_data(vertices);
        vbo.unbind();

        let ebo = ElementArrayBuffer::new();
        ebo.bind();
        ebo.static_draw_data(indices);
        ebo.unbind();

        let vao = VertexArray::new();
        vao.bind();
        vbo.bind();
        vao.attrib_pointers();
        vbo.unbind();
        vao.unbind();

        Self {
            vert_count: vcount,
            ebo: ebo,
            vbo: vbo,
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
}
