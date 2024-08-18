use std::fs;
use clap::Parser;

mod main_lua;


fn main() {
    println!("starting..");
    {
        let args = CliArgs::parse();
        println!("..after parsing..");

        println!("trying open level -> {}..", args.level);
        let level_lua_code = fs::read_to_string(args.level).
            expect("impossible to open lua_level file");

        match main_lua::main_lua_loop(mlua::Lua::new(), &level_lua_code) {
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
