use std::fs;
use clap::Parser;
use mlua::prelude::*;
use mlua::Function;


fn main() {
    println!("starting..");
    {
        let args = CliArgs::parse();
        println!("..after parsing..");

        println!("trying open level -> {}..", args.level);
        let level_lua_code = fs::read_to_string(args.level).
            expect("impossible to open lua_level file");

        let main_lua = Lua::new();
        match setup_lua(&main_lua, &level_lua_code) {
            Ok(()) => println!("...Ok!"),
            Err(e) => {
                eprintln!("Lua: {}", e);
            },
        }
        match run_lua(&main_lua) {
            Ok(()) => println!("...Ok!"),
            Err(e) => {
                eprintln!("Lua: {}", e);
            },
        }
    }
}

#[derive(Parser)]
#[command(version, about)]
struct CliArgs {
    #[arg(short,long, default_value = "demo_level.lua")]
    level: String,
}



// // // // // // // //
fn run_lua( lua: &Lua) -> mlua::Result<()> {
    let call_lua_update: Function = lua.globals().get("update")?;
    for time in 1..5 {
        let txt = call_lua_update.call::<_, String>(time)?;
        println!("time = {} : {}", time, txt);
    }
    Ok(())
}
// // // // // // // //
fn setup_lua( lua: &Lua, level_lua_code: &str ) -> mlua::Result<()> {
    lua.load( level_lua_code ).exec()?;

    let setup_params = lua.create_table()?;
    setup_params.set("X", 16)?;
    setup_params.set("Y", 15)?;

    let call_lua_setup: Function = lua.globals().get("setup")?;
    call_lua_setup.call::<_, ()>(setup_params)?;
    Ok(())
}

