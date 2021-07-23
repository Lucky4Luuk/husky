use std::ffi::CString;

use super::UniformValue;
use crate::shader::ShaderProgram;

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct f32_f32 {
    pub d0: f32,
    pub d1: f32,
}

impl f32_f32 {
    pub fn new(d0: f32, d1: f32) -> f32_f32 {
        f32_f32 {
            d0, d1
        }
    }

    pub unsafe fn vertex_attrib_pointer(stride: usize, location: usize, offset: usize) {
        gl::EnableVertexAttribArray(location as gl::types::GLuint);
        gl::VertexAttribPointer(
            location as gl::types::GLuint,
            2, // the number of components per generic vertex attribute
            gl::FLOAT, // data type
            gl::FALSE, // normalized (int-to-float conversion)
            stride as gl::types::GLint,
            offset as *const gl::types::GLvoid
        );
    }
}

impl From<(f32, f32)> for f32_f32 {
    fn from(other: (f32, f32)) -> Self {
        f32_f32::new(other.0, other.1)
    }
}

impl UniformValue for f32_f32 {
    fn update(&self, shader: &ShaderProgram, name: &CString) {
        let loc = unsafe { gl::GetUniformLocation(shader.id, name.as_ptr()) };
        unsafe { gl::Uniform2f(loc, self.d0, self.d1); }
    }
}

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct f32_f32_f32 {
    pub d0: f32,
    pub d1: f32,
    pub d2: f32,
}

impl f32_f32_f32 {
    pub fn new(d0: f32, d1: f32, d2: f32) -> f32_f32_f32 {
        f32_f32_f32 {
            d0, d1, d2
        }
    }

    pub unsafe fn vertex_attrib_pointer(stride: usize, location: usize, offset: usize) {
        gl::EnableVertexAttribArray(location as gl::types::GLuint);
        gl::VertexAttribPointer(
            location as gl::types::GLuint,
            3, // the number of components per generic vertex attribute
            gl::FLOAT, // data type
            gl::FALSE, // normalized (int-to-float conversion)
            stride as gl::types::GLint,
            offset as *const gl::types::GLvoid
        );
    }
}

impl From<(f32, f32, f32)> for f32_f32_f32 {
    fn from(other: (f32, f32, f32)) -> Self {
        f32_f32_f32::new(other.0, other.1, other.2)
    }
}

impl UniformValue for f32_f32_f32 {
    fn update(&self, shader: &ShaderProgram, name: &CString) {
        let loc = unsafe { gl::GetUniformLocation(shader.id, name.as_ptr()) };
        unsafe { gl::Uniform3f(loc, self.d0, self.d1, self.d2); }
    }
}

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct f32_f32_f32_f32 {
    pub d0: f32,
    pub d1: f32,
    pub d2: f32,
    pub d3: f32,
}

impl f32_f32_f32_f32 {
    pub fn new(d0: f32, d1: f32, d2: f32, d3: f32) -> f32_f32_f32_f32 {
        f32_f32_f32_f32 {
            d0, d1, d2, d3
        }
    }

    pub unsafe fn vertex_attrib_pointer(stride: usize, location: usize, offset: usize) {
        gl::EnableVertexAttribArray(location as gl::types::GLuint);
        gl::VertexAttribPointer(
            location as gl::types::GLuint,
            4, // the number of components per generic vertex attribute
            gl::FLOAT, // data type
            gl::FALSE, // normalized (int-to-float conversion)
            stride as gl::types::GLint,
            offset as *const gl::types::GLvoid
        );
    }
}

impl From<(f32, f32, f32, f32)> for f32_f32_f32_f32 {
    fn from(other: (f32, f32, f32, f32)) -> Self {
        f32_f32_f32_f32::new(other.0, other.1, other.2, other.3)
    }
}

impl UniformValue for f32_f32_f32_f32 {
    fn update(&self, shader: &ShaderProgram, name: &CString) {
        let loc = unsafe { gl::GetUniformLocation(shader.id, name.as_ptr()) };
        unsafe { gl::Uniform4f(loc, self.d0, self.d1, self.d2, self.d3); }
    }
}

impl UniformValue for glam::Mat4 {
    fn update(&self, shader: &ShaderProgram, name: &CString) {
        let loc = unsafe { gl::GetUniformLocation(shader.id, name.as_ptr()) };
        unsafe { gl::UniformMatrix4fv(loc, 1, gl::FALSE, self.to_cols_array().as_ptr()); }
    }
}
