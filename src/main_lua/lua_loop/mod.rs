use std::io::{stdin,Read};
use std::{thread::sleep, time::Duration};

use colored::Colorize;
use anyhow::Result;


mod screen;
mod command_string;
mod user_input;

//  //  //  //  //  //  //  //
pub fn enter_loop(_lua: &mlua::Lua, updater: &mlua::Function) -> Result<()> {
    let mut game_state = GameState::new()?;

        /*
        loop {
            ......
            update_state_by_input
            show_state
            wait_for_continue
        }
        show_exit_result
        */

    loop{
        game_state.update_by_lua( updater )?;
        if cfg!(test) {
            println!("test run..\n..ended!");
            return Ok(());
        }
        screen::show_state( &game_state )?;
        let src_line = user_input::read_line()?;
        let res_line = command_string::expand(&src_line)?;
        if res_line == "q" {
            break;
        }
            println!("\nexpanded: \n<{}>",res_line.blue() );
            //sleep( Duration::from_millis(1000) );

        // wait_for_continue
        let _ = stdin().read(&mut [0_u8])?;
    }

    println!("{}", "\nthe END\n".yellow() );
    sleep( Duration::from_millis(1000) );
    //let _ = stdin().read(&mut [0_u8])?;
    Ok(())
}


//  //  //  //  //  //  //  //
struct GameState {
    time_step: i64,
    target: (u16,u16),
    player: (u16,u16),
}

impl GameState {
    fn new() -> Result<Self> {
        let new_one = GameState{
            time_step: -1,
            target: (15,0),
            player: (0,15),
        };
        Ok( new_one )
    }

    fn update_by_lua(&mut self, updater: &mlua::Function) -> Result<()> {
        let update_result: mlua::Table = updater.call::<_, mlua::Table>( mlua::Value::Integer( self.time_step ) )?;
        {
            let target_pos: mlua::Table = update_result.get("target")?;
            let target_x: i64   = target_pos.get("x")?;
            let target_y: i64   = target_pos.get("y")?;
            self.target = (target_x as u16, target_y as u16);
        }

        if self.time_step == -1 {
            self.time_step = 1;
        }else{
            self.time_step += 1;
        }
        Ok(())
    }
}

