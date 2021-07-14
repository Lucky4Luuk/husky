use std::sync::Arc;

use crate::gl_wrapper::shader::{Shader as GlShader, ShaderProgram as GlShaderProgram};

use mlua::{UserData, UserDataMethods};

pub fn add_methods<'lua, M: UserDataMethods<'lua, crate::Renderer>>(methods: &mut M) {
    methods.add_method("newShader", |_, obj, (path_vs, path_gs, path_fs): (String, Option<String>, Option<String>)| {
        if path_gs.is_some() && path_fs.is_none() {
            //Vertex, fragment, none
            
        } else if path_gs.is_some() && path_fs.is_some() {
            //Vertex, geom, fragment
        } else if path_gs.is_none() && path_fs.is_some() {
            //Default vertex shader
        }
        Ok(())
    });
}

pub struct Shader {
    raw_program: Arc<GlShaderProgram>,
}

impl Shader {
    pub fn from_shaders(shaders: Vec<&GlShader>) -> Self {
        Self {
            raw_program: Arc::new(GlShaderProgram::from_shaders(shaders))
        }
    }
}

impl UserData for Shader {}
