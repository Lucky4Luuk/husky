use std::ffi::CString;

mod primitives;
pub use primitives::*;

mod buffer;
pub use buffer::*;

mod texture;
pub use texture::*;

mod framebuffer;
pub use framebuffer::Framebuffer;

use super::shader::ShaderProgram;

pub trait UniformValue {
    fn update(&self, shader: &ShaderProgram, name: &CString);
}

//Implementations for basic rust types
impl UniformValue for f32 {
    fn update(&self, shader: &ShaderProgram, name: &CString) {
        let loc = unsafe { gl::GetUniformLocation(shader.id, name.as_ptr()) };
        unsafe { gl::Uniform1f(loc, *self); }
    }
}

impl UniformValue for i32 {
    fn update(&self, shader: &ShaderProgram, name: &CString) {
        let loc = unsafe { gl::GetUniformLocation(shader.id, name.as_ptr()) };
        unsafe { gl::Uniform1i(loc, *self); }
    }
}

impl UniformValue for u32 {
    fn update(&self, shader: &ShaderProgram, name: &CString) {
        let loc = unsafe { gl::GetUniformLocation(shader.id, name.as_ptr()) };
        unsafe { gl::Uniform1ui(loc, *self); }
    }
}

impl UniformValue for bool {
    fn update(&self, shader: &ShaderProgram, name: &CString) {
        let loc = unsafe { gl::GetUniformLocation(shader.id, name.as_ptr()) };
        unsafe { gl::Uniform1i(loc, *self as i32); }
    }
}
