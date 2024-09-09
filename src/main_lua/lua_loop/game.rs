use anyhow::Result;
use colored::Colorize;


//  //  //  //  //  //  //  //
pub struct GameState {
    time_step: i64,
    target: (u16,u16),
    player: (u16,u16),
    status: GameStatus,
}
pub enum GameStatus {
    Ok,
    GameOver(String),
    Debug(String),
}

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

    pub fn set_exiting(&mut self) {
        self.status = GameStatus::Debug("..the END!".to_string());
    }
    pub fn is_gameover(&self) -> bool {
        match self.status {
            GameStatus::GameOver(_) => true,
            _ => false,
        }
    }


    pub fn move_up(&mut self) {
        if let GameStatus::GameOver(_) = self.status {
            return;
        }
        if self.player.1 <=0 {
            self.status = GameStatus::GameOver("touched the top edge!".to_string());
            return;
        }
        self.player.1 -= 1;
    }
    pub fn move_down(&mut self) {
        if let GameStatus::GameOver(_) = self.status {
            return;
        }
        if self.player.1 >= 15 {
            self.status = GameStatus::GameOver("touched the bottom edge!".to_string());
            return;
        }
        self.player.1 += 1;
    }
    pub fn move_left(&mut self) {
        if let GameStatus::GameOver(_) = self.status {
            return;
        }
        if self.player.0 <=0 {
            self.status = GameStatus::GameOver("touched the left edge!".to_string());
            return;
        }
        self.player.0 -= 1;
    }
    pub fn move_right(&mut self) {
        if let GameStatus::GameOver(_) = self.status {
            return;
        }
        if self.player.0 >= 15 {
            self.status = GameStatus::GameOver("touched the right edge!".to_string());
            return;
        }
        self.player.0 += 1;
    }

    pub fn get_visual_array(&self) -> Vec<(u16,u16,String)> {
        let mut res = Vec::new();

        res.push( (0,16, get_status_string(&self.status)) );


        // insert target
        let (target_x,target_y) = self.target;
        res.push( (target_x, target_y, get_target_string()) );
        // insert player
        let (player_x,player_y) = self.player;
        res.push( (player_x, player_y, get_player_string()) );

        return res;
    }

}

//  //  //  //  //  //  //  //
fn get_status_string(status: &GameStatus) -> String {
    match status {
        GameStatus::Ok => {
            return format!("\n{}", "STATUS: Ok".green());
        },
        GameStatus::GameOver(s) => {
            return format!("\n{}{}", "STATUS: GAME OVER\n".red(), s.red());
        },
        GameStatus::Debug(m) => {
            return format!("\nSTATUS: {}", m.yellow());
        },
    };
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

