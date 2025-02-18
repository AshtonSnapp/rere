//--> Imports & Modules <--

use crate::libs::RereStandardLibrary;

use std::{collections::HashMap, path::PathBuf, sync::Arc};

use lune_utils::TableBuilder;

use mlua::{prelude::*, AppDataRef};

use tokio::sync::Mutex as AsyncMutex;

//--> Constants <--

const REQUIRE_IMPL: &str = r"
return require(source(), ...)
";

//--> Structs <--

pub struct RequireContext {
    libraries: Arc<AsyncMutex<HashMap<RereStandardLibrary, LuaResult<LuaRegistryKey>>>>,
    processed: Arc<AsyncMutex<HashMap<String, LuaResult<LuaRegistryKey>>>>,
    pending: Arc<AsyncMutex<Vec<String>>>,
}

//--> Enums <--

//--> Functions & Impls <--

pub fn create(luau: &Lua) -> LuaResult<LuaValue> {
    let require_fn = luau.create_async_function(require)?;
    let get_source_fn = luau.create_function(move |luau, (): ()| match luau.inspect_stack(2) {
        None => Err(LuaError::runtime(
            "Failed to get stack info for require source",
        )),
        Some(info) => match info.source().source {
            None => Err(LuaError::runtime(
                "Stack info is missing source for require",
            )),
            Some(source) => luau.create_string(source.as_bytes()),
        },
    })?;

    let require_env = TableBuilder::new(luau)?
        .with_value("source", get_source_fn)?
        .with_value("require", require_fn)?
        .build_readonly()?;

    luau.load(REQUIRE_IMPL)
        .set_name("require")
        .set_environment(require_env)
        .into_function()?
        .into_lua(luau)
}

async fn require<'lua>(
    luau: &'lua Lua,
    (source, path): (LuaString<'lua>, LuaString<'lua>),
) -> LuaResult<LuaMultiValue<'lua>> {
    let source = source
        .to_str()
        .into_lua_err()
        .context("Failed to parse require source as string")?
        .to_string();

    let path = path
        .to_str()
        .into_lua_err()
        .context("Failed to parse require path as string")?
        .to_string();

    let context: AppDataRef<RequireContext> = luau
        .app_data_ref()
        .expect("Failed to get RequireContext from app data");

    if let Some(builtin_name) = path.strip_prefix("@rere/").map(str::to_ascii_lowercase) {
        context.load_library(luau, builtin_name)
    } else if let Some(aliased_path) = path.strip_prefix('@') {
        let (alias, path) = aliased_path.split_once('/').ok_or(LuaError::runtime(
            "Require with custom alias must contain '/' delimiter",
        ))?;
        require_alias(luau, &context, &source, alias, path).await
    } else {
        require_path(luau, &context, &source, &path).await
    }
}