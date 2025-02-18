// A module which contains utilities

use mlua::prelude::*;
use tokio::fs;
use tokio::fs::File;
use std::fs::Metadata;

pub async fn handle_metadata_functions(path: String) -> Result<Metadata, LuaError> {
    let file_not_found_err_message = format!("file at {path} could not be found"); // Declare a file not found error message

    // Make sure the file exists
    match fs::try_exists(&path).await {
        Ok(true) => {
            let file = File::open(path).await.into_lua_err()?;
            let metadata = file.metadata().await.into_lua_err()?;

            return Ok(metadata)
        } // Handle case where the file exists

        Ok(false) => Err(LuaError::RuntimeError(file_not_found_err_message)), // Handle when the file doesn't exist by doing runtime error
        Err(e) => Err(LuaError::RuntimeError(e.to_string())) // Handle error case by doing runtime error
    }
}
