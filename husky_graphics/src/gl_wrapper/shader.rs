use std::ffi::CString;

use super::util;
use super::gl_types::UniformValue;

pub struct Shader {
    pub id: gl::types::GLuint,
}

impl Shader {
    pub fn from_source(source: &str, kind: gl::types::GLuint) -> Result<Self, String> {
        let id = match shader_from_source(source, kind) {
            Ok(id) => id,
            Err(why) => {
                error!("Shader failed to compile: {}", why);
                return Err(why);
            },
        };
        Ok(Self {
            id: id
        })
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteShader(self.id);
        }
    }
}

pub struct ShaderProgram {
    pub id: gl::types::GLuint,
}

impl ShaderProgram {
    pub fn from_shaders(shaders: Vec<&Shader>) -> Self {
        let mut ids = Vec::new();
        for shader in shaders {
            ids.push(shader.id);
        }
        let id = program_from_ids(ids);
        Self {
            id: id
        }
    }

    pub fn from_shader(shader: &Shader) -> Self {
        let id = program_from_ids(vec![shader.id]);
        Self {
            id: id
        }
    }

    pub fn uniform(&self, name: &str, val: impl UniformValue) {
        let cname = CString::new(name).unwrap();
        val.update(self, &cname);
    }

    pub fn bind(&self) {
        unsafe {
            gl::UseProgram(self.id);
        }
    }

    pub fn unbind(&self) {
        unsafe {
            gl::UseProgram(0);
        }
    }
}

impl Drop for ShaderProgram {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteProgram(self.id);
        }
    }
}

fn program_from_ids(ids: Vec<gl::types::GLuint>) -> gl::types::GLuint {
    let id = unsafe { gl::CreateProgram() };

    for shader_id in &ids {
        unsafe { gl::AttachShader(id, *shader_id); }
    }

    unsafe { gl::LinkProgram(id); }

    for shader_id in &ids {
        unsafe { gl::DetachShader(id, *shader_id); }
    }

    id
}

fn shader_from_source(source: &str, kind: gl::types::GLuint) -> Result<gl::types::GLuint, String> {
    let id = unsafe { gl::CreateShader(kind) };

    let c_str = CString::new(source.as_bytes()).unwrap();

    unsafe {
        gl::ShaderSource(id, 1, &c_str.as_ptr(), std::ptr::null());
        gl::CompileShader(id);
    }

    let mut success: gl::types::GLint = 1;
    unsafe {
        gl::GetShaderiv(id, gl::COMPILE_STATUS, &mut success);
    }

    if success == 0 {
        let mut len: gl::types::GLint = 0;
        unsafe {
            gl::GetShaderiv(id, gl::INFO_LOG_LENGTH, &mut len);
        }

        let error = util::create_whitespace_cstring_with_len(len as usize);

        unsafe {
            gl::GetShaderInfoLog(
                id,
                len,
                std::ptr::null_mut(),
                error.as_ptr() as *mut gl::types::GLchar
            );
        }

        return Err(error.to_string_lossy().into_owned());
    }

    Ok(id)
}
