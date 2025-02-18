use tokio::fs;
use mlua::prelude::*;
use lune_utils::TableBuilder;
use bstr::{BString, ByteSlice};

use super::super::util::*;

pub fn module(luau: &Lua) -> LuaResult<LuaTable> {
    TableBuilder::new(luau)?
        .with_async_function("readFile", fs_read_file)?
        .with_async_function("writeFile", fs_write_file)?
        .with_async_function("writeDir", fs_write_dir)?
        .with_async_function("removeFile", fs_remove_file)?
        .with_async_function("removeDir", fs_remove_dir)?
        .with_async_function("appendFile", fs_append_file)?
        .with_async_function("isReadOnly", fs_is_read_only)?
        .with_async_function("readOnly", fs_read_only)?
        .with_async_function("isDir", fs_is_dir)?
        .with_async_function("isFile", fs_is_file)?
        .with_async_function("size", fs_size)?
        .with_async_function("readDir", fs_read_dir)?
        .with_async_function("exists", fs_exists)?
        .build_readonly()
}

async fn fs_read_file(luau: &Lua, path: String) -> LuaResult<LuaString> {
    let bytes = fs::read(&path).await.into_lua_err()?;

    luau.create_string(bytes)
}

async fn fs_write_file(_: &Lua, (path, contents): (String, BString)) -> LuaResult<()> {
    fs::write(&path, contents.as_bytes()).await.into_lua_err()
}

async fn fs_write_dir(_: &Lua, path: String) -> LuaResult<()> {
    fs::create_dir_all(&path).await.into_lua_err()
}

async fn fs_remove_file(_: &Lua, path: String) -> LuaResult<()> {
    fs::remove_file(&path).await.into_lua_err()
}

async fn fs_remove_dir(_: &Lua, path: String) -> LuaResult<()> {
    fs::remove_dir_all(&path).await.into_lua_err()
}

async fn fs_append_file(_: &Lua, (path, contents): (String, BString)) -> LuaResult<()> {
    let file_contents = fs::read_to_string(&path).await.into_lua_err().to_owned()?;
    let owned_contents = &contents;
    let new_contents = format!("{file_contents}{owned_contents}");

    fs::write(&path, new_contents.as_bytes()).await.into_lua_err()
}

async fn fs_is_read_only(_: &Lua, path: String) -> LuaResult<bool> {
    let metadata = handle_metadata_functions(path).await.into_lua_err()?;

    Ok(metadata.permissions().readonly())
}

async fn fs_read_only(_: &Lua, (path, readonly): (String, bool)) -> LuaResult<()> {
    let metadata = handle_metadata_functions(path).await.into_lua_err()?;

    Ok(metadata.permissions().set_readonly(readonly))
}

async fn fs_is_dir(_: &Lua, path: String) -> LuaResult<bool> {
    let metadata = handle_metadata_functions(path).await.into_lua_err()?;

    Ok(metadata.is_dir())
}

async fn fs_is_file(_: &Lua, path: String) -> LuaResult<bool> {
    let metadata = handle_metadata_functions(path).await.into_lua_err()?;

    Ok(metadata.is_file())
}

async fn fs_size(_: &Lua, path: String) -> LuaResult<LuaNumber> {
    let metadata = handle_metadata_functions(path).await.into_lua_err()?;

    Ok(metadata.len() as f64)
}

async fn fs_read_dir(_: &Lua, path: String) -> LuaResult<Vec<String>> {
    let mut dir_strings = Vec::new();
    let mut dir = fs::read_dir(&path).await.into_lua_err()?;
    while let Some(dir_entry) = dir.next_entry().await.into_lua_err()? {
        if let Some(dir_name_str) = dir_entry.file_name().to_str() {
            dir_strings.push(dir_name_str.to_owned());
        } else {
            return Err(LuaError::RuntimeError(format!(
                "File name could not be converted into a string: '{}'",
                dir_entry.file_name().to_string_lossy()
            )));
        }
    }

    return Ok(dir_strings)
}

async fn fs_exists(_: &Lua, path: String) -> LuaResult<bool> {
    fs::try_exists(path).await.into_lua_err()
}
