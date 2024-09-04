use std::io::{stdout, Stdout,Write, stdin,Read};
use std::{thread::sleep, time::Duration};

use colored::Colorize;
use anyhow::Result;


mod command_string;
mod user_input;

//  //  //  //  //  //  //  //
pub fn enter_loop(_lua: &mlua::Lua, update: &mlua::Function) -> Result<()> {

    let _lua_update_result = update.call::<_, ()>( mlua::Value::Integer(-1) )?;

    let mut update_result: mlua::Table;
    let mut target_pos: mlua::Table;
    let mut target_x: i64;
    let mut target_y: i64;

    for time in 0..5 {
        update_result = update.call::<_, mlua::Table>( mlua::Value::Integer(time) )?;
        target_pos = update_result.get("target")?;
        target_x   = target_pos.get("x")?;
        target_y   = target_pos.get("y")?;

            println!( "x={}, y={}", target_x, target_y );

        //  //  //  //
        if cfg!(test) {
            println!( "Lua testing\nx={}, y={}", target_x, target_y );
            return Ok(());
        }
        sleep( Duration::from_millis(50) );
    }

    loop {
        //call_lua_update()?;
        screen::invoke_redraw( 1, 1 )?;

        // input and converting
        let raw_line = user_input::read_line()?;
            println!("\ninput content: \n<{}>",raw_line.green() );

        let ex_line = command_string::expand( &raw_line)?;
            println!("\nexpanded: \n<{}>",ex_line.blue() );
            sleep( Duration::from_millis(1000) );

        //apply_input()?;
        //break;
        let _ = stdin().read(&mut [0_u8])?;
    }

    //let _ = stdin().read(&mut [0_u8])?;
    //Ok(())
}


//  //  //  //  //  //  //  //
mod screen {
    use super::*;
    use crossterm::terminal::*;
    use crossterm::{queue, cursor};

    pub(super) fn invoke_redraw(target_x: u16, target_y: u16) -> Result<()>{
        let mut stdout = stdout();
        stdout.flush()?;
        {
            redraw_background(&mut stdout)?;
            redraw_target(&mut stdout, target_x,target_y)?;

            print_prompt(&mut stdout)?;
        }
        stdout.flush()?;
        Ok(())
    }

    fn redraw_background(stdout: &mut Stdout) -> Result<()>{
        queue!(stdout, Clear(ClearType::All) )?;
        queue!(stdout, cursor::MoveTo(2,0) )?;

        println!(" 00 01 02 03 04 05 06 07 08 09 0A 0B 0C 0D 0E 0F");
        print!("00\n10\n20\n30\n40\n50\n60\n70\n80\n90\nA0\nB0\nC0\nD0\nE0\nF0");

        Ok(())
    }

    fn redraw_target(stdout: &mut Stdout, target_x: u16, target_y: u16) -> Result<()>{
        queue!(stdout, cursor::MoveTo(target_x*3 + 4, target_y+1) )?;
        print!("X");
        Ok(())
    }

    fn print_prompt(stdout: &mut Stdout) -> Result<()>{
        queue!(stdout, cursor::MoveTo(0,18) )?;
        print!(":");

        Ok(())
    }

}


