//--> Imports & Modules <--

mod rere_std_io;
mod rere_std_fs;

use lune_utils::TableBuilder;

use mlua::prelude::*;

//--> Constants <--

//--> Type Aliases <--

//--> Structs <--

//--> Enums <--

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum RereStandardLibrary {
    Io,
}

//--> Functions & Impls <--

impl RereStandardLibrary {
    pub const ALL: &'static [Self] = &[
        Self::Io,
    ];

    pub fn name(&self) -> &'static str {
        match self {
            Self::Io => "io",
        }
    }

    pub fn module<'lua>(&self, luau: &'lua Lua) -> LuaResult<LuaMultiValue<'lua>> {
        let res: LuaResult<LuaTable> = match self {
            Self::Io => rere_std_io::module(luau),
        };
        match res {
            Ok(v) => v.into_lua_multi(luau),
            Err(e) => Err(e.context(format!(
                "Failed to create standard library '{}'",
                self.name()
            ))),
        }
    }
}

pub(crate) fn inject(luau: &Lua) -> LuaResult<()> {
    todo!()
}