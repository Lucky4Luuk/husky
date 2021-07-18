#[macro_use] extern crate log;
#[macro_use] extern crate lazy_static;

use std::sync::MutexGuard;
use std::sync::atomic::{Ordering, AtomicBool};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use rusttype::Font;

use mlua::prelude::*;
use mlua::{Table, UserData, UserDataMethods};

pub(crate) mod gl_wrapper;
use gl_wrapper::shader::Shader as GlShader;

pub mod husky2d;

mod shader_wrapper;
pub use shader_wrapper::Shader;

pub fn load_gl(gl_context: &glutin::Context<glutin::PossiblyCurrent>) {
    gl::load_with(|ptr| gl_context.get_proc_address(ptr) as *const _);
}

lazy_static! {
    pub static ref WINDOW_SIZE: Mutex<(u32, u32)> = Mutex::new((1,1));
}

pub struct Renderer {
    pub fonts: HashMap<String, Font<'static>>,
    pub working_directory: String,

    pub renderer2d: husky2d::Renderer2D,

    pub active_color: (f32, f32, f32, f32),

    ///Lets us know if the shader is actually already bound.
    //This is probably redundant to store, but I usually code at 3 am
    //so I can't think if it is or not.
    is_default_shader_bound: bool,
    ///Lets us know if the shader is actually already bound.
    is_shader_bound: bool,
    pub default_shader: Shader,
    pub active_shader: Option<Shader>,
}

impl Renderer {
    pub fn new(working_directory: String) -> Self {
        let roboto = Font::try_from_bytes(include_bytes!("../../fonts/RobotoMono-Regular.ttf") as &[u8]).expect("Failed to load font!");
        let mut fonts = HashMap::new();
        fonts.insert("roboto".to_string(), roboto);

        let default_shader_vs = GlShader::from_source(include_str!("../../shaders/default_vs.glsl"), gl::VERTEX_SHADER).expect("Failed to compile default vs shader!");
        let default_shader_fs = GlShader::from_source(include_str!("../../shaders/default_fs.glsl"), gl::FRAGMENT_SHADER).expect("Failed to compile default fs shader!");
        let default_shader = Shader::from_shaders(vec![&default_shader_vs, &default_shader_fs]);

        Self {
            fonts: fonts,
            working_directory: working_directory,

            renderer2d: husky2d::Renderer2D::new("roboto".to_string()),

            active_color: (1.0, 1.0, 1.0, 1.0),

            is_default_shader_bound: false,
            is_shader_bound: false,
            default_shader: default_shader,
            active_shader: None,
        }
    }

    fn set_active_shader(&mut self, shader_opt: Option<Shader>) {
        match shader_opt {
            Some(shader) => {
                shader.raw_program.bind();
                self.active_shader = Some(shader);
                self.is_shader_bound = true;
                self.is_default_shader_bound = false;
            },
            None => {
                self.default_shader.raw_program.bind();
                self.active_shader = None;
                self.is_shader_bound = false;
                self.is_default_shader_bound = true;
            }
        }
    }

    fn get_active_shader(&mut self) -> &Shader {
        if self.is_default_shader_bound { return &self.default_shader; }
        if self.is_shader_bound { return self.active_shader.as_ref().unwrap(); }

        //No shader is already bound, so we have to figure out which one to bind
        match &self.active_shader {
            Some(shader) => {
                self.is_shader_bound = true;
                shader.raw_program.bind();
                shader
            },
            None => {
                self.default_shader.raw_program.bind();
                self.is_default_shader_bound = true;
                &self.default_shader
            }
        }
    }

    pub fn clear(&self, r: f32, g: f32, b: f32, a: f32) {
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
            gl::ClearColor(r,g,b,a);
        }
    }

    pub fn set_color(&mut self, r: f32, g: f32, b: f32, a: f32) {
        self.active_color = (r,g,b,a);
    }

    pub fn begin_frame(&self) {
        unsafe {
            let win_size = WINDOW_SIZE.lock().unwrap();
            gl::Viewport(0,0, win_size.0 as i32, win_size.1 as i32);
        }
    }

    pub fn finish_frame(&mut self) {
        //TODO: Start using your own setShader functions, to avoid code repetition
        if self.is_default_shader_bound {
            self.default_shader.raw_program.unbind();
            self.is_default_shader_bound = false;
        }
        if self.is_shader_bound {
            match &self.active_shader {
                Some(shader) => {
                    shader.raw_program.unbind();
                    self.active_shader = None;
                    self.is_shader_bound = false;
                },
                None => panic!("Shader bound but not in memory!")
            }
        }

        self.renderer2d.finish_frame();
    }
}

#[derive(Clone)]
pub struct RendererGuard {
    renderer: Arc<Mutex<Renderer>>
}

impl RendererGuard {
    pub fn new(working_directory: String) -> Self {
        Self {
            renderer: Arc::new(Mutex::new(Renderer::new(working_directory)))
        }
    }

    pub fn get_lock(&self) -> MutexGuard<Renderer> {
        self.renderer.lock().expect("Failed to acquire lock on renderer!")
    }
}

impl UserData for RendererGuard {
    fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method("begin_frame", |_, obj, ()| {
            obj.get_lock().begin_frame();
            Ok(())
        });
        methods.add_method("finish_frame", |_, obj, ()| {
            obj.get_lock().finish_frame();
            Ok(())
        });

        methods.add_method("clear", |_, obj, (r,g,b,a): (f32, f32, f32, Option<f32>)| {
            obj.get_lock().clear(r,g,b,a.unwrap_or(1.0));
            Ok(())
        });

        methods.add_method("setColor", |_, obj, (r,g,b,a): (f32, f32, f32, Option<f32>)| {
            obj.get_lock().set_color(r,g,b,a.unwrap_or(1.0));
            Ok(())
        });

        methods.add_method("getSize", |_, _obj, ()| {
            let win_size = *WINDOW_SIZE.lock().unwrap();
            Ok(win_size)
        });

        methods.add_method("print", |_, obj, (text, x,y): (String, f32,f32)| {
            let mut renderer = obj.get_lock();
            let font = renderer.fonts.get("roboto").unwrap().clone();
            let color = renderer.active_color;
            renderer.renderer2d.gfx_print(color, &font, &text, x,y);
            Ok(())
        });

        shader_wrapper::add_methods(methods);
        husky2d::add_methods(methods);
    }
}
