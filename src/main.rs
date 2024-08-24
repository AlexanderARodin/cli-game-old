use clap::Parser;

mod main_lua;


// // // // // // // //
fn main() {
    println!("starting..");
    {
        let args = CliArgs::parse();

        let lua_game_code = std::fs::read_to_string(args.level)
            .expect("impossible to open lua_level file");

        match main_lua::main_lua(&lua_game_code) {
            Ok(()) => println!("...Ok!"),
            Err(e) => {
                eprintln!("Lua: {}", e);
            },
        }
    }
}

// // // // // // // //
#[derive(Parser)]
#[command(version, about)]
struct CliArgs {
    #[arg(short,long, default_value = "demo_level.lua")]
    level: String,
}



// // // // // // // //
