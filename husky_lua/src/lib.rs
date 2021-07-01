use mlua::Chunk;
use mlua::prelude::*;

pub struct LuaProgram<'a> {
    chunk: Chunk<'a, 'a>,
}

impl<'a> LuaProgram<'a> {
    pub fn new_lua_env() -> Lua {
        Lua::new()
    }

    pub fn from_source(lua: &'a Lua, source: &'a str) -> LuaResult<Self> {
        let api_table = lua.create_table()?;

        lua.globals().set("husky", api_table)?;

        let chunk = lua.load(source);

        Ok(Self {
            chunk: chunk,
        })
    }
}
