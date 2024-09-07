use std::io::{stdout, Stdout,Write};

use anyhow::Result;


use crossterm::{queue, execute, cursor};
use crossterm::terminal::*;
use crossterm::style::Print;

use super::GameState;
use super::GameStatus;


//  //  //  //  //  //  //  //
pub struct AltScreen {
    stdout: Stdout,
}

impl AltScreen {
    pub fn new() -> Result<Self> {
        let mut new_stdout = stdout();

        new_stdout.flush()?;
        if !cfg!(test) {
            crossterm::execute!(new_stdout, EnterAlternateScreen)?;
        }else{
            println!("--> EnterAlternateScreen");
        }

        Ok(
            Self {
                stdout: new_stdout,
            }
        )
    }
}
impl Drop for AltScreen {
    fn drop(&mut self) {
        if !cfg!(test) {
            let _ = crossterm::execute!(self.stdout, LeaveAlternateScreen);
        }else{
            println!("<-- LeaveAlternateScreen");
        }
        let _ = self.stdout.flush();
    }
}

//  //  //  //  //  //  //  //
impl AltScreen {
    pub fn clean(&mut self) -> Result<()> {
        queue!(
            self.stdout,
            Clear(ClearType::All),
            cursor::Hide,
        )?;
        Ok(())
    }
    pub fn show_state(&mut self, state: &GameState, is_prompt: bool) -> Result<()> {
        self.stdout.flush()?;
        self.begin_synchro()?;
        {
            redraw_background(&mut self.stdout)?;
            for xx in 0..16 {
                for yy in 0..16 {
                    print_on_pos(&mut self.stdout, xx, yy, &get_item_text::empty() )?;
                }
            }
            // print Target
            let (target_x,target_y) = state.target;
            print_on_pos(&mut self.stdout, target_x,target_y, &get_item_text::target() )?;
            // print Player
            let (player_x,player_y) = state.player;
            print_on_pos(&mut self.stdout, player_x,player_y, &get_item_text::player() )?;

            show_status(&mut self.stdout, &state.status )?;
            if is_prompt {
                show_prompt(&mut self.stdout)?;
            }
        }
        self.end_synchro()?;
        self.stdout.flush()?;
        Ok(())
    }

    fn begin_synchro(&mut self) -> Result<()> {
        execute!(
            self.stdout,
            BeginSynchronizedUpdate,
        )?;
        Ok(())
    }
    fn end_synchro(&mut self) -> Result<()> {
        execute!(
            self.stdout,
            EndSynchronizedUpdate,
        )?;
        Ok(())
    }

}

//  //  //  //  //  //  //  //
fn show_status(stdout: &mut Stdout, status: &GameStatus) -> Result<()>{
    use colored::Colorize;

    queue!(
        stdout,
        cursor::MoveTo(0,35),
    )?;
    match status {
        GameStatus::Ok => {
            print!("STATUS: {}", "Ok".green());
        },
        GameStatus::GameOver(s) => {
            print!("STATUS: {}", s.red());
        },
        GameStatus::Debug(m) => {
            print!("STATUS: {}", m.yellow());
        },
    };
    Ok(())
}

fn show_prompt(stdout: &mut Stdout) -> Result<()>{
    queue!(
        stdout,
        cursor::MoveTo(0,38),
        Print( ">" ),
        cursor::Show,
    )?;
    Ok(())
}

fn redraw_background(stdout: &mut Stdout) -> Result<()>{
    queue!(
        stdout,
        cursor::MoveTo(2,0),
        Print( "  0;0 0;1 0;2 0;3 0;4 0;5 0;6 0;7 0;8 0;9 0;A 0;B 0;C 0;D 0;E 0;F\n" ),
        Print( "\n0;0\n\n1;0\n\n2;0\n\n3;0\n\n4;0\n\n5;0\n\n6;0\n\n7;0\n\n8;0\n\n9;0\n" ),
        Print( "\nA;0\n\nB;0\n\nC;0\n\nD;0\n\nE;0\n\nF;0\n" ),
    )?;
    Ok(())
}

fn print_on_pos( stdout: &mut Stdout, x: u16, y: u16, s: &str ) -> Result<()> {
    let screen_x = x*4+4;
    let screen_y = y*2+2;
    queue!(
        stdout,
        cursor::MoveTo(screen_x, screen_y),
        Print( s ),
    )?;
    Ok(())
}

//  //  //  //  //  //  //  //
mod get_item_text {
    use colored::Colorize;

    pub(super) fn target() -> String {
        String::from(" X ")
            .on_red().black()
            .to_string()
    }

    pub(super) fn player() -> String {
        format!(
            "{}{}{}",
            "[", "*".green(), "]"
        )
    }

    pub(super) fn empty() -> String {
        String::from("   ")
            .truecolor(128,128,128).on_truecolor(32,32,32)
            .to_string()
    }

}

