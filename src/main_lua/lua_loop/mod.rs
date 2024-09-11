use std::{thread::sleep, time::Duration};

use anyhow::Result;


mod screen;
mod command_string;
mod user_input;
mod game;

//  //  //  //  //  //  //  //
pub fn enter_loop(_lua: &mlua::Lua, updater: &mlua::Function) -> Result<()> {
    let mut game_state = game::GameState::new()?;
    let mut alt_screen = screen::AltScreen::new()?;

    'game_loop: loop{
        game_state.update_by_lua( updater )?;
        if cfg!(test) {
            println!("test run..\n..ended!");
            return Ok(());
        }

        alt_screen.show_state( &game_state.get_visual_array(), true)?;

        let src_line = user_input::read_line()?;
        let res_line = command_string::expand(&src_line)?;

        // apply user input
        alt_screen.clean()?;
        for cmd in res_line.chars() {
            match cmd {
                'q' => game_state.invoke_command( &game::GameCommand::Exit ),
                'j' => game_state.invoke_command( &game::GameCommand::Down ),
                'k' => game_state.invoke_command( &game::GameCommand::Up ),
                'h' => game_state.invoke_command( &game::GameCommand::Left ),
                'l' => game_state.invoke_command( &game::GameCommand::Right ),
                _ => todo!("un-un-unSupported"),
            }
            if game_state.is_ended() {
                break 'game_loop;
            }
            alt_screen.show_state( &game_state.get_visual_array(), false)?;
            sleep( Duration::from_millis(100) );
        }
    } // 'game_loop

    // pre Exit
    alt_screen.clean()?;
    alt_screen.show_state( &game_state.get_visual_array(), true)?;
    sleep( Duration::from_millis(300) );
    let _ = std::io::Read::read( &mut std::io::stdin(), &mut [0_u8] )?;
    Ok(())
}


//  //  //  //  //  //  //  //

