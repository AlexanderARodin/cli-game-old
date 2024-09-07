use std::{thread::sleep, time::Duration};

use anyhow::Result;


mod screen;
mod command_string;
mod user_input;

//  //  //  //  //  //  //  //
pub fn enter_loop(_lua: &mlua::Lua, updater: &mlua::Function) -> Result<()> {
    let mut game_state = GameState::new()?;
    let mut alt_screen = screen::AltScreen::new()?;

    'game_loop: loop{
        game_state.update_by_lua( updater )?;
        if cfg!(test) {
            println!("test run..\n..ended!");
            return Ok(());
        }

        alt_screen.show_state(&game_state, true)?;

        let src_line = user_input::read_line()?;
        let res_line = command_string::expand(&src_line)?;
        // check for exit command
        if res_line == "q" {
            game_state.status = GameStatus::Debug("..the END!".to_string());
            break 'game_loop;
        }

        // apply user input
        alt_screen.clean()?;
        for cmd in res_line.chars() {
            match cmd {
                'j' => game_state.move_down(),
                'k' => game_state.move_up(),
                'h' => game_state.move_left(),
                'l' => game_state.move_right(),
                _ => todo!("un-un-unSupported"),
            }
            if let GameStatus::GameOver(_) = game_state.status {
                break 'game_loop;
            }
            alt_screen.show_state(&game_state, false)?;
            sleep( Duration::from_millis(100) );
        }
    } // 'game_loop

    // pre Exit
    alt_screen.clean()?;
    alt_screen.show_state(&game_state, true )?;
    sleep( Duration::from_millis(300) );
    let _ = std::io::Read::read( &mut std::io::stdin(), &mut [0_u8] )?;
    Ok(())
}


//  //  //  //  //  //  //  //
struct GameState {
    time_step: i64,
    target: (u16,u16),
    player: (u16,u16),
    status: GameStatus,
}
enum GameStatus {
    Ok,
    GameOver(String),
    Debug(String),
}

impl GameState {
    fn new() -> Result<Self> {
        let new_one = GameState{
            time_step: -1,
            target: (15,0),
            player: (0,15),
            status: GameStatus::Ok,
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

    fn move_up(&mut self) {
        if let GameStatus::GameOver(_) = self.status {
            return;
        }
        if self.player.1 <=0 {
            self.status = GameStatus::GameOver("touched the top edge!".to_string());
            return;
        }
        self.player.1 -= 1;
    }
    fn move_down(&mut self) {
        if let GameStatus::GameOver(_) = self.status {
            return;
        }
        if self.player.1 >= 15 {
            self.status = GameStatus::GameOver("touched the bottom edge!".to_string());
            return;
        }
        self.player.1 += 1;
    }
    fn move_left(&mut self) {
        if let GameStatus::GameOver(_) = self.status {
            return;
        }
        if self.player.0 <=0 {
            self.status = GameStatus::GameOver("touched the left edge!".to_string());
            return;
        }
        self.player.0 -= 1;
    }
    fn move_right(&mut self) {
        if let GameStatus::GameOver(_) = self.status {
            return;
        }
        if self.player.0 >= 15 {
            self.status = GameStatus::GameOver("touched the right edge!".to_string());
            return;
        }
        self.player.0 += 1;
    }

}

