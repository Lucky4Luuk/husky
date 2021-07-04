use std::ops::{Deref, DerefMut};

use mlua::{Chunk, Table, Function};
use mlua::prelude::*;

use husky_graphics::Renderer;

pub struct LuaProgram {
    lua: Lua,
}

impl LuaProgram {
    fn new_lua_env() -> Lua {
        Lua::new()
    }

    pub fn from_source(source: &str, window_size: (u32, u32)) -> LuaResult<Self> {
        let lua = Self::new_lua_env();
        let api_table = lua.create_table()?;

        api_table.set("graphics", Renderer::new(window_size.into()))?;
        lua.globals().set("husky", api_table)?;

        lua.load(source).exec()?;

        Ok(Self {
            lua: lua,
        })
    }

    pub fn update(&self, dt_s: f32) {
        let globals = self.lua.globals();
        if globals.contains_key("husky").expect("Somehow failed to check for key in table!") {
            let api = globals.get::<&str, Table>("husky").unwrap();
            if api.contains_key("update").expect("Somehow failed to check for key in table!") {
                api.get::<&str, Function>("update").unwrap().call::<_, ()>(dt_s).expect("Failed to call update function!");
            }
        }
    }

    pub fn draw(&self) {
        let globals = self.lua.globals();
        if globals.contains_key("husky").expect("Somehow failed to check for key in table!") {
            let api = globals.get::<&str, Table>("husky").unwrap();
            if api.contains_key("draw").expect("Somehow failed to check for key in table!") {
                api.get::<&str, Function>("draw").unwrap().call::<_, ()>(()).expect("Failed to call draw function!");
            }
        }
    }
}
