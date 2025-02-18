//--> Imports & Modules <--

mod libs;
mod util;

mod require;

use std;

use clap::{Command, Arg};

use tokio;

use mlua::{
    prelude::*,
    Compiler,
    StdLib,
};

//--> Type Aliases <--

//--> Structs <--

//--> Enums <--

//--> Functions & Impls <--

fn main() {
    // DO CLAP THINGS
    let matches = Command::new("rere")
        .get_matches();

    // DO MLUA THINGS
    let luau = Lua::new_with(StdLib::ALL_SAFE, LuaOptions::default()).unwrap();
    luau.set_compiler(Compiler::new()
        .set_optimization_level(1)
        .set_coverage_level(2));

    libs::inject(&luau);

    // DO TOKIO THINGS
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async {});
}
