//--> Imports & Modules <--

use mlua::prelude::*;

use lune_utils::TableBuilder;
use tokio::io::{stderr, stdout, AsyncWriteExt};

//--> Type Aliases <--

//--> Structs <--

//--> Enums <--

//--> Functions & Impls <--

pub fn module(luau: &Lua) -> LuaResult<LuaTable> {
    TableBuilder::new(luau)?
        .with_async_function("write", io_write)?
        .with_async_function("ewrite", io_ewrite)?
        .build_readonly()
}

async fn io_write(_: &Lua, string: LuaString<'_>) -> LuaResult<()> {
    let mut stdout = stdout();
    stdout.write_all(string.as_bytes()).await?;
    stdout.flush().await?;
    Ok(())
}

async fn io_ewrite(_: &Lua, string: LuaString<'_>) -> LuaResult<()> {
    let mut stderr = stderr();
    stderr.write_all(string.as_bytes()).await?;
    stderr.flush().await?;
    Ok(())
}