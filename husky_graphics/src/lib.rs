#[macro_use] extern crate log;
#[macro_use] extern crate lazy_static;

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use mlua::prelude::*;
use mlua::{Table, UserData, UserDataMethods};

pub(crate) mod text_util;
pub(crate) mod gl_wrapper;

pub mod husky2d;

pub fn load_gl(gl_context: &glutin::Context<glutin::PossiblyCurrent>) {
    gl::load_with(|ptr| gl_context.get_proc_address(ptr) as *const _);
}

#[derive(Clone)]
pub struct Renderer {
    pub fonts: HashMap<String, Arc< Mutex<text_util::FontObject> >>,

    pub renderer2d: husky2d::Renderer2D,
}

impl Renderer {
    pub fn new(window_size: glutin::dpi::PhysicalSize<u32>) -> Self {
        let roboto = glyph_brush::ab_glyph::FontArc::try_from_slice(include_bytes!("../../fonts/RobotoMono-Regular.ttf")).expect("Failed to load font!");
        let roboto_fontarc = Arc::new( Mutex::new(text_util::FontObject::from_fontarc(roboto) ));
        let mut fonts = HashMap::new();
        fonts.insert("roboto".to_string(), roboto_fontarc);

        Self {
            fonts: fonts,

            renderer2d: husky2d::Renderer2D::new(window_size, "roboto".to_string()),
        }
    }

    pub fn clear(&self, r: f32, g: f32, b: f32, a: f32) {
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
            gl::ClearColor(r,g,b,a);
        }
    }
}

fn get_gfx_table(lua: &Lua) -> Renderer {
    lua.globals().get::<&str, Table>("husky").expect("Failed to get husky table!").get::<&str, Renderer>("graphics").expect("Failed to get graphics table!")
}

impl UserData for Renderer {
    fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_function("clear", |lua, (r,g,b,a): (f32, f32, f32, Option<f32>)| {
            get_gfx_table(lua).clear(r,g,b,a.unwrap_or(1.0));
            Ok(())
        });

        methods.add_function("print", |lua, (text, x,y): (String, f32,f32)| {
            let mut gfx = get_gfx_table(lua);
            let mut font = gfx.fonts.get("roboto").unwrap().lock().unwrap();
            gfx.renderer2d.gfx_print(&mut font, &text, x,y);
            Ok(())
        });
    }
}
