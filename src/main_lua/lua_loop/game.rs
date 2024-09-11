use anyhow::Result;
use crossterm::style::Stylize;


//  //  //  //  //  //  //  //
pub struct GameState {
    time_step: i64,
    target: (u16,u16),
    player: (u16,u16),
    status: GameStatus,
}
pub enum GameStatus {
    Ok,
    Exit,
    GameOver(String),
    #[allow(dead_code)]
    Debug(String),
}

pub enum GameCommand {
    Exit,
    Up,
    Down,
    Left,
    Right,
}

//  //  //  //  //  //  //  //
impl GameState {
    pub fn new() -> Result<Self> {
        let new_one = GameState{
            time_step: -1,
            target: (15,0),
            player: (0,15),
            status: GameStatus::Ok,
        };
        Ok( new_one )
    }

    pub fn is_ended(&self) -> bool {
        matches!( self.status,
            GameStatus::GameOver(_) | GameStatus::Exit
        )
    }

    pub fn update_by_lua(&mut self, updater: &mlua::Function) -> Result<()> {
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

    pub fn invoke_command(&mut self, cmd: &GameCommand) {
        if let GameStatus::Ok = self.status {
            match cmd {
                GameCommand::Exit => {
                    self.status = GameStatus::Exit;
                },
                GameCommand::Down => {
                    self.move_down();
                },
                GameCommand::Up => {
                    self.move_up();
                },
                GameCommand::Left=> {
                    self.move_left();
                },
                GameCommand::Right => {
                    self.move_right();
                },
            }
        }
    }

    fn move_up(&mut self) {
        if self.player.1 ==0 {
            self.status = GameStatus::GameOver("touched the top edge!".to_string());
            return;
        }
        self.player.1 -= 1;
    }
    fn move_down(&mut self) {
        if self.player.1 >= 15 {
            self.status = GameStatus::GameOver("touched the bottom edge!".to_string());
            return;
        }
        self.player.1 += 1;
    }
    fn move_left(&mut self) {
        if self.player.0 ==0 {
            self.status = GameStatus::GameOver("touched the left edge!".to_string());
            return;
        }
        self.player.0 -= 1;
    }
    fn move_right(&mut self) {
        if self.player.0 >= 15 {
            self.status = GameStatus::GameOver("touched the right edge!".to_string());
            return;
        }
        self.player.0 += 1;
    }

    pub fn get_visual_array(&self) -> Vec<(u16,u16,String)> {
        let mut res = Vec::new();

        res.push( (0,16, generate_status_string(&self.status)) );

        // insert target and player
        let (target_x,target_y) = self.target;
        let (player_x,player_y) = self.player;
        if target_x == player_x && target_y == player_y {
            res.push( (target_x, target_y, get_homed_string()) );
        }else{
            res.push( (target_x, target_y, get_target_string()) );
            res.push( (player_x, player_y, get_player_string()) );
        }

        res
    }

}

//  //  //  //  //  //  //  //
fn generate_status_string(status: &GameStatus) -> String {
    match status {
        GameStatus::Ok => {
            format!("\n{}", "STATUS: Ok".green())
        },
        GameStatus::Exit => {
            format!("\n{}", "Exit\n".blue())
        },
        GameStatus::GameOver(s) => {
            format!("\n{}{}", "GAME OVER\n".red(), String::from(s).red())
        },
        GameStatus::Debug(m) => {
            format!("\nSTATUS: {}", String::from(m).yellow())
        },
    }
}

fn get_homed_string() -> String {
    String::from("[+]")
        .on_green().white()
        .to_string()
}

fn get_target_string() -> String {
    String::from(" X ")
        .on_red().black()
        .to_string()
}

fn get_player_string() -> String {
    format!(
        "{}{}{}",
        "[", "*".green(), "]"
    )
}

