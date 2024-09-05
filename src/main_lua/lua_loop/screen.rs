use std::io::{stdout, Stdout,Write};

use colored::Colorize;
use anyhow::Result;


use crossterm::{queue, cursor};
use crossterm::terminal::*;
use crossterm::style::Print;

use super::GameState;


//  //  //  //  //  //  //  //
pub fn show_state(state: &GameState) -> Result<()>{
    let mut stdout = stdout();
    stdout.flush()?;
    {
        redraw_background(&mut stdout)?;

        for xx in 0..16 {
            for yy in 0..16 {
                print_on_pos(&mut stdout, xx, yy, 
                    &format!("{}", "   ".truecolor(128,128,128).on_truecolor(32,32,32) )
                )?;
            }
        }

        let (target_x,target_y) = state.target;
        print_on_pos(&mut stdout, target_x,target_y, &format!("{}", " X ".on_red().black()) )?;

        let (player_x,player_y) = state.player;
        print_on_pos(&mut stdout, player_x,player_y,
            &format!(
                "{}{}{}",
                "[".bold(),
                "*".bold().green(),
                "]".bold()
            )
        )?;

        print_prompt(&mut stdout)?;
    }
    stdout.flush()?;
    Ok(())
}

fn redraw_background(stdout: &mut Stdout) -> Result<()>{
    queue!(
        stdout,
        Clear(ClearType::All),
        cursor::MoveTo(2,0),
        Print( "  0;0 0;1 0;2 0;3 0;4 0;5 0;6 0;7 0;8 0;9 0;A 0;B 0;C 0;D 0;E 0;F\n" ),
        Print( "\n0;0\n\n1;0\n\n2;0\n\n3;0\n\n4;0\n\n5;0\n\n6;0\n\n7;0\n\n8;0\n\n9;0\n\nA;0\n\nB;0\n\nC;0\n\nD;0\n\nE;0\n\nF;0\n" ),
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

fn print_prompt(stdout: &mut Stdout) -> Result<()>{
    queue!(
        stdout,
        cursor::MoveTo(0,36),
        Print( ":" ),
    )?;
    Ok(())
}

