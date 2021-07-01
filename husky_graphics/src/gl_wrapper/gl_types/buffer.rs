use crate::gl_wrapper::mesh::Vertex;
use super::{f32_f32, f32_f32_f32, f32_f32_f32_f32};

pub trait BufferType {
    const BUFFER_TYPE: gl::types::GLuint;
}

pub struct BufferTypeArray;
impl BufferType for BufferTypeArray {
    const BUFFER_TYPE: gl::types::GLuint = gl::ARRAY_BUFFER;
}
pub type ArrayBuffer = Buffer<BufferTypeArray>;

pub struct BufferTypeElementArray;
impl BufferType for BufferTypeElementArray {
    const BUFFER_TYPE: gl::types::GLuint = gl::ELEMENT_ARRAY_BUFFER;
}
pub type ElementArrayBuffer = Buffer<BufferTypeElementArray>;

pub struct BufferTypeSSBO;
impl BufferType for BufferTypeSSBO {
    const BUFFER_TYPE: gl::types::GLuint = gl::SHADER_STORAGE_BUFFER;
}
pub type ShaderStorageBuffer = Buffer<BufferTypeSSBO>;

impl ShaderStorageBuffer {
    pub fn bind_buffer_base(&self, index: u32) {
        self.bind();
        unsafe {
            gl::BindBufferBase(gl::SHADER_STORAGE_BUFFER, index, self.vbo);
        }
        self.unbind();
    }
}

pub struct Buffer<B> where B: BufferType {
    vbo: gl::types::GLuint,
    _marker: std::marker::PhantomData<B>,
}

impl<B> Buffer<B> where B: BufferType {
    pub fn new() -> Buffer<B> {
        let mut vbo: gl::types::GLuint = 0;
        unsafe {
            gl::GenBuffers(1, &mut vbo);
        }

        Buffer {
            vbo: vbo,
            _marker: std::marker::PhantomData,
        }
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindBuffer(B::BUFFER_TYPE, self.vbo);
        }
    }

    pub fn unbind(&self) {
        unsafe {
            gl::BindBuffer(B::BUFFER_TYPE, 0);
        }
    }

    //TODO: Remove this function
    pub fn static_draw_data<T>(&self, data: &[T]) {
        unsafe {
            gl::BufferData(
                B::BUFFER_TYPE, // target
                (data.len() * ::std::mem::size_of::<T>()) as gl::types::GLsizeiptr, // size of data in bytes
                data.as_ptr() as *const gl::types::GLvoid, // pointer to data
                gl::STATIC_DRAW,
            );
        }
    }

    pub fn data<T>(&self, data: &[T], usage: gl::types::GLenum) {
        unsafe {
            gl::BufferData(
                B::BUFFER_TYPE, // target
                (data.len() * ::std::mem::size_of::<T>()) as gl::types::GLsizeiptr, // size of data in bytes
                data.as_ptr() as *const gl::types::GLvoid, // pointer to data
                usage,
            );
        }
    }

    pub fn sub_data<T>(&self, data: &[T], offset: isize) {
        unsafe {
            gl::BufferSubData(
                B::BUFFER_TYPE, // target
                offset,
                (data.len() * ::std::mem::size_of::<T>()) as gl::types::GLsizeiptr, // size of data in bytes
                data.as_ptr() as *const gl::types::GLvoid, // pointer to data
            );
        }
    }
}

impl<B> Drop for Buffer<B> where B: BufferType {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteBuffers(1, &mut self.vbo);
        }
    }
}

//TODO: Generic over Vertex?
pub struct VertexArray {
    vao: gl::types::GLuint,
}

impl Drop for VertexArray {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteVertexArrays(1, &mut self.vao);
        }
    }
}

impl VertexArray {
    pub fn new() -> Self {
        let mut vao: gl::types::GLuint = 0;
        unsafe {
            gl::GenVertexArrays(1, &mut vao);
        }
        Self {
            vao: vao,
        }
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindVertexArray(self.vao);
        }
    }

    pub fn unbind(&self) {
        unsafe {
            gl::BindVertexArray(0);
        }
    }

    pub fn attrib_pointers(&self) {
        let stride = std::mem::size_of::<Vertex>();

        let mut location = 0;
        let mut offset = 0;

        unsafe {
            //XYZ
            f32_f32_f32::vertex_attrib_pointer(stride, location, offset);
            location += 1;
            offset += std::mem::size_of::<f32_f32_f32>();

            //UV
            f32_f32::vertex_attrib_pointer(stride, location, offset);
            location += 1;
            offset += std::mem::size_of::<f32_f32>();

            //RGBA
            f32_f32_f32_f32::vertex_attrib_pointer(stride, location, offset);
            // location += 1;
            // offset += std::mem::size_of::<f32_f32_f32_f32>();
        }
    }
}
