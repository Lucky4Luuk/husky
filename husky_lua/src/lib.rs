#[macro_use] extern crate log;

use std::ops::{Deref, DerefMut};

use mlua::{Chunk, Table, Function};
use mlua::prelude::*;

use husky_graphics::RendererGuard;
use husky_voxel::VoxelInterface;

pub struct LuaProgram {
    lua: Lua,
    working_directory: String,
}

impl LuaProgram {
    fn new_lua_env() -> Lua {
        Lua::new()
    }

    pub fn from_source(working_directory: String, source: &str) -> LuaResult<Self> {
        let lua = Self::new_lua_env();
        let api_table = lua.create_table()?;

        api_table.set("graphics", RendererGuard::new(working_directory.clone()))?;
        api_table.set("voxel", VoxelInterface::new())?;

        lua.globals().set("husky", api_table)?;

        lua.load(source).exec()?;

        Ok(Self {
            lua: lua,
            working_directory: working_directory
        })
    }

    pub fn on_resize(&self, window_size: (u32, u32)) {
        trace!("Window resizing! New size: {:?}", window_size);
        {
            let mut win_size = husky_graphics::WINDOW_SIZE.lock().unwrap();
            *win_size = window_size;
        }
    }

    pub fn update(&self, dt_s: f32) {
        let globals = self.lua.globals();
        if globals.contains_key("husky").expect("Somehow failed to check for key in table!") {
            let api = globals.get::<&str, Table>("husky").unwrap();
            if api.contains_key("update").expect("Somehow failed to check for key in table!") {
                api.get::<&str, Function>("update").unwrap().call::<_, ()>(dt_s).expect("Failed to call update function!");
            }
        }

        //Alternative
        // let _ = self.lua.load("husky.update(0)").exec();
    }

    pub fn draw(&self) {
        let globals = self.lua.globals();
        if globals.contains_key("husky").expect("Somehow failed to check for key in table!") {
            let api = globals.get::<&str, Table>("husky").unwrap();
            if api.contains_key("draw").expect("Somehow failed to check for key in table!") {
                self.lua.load("husky.graphics:begin_frame()").exec().expect("Failed to call `husky.graphics.begin_frame`!");
                api.get::<&str, Function>("draw").unwrap().call::<_, ()>(()).expect("Failed to call draw function!");
                self.lua.load("husky.graphics:finish_frame()").exec().expect("Failed to call `husky.graphics.finish_frame`!");
            }
        }
    }
}
