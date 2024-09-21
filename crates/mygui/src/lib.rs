#![allow(clippy::cargo_common_metadata)]

use lune_utils::TableBuilder;
use mlua::prelude::*;

/**
    Creates the `mygui` standard library module.

    # Errors

    Errors when out of memory.
*/
pub fn module(lua: &Lua) -> LuaResult<LuaTable> {
    TableBuilder::new(lua)?
        .with_async_function("construct", mygui_construct)?
        .build_readonly()
}

async fn mygui_construct(_: &Lua, options: LuaTable<'_>) -> LuaResult<()> {
    println!(
        "Constructing mygui with label: {}",
        options.get::<&str, String>("label")?
    );

    Ok(())
}
