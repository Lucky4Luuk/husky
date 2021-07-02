use std::ops::{Deref, DerefMut};

use mlua::{Chunk, Table, Function};
use mlua::prelude::*;

pub struct LuaProgram {
    lua: Lua,
}

impl LuaProgram {
    fn new_lua_env() -> Lua {
        Lua::new()
    }

    pub fn from_source<F>(source: &str, api_load: F) -> LuaResult<Self>
    where
        F: Fn(&Lua, &Table) -> LuaResult<()>
    {
        let lua = Self::new_lua_env();

        let api_table = lua.create_table()?;

        api_load(&lua, &api_table)?;

        lua.globals().set("husky", api_table)?;

        lua.load(source).exec()?;

        Ok(Self {
            lua: lua,
        })
    }

    pub fn update(&self, dt_s: f32) {
        match self.lua.globals().get::<&str, Table>("husky") {
            Err(_) => {}, //Husky table doesn't exist
            Ok(api) => {
                match api.get::<&str, Function>("update") {
                    Err(_) => {}, //Update function in husky table doesn't exist
                    Ok(func) => {
                        func.call::<_, ()>(dt_s).expect("Failed to call update function!");
                    }
                }
            }
        }
    }

    pub fn draw(&self) {
        match self.lua.globals().get::<&str, Table>("husky") {
            Err(_) => {}, //Husky table doesn't exist
            Ok(api) => {
                match api.get::<&str, Function>("draw") {
                    Err(_) => {}, //Update function in husky table doesn't exist
                    Ok(func) => {
                        func.call::<_, ()>(()).expect("Failed to call update function!");
                    }
                }
            }
        }
    }
}
