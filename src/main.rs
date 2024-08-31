use clap::Parser;

mod main_lua;
mod lua_loop;


// // // // // // // //
fn main() -> anyhow::Result<()> {
    println!("starting..");
    {
        let args = CliArgs::parse();

        let lua_game_code = std::fs::read_to_string(args.level)
            .expect("impossible to open lua_level file");

        let _ = main_lua::main_lua(&lua_game_code)?;
    }
    println!("..Ok!");
    Ok(())
}

// // // // // // // //
#[derive(Parser)]
#[command(version, about)]
struct CliArgs {
    #[arg(short,long, default_value = "demo_level.lua")]
    level: String,
}

