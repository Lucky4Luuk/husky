#[macro_use] extern crate log;
#[macro_use] extern crate lazy_static;

use std::sync::Mutex;

use mlua::prelude::*;
use mlua::Table;

pub(crate) mod text_util;
pub(crate) mod gl_wrapper;

pub mod husky2d;

lazy_static! {
    static ref WINDOW_SIZE: Mutex<glutin::dpi::PhysicalSize<u32>> = Mutex::new((1, 1).into());
}

pub fn load_gl(gl_context: &glutin::Context<glutin::PossiblyCurrent>) {
    gl::load_with(|ptr| gl_context.get_proc_address(ptr) as *const _);
}

pub fn load_api(lua: &Lua, api: &Table) -> LuaResult<()> {
    let gfx_table = lua.create_table()?;

    let clear_func = lua.create_function(|_, (r,g,b,a): (f32, f32, f32, Option<f32>)| {
        clear(r,g,b,a.unwrap_or(1.0));
        Ok(())
    })?;
    gfx_table.set("clear", clear_func)?;

    husky2d::load_husky2d_api(lua, &gfx_table)?;

    api.set("graphics", gfx_table)?;

    Ok(())
}

pub fn on_resize(window_size: glutin::dpi::PhysicalSize<u32>) {
    *WINDOW_SIZE.lock().unwrap() = window_size;
}

pub fn clear(r: f32, g: f32, b: f32, a: f32) {
    unsafe {
        gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        gl::ClearColor(r,g,b,a);
    }
}
