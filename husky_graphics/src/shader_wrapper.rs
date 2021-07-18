use std::collections::HashMap;
use std::path::Path;
use std::sync::Arc;

use crate::gl_wrapper::gl_types::UniformValue;
use crate::gl_wrapper::shader::{Shader as GlShader, ShaderProgram as GlShaderProgram};
use gl::types::*;

use mlua::prelude::{LuaResult, LuaValue, LuaError};
use mlua::{UserData, UserDataMethods, Error};

pub fn add_methods<'lua, M: UserDataMethods<'lua, crate::RendererGuard>>(methods: &mut M) {
    methods.add_method("newShader", |_, obj, (path_vs, path_gs, path_fs): (String, Option<String>, Option<String>)| {
        let wd_str = {
            let renderer = obj.get_lock();
            renderer.working_directory.clone()
        };
        let wd = Path::new(&wd_str);
        if path_gs.is_some() && path_fs.is_none() {
            //Vertex, fragment, none
            let vs_src = std::fs::read_to_string(wd.join(path_vs))?;
            let shader_vs = GlShader::from_source(&vs_src, gl::VERTEX_SHADER).map_err(|_| Error::RuntimeError("Failed to compile vertex shader!".to_string()) )?;
            let fs_src = std::fs::read_to_string(wd.join(path_gs.unwrap()))?;
            let shader_fs = GlShader::from_source(&fs_src, gl::FRAGMENT_SHADER).map_err(|_| Error::RuntimeError("Failed to compile fragment shader!".to_string()) )?;
            return Ok(Shader::from_shaders(vec![&shader_vs, &shader_fs]));
        } else if path_gs.is_some() && path_fs.is_some() {
            //Vertex, geom, fragment
            let vs_src = std::fs::read_to_string(wd.join(path_vs))?;
            let shader_vs = GlShader::from_source(&vs_src, gl::VERTEX_SHADER).map_err(|_| Error::RuntimeError("Failed to compile vertex shader!".to_string()) )?;
            let gs_src = std::fs::read_to_string(wd.join(path_gs.unwrap()))?;
            let shader_gs = GlShader::from_source(&gs_src, gl::GEOMETRY_SHADER).map_err(|_| Error::RuntimeError("Failed to compile fragment shader!".to_string()) )?;
            let fs_src = std::fs::read_to_string(wd.join(path_fs.unwrap()))?;
            let shader_fs = GlShader::from_source(&fs_src, gl::FRAGMENT_SHADER).map_err(|_| Error::RuntimeError("Failed to compile fragment shader!".to_string()) )?;
            return Ok(Shader::from_shaders(vec![&shader_vs, &shader_gs, &shader_fs]));
        } else if path_gs.is_none() && path_fs.is_none() {
            //Default vertex shader
            let vs_src = include_str!("../../shaders/default_vs.glsl");
            let shader_vs = GlShader::from_source(&vs_src, gl::VERTEX_SHADER).map_err(|_| Error::RuntimeError("Failed to compile vertex shader!".to_string()) )?;
            let fs_src = std::fs::read_to_string(wd.join(path_vs))?;
            let shader_fs = GlShader::from_source(&fs_src, gl::FRAGMENT_SHADER).map_err(|_| Error::RuntimeError("Failed to compile fragment shader!".to_string()) )?;
            return Ok(Shader::from_shaders(vec![&shader_vs, &shader_fs]));
        }
        Err(Error::RuntimeError("Shader compilation failed! How did you trigger this?".to_string()))
    });

    methods.add_method("setShader", |_, obj, shader: Option<Shader>| {
        let mut renderer = obj.get_lock();
        renderer.set_active_shader(shader);
        Ok(())
    });
}

#[derive(Clone)]
pub struct Shader {
    pub raw_program: Arc<GlShaderProgram>,
    uniform_hashmap: HashMap<String, GLenum>,
}

impl Shader {
    pub fn from_shaders(shaders: Vec<&GlShader>) -> Self {
        use std::ffi::CString;

        let program = GlShaderProgram::from_shaders(shaders);
        let mut map = HashMap::new();
        let mut count = 0;
        unsafe { gl::GetProgramiv(program.id, gl::ACTIVE_UNIFORMS, &mut count); }
        for i in 0..count {
            unsafe {
                const BUFSIZE: i32 = 64;
                let mut length = 0;
                let mut size = 0;
                let mut ty = 0;
                let name: [i8; BUFSIZE as usize] = [0; BUFSIZE as usize];
                gl::GetActiveUniform(program.id, i as u32, BUFSIZE, &mut length, &mut size, &mut ty, name.as_ptr() as *mut i8);
                let name_u8: Vec<u8> = name.iter().map(|s| *s as u8).filter(|s| *s != 0).collect();
                let name_str = CString::new(name_u8).expect("Failed to create cstring!").into_string().expect("Failed to create string!");
                map.insert(name_str.trim().to_string(), ty);
            }
        }
        Self {
            raw_program: Arc::new(program),
            uniform_hashmap: map,
        }
    }

    fn get_uniform_type(&self, name: &str) -> LuaResult<&GLenum> {
        self.uniform_hashmap.get(name).ok_or(LuaError::RuntimeError("Uniform does not exist!".to_string()))
    }

    fn uniform_bool(&self, name: &str, value: bool) -> LuaResult<()> {
        let ty = self.get_uniform_type(name)?;
        match *ty {
            gl::INT => Ok(self.raw_program.uniform(&name, value)),
            gl::FLOAT => Ok(self.raw_program.uniform(&name, value as i32 as f32)),
            _ => Err(LuaError::RuntimeError("Unknown uniform type!".to_string())),
        }
    }

    fn uniform_int(&self, name: &str, value: i32) -> LuaResult<()> {
        let ty = self.get_uniform_type(name)?;
        match *ty {
            gl::INT => Ok(self.raw_program.uniform(&name, value)),
            gl::FLOAT => Ok(self.raw_program.uniform(&name, value as f32)),
            _ => Err(LuaError::RuntimeError("Unknown uniform type!".to_string())),
        }
    }

    fn uniform_float(&self, name: &str, value: f32) -> LuaResult<()> {
        let ty = self.get_uniform_type(name)?;
        match *ty {
            gl::INT => Ok(self.raw_program.uniform(&name, value as i32)),
            gl::FLOAT => Ok(self.raw_program.uniform(&name, value)),
            _ => Err(LuaError::RuntimeError("Unknown uniform type!".to_string())),
        }
    }

    pub fn uniform(&self, name: String, value: LuaValue) -> LuaResult<()> {
        match value {
            LuaValue::Boolean(v) => self.uniform_bool(&name, v),
            LuaValue::Integer(v) => self.uniform_int(&name, v as i32),
            LuaValue::Number(v) => self.uniform_float(&name, v as f32),
            LuaValue::Table(v) => todo!(),
            LuaValue::UserData(v) => todo!(),
            _ => panic!("Unsupported uniform type!")
        }
    }
}

impl UserData for Shader {
    fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method("uniform", |_, obj, (name, value): (String, LuaValue)| {
            obj.uniform(name, value)?;
            Ok(())
        });
    }
}

#[derive(Clone)]
pub struct Uniform {
    pub raw: Arc<Box<dyn UniformValue>>,
}

impl UserData for Uniform {}
