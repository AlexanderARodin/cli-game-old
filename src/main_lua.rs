use mlua::prelude::*;
use mlua::{Function, Variadic};

pub fn main_lua_loop(lua: Lua, main_lua_code: &str) -> mlua::Result<()> {
    let globals = lua.globals();

    let lua_print = lua.create_function( |_, lua_args: Variadic<String>| {
        invoke_lua_print(&lua_args);
        Ok(())
    })?;
    globals.set("print", lua_print)?;

    lua.load( main_lua_code ).exec()?;
    let setup_params = lua.create_table()?;
    let call_lua_setup: Function = globals.get("setup")?;
    let _lua_setup_result = call_lua_setup.call::<_, ()>(setup_params)?;
    
    enter_loop(&lua, &globals)?;

    Ok(())
}

// // // // // // // //
fn enter_loop(_lua: &Lua, globals: &mlua::Table) -> mlua::Result<()> {
    let call_lua_update: Function = globals.get("update")?;

    for time in 1..5 {
        let txt = call_lua_update.call::<_, String>(time)?;
        println!("time = {} : {}", time, txt);
    }

    Ok(())
}

// // // // // // // //
fn invoke_lua_print(args: &Variadic<String>) {
    print!("\nLUA:\t");
    for item in args.iter() {
        print!("{} - ", item);
    }
    print!("\n");
}

