#[macro_use] extern crate log;
#[macro_use] extern crate lazy_static;

use std::collections::HashMap;

use rusttype::Font;

use mlua::prelude::*;
use mlua::{Table, UserData, UserDataMethods};

pub(crate) mod gl_wrapper;

pub mod husky2d;

pub fn load_gl(gl_context: &glutin::Context<glutin::PossiblyCurrent>) {
    gl::load_with(|ptr| gl_context.get_proc_address(ptr) as *const _);
}

#[derive(Clone)]
pub struct Renderer {
    pub fonts: HashMap<String, Font<'static>>,

    pub renderer2d: husky2d::Renderer2D,
}

impl Renderer {
    pub fn new(window_size: glutin::dpi::PhysicalSize<u32>) -> Self {
        unsafe {
            gl::Viewport(0, 0, window_size.width as _, window_size.height as _);
        }

        let roboto = Font::try_from_bytes(include_bytes!("../../fonts/RobotoMono-Regular.ttf") as &[u8]).expect("Failed to load font!");
        let mut fonts = HashMap::new();
        fonts.insert("roboto".to_string(), roboto);

        Self {
            fonts: fonts,

            renderer2d: husky2d::Renderer2D::new("roboto".to_string()),
        }
    }

    pub fn clear(&self, r: f32, g: f32, b: f32, a: f32) {
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
            gl::ClearColor(r,g,b,a);
        }
    }
}

impl UserData for Renderer {
    fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method("clear", |_, obj, (r,g,b,a): (f32, f32, f32, Option<f32>)| {
            obj.clear(r,g,b,a.unwrap_or(1.0));
            Ok(())
        });

        methods.add_method("print", |_, obj, (text, x,y): (String, f32,f32)| {
            let font = obj.fonts.get("roboto").unwrap();
            obj.renderer2d.gfx_print(&font, &text, x,y);
            Ok(())
        });
    }
}
