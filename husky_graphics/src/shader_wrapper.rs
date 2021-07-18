use std::path::Path;
use std::sync::Arc;

use crate::gl_wrapper::shader::{Shader as GlShader, ShaderProgram as GlShaderProgram};

use mlua::{UserData, UserDataMethods, Error, Lua, ToLua, FromLua, AnyUserData, Value, Result as LuaResult};

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
}

impl Shader {
    pub fn from_shaders(shaders: Vec<&GlShader>) -> Self {
        Self {
            raw_program: Arc::new(GlShaderProgram::from_shaders(shaders))
        }
    }
}

impl UserData for Shader {}
