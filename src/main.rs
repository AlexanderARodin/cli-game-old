use std::io::{self,Write};
use crossterm::{execute, terminal::{EnterAlternateScreen, LeaveAlternateScreen}};

use clap::Parser;

mod main_lua;
mod lua_loop;


// // // // // // // //
fn main() -> io::Result<()> {
    println!("starting..");
    {
        let args = CliArgs::parse();

        let lua_game_code = std::fs::read_to_string(args.level)
            .expect("impossible to open lua_level file");

        execute!(io::stdout(), EnterAlternateScreen)?;
        match main_lua::main_lua(&lua_game_code) {
            Ok(()) => println!("...Ok!"),
            Err(e) => {
                eprintln!("Lua: {}", e);
            },
        }
        execute!(io::stdout(), LeaveAlternateScreen)?;
    }
    Ok(())
}

// // // // // // // //
#[derive(Parser)]
#[command(version, about)]
struct CliArgs {
    #[arg(short,long, default_value = "demo_level.lua")]
    level: String,
}



// // // // // // // //
