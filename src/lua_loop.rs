use std::io::{stdout, Stdout,Write, stdin,Read};
use std::{thread::sleep, time::Duration};

use anyhow::Result;
use anyhow::anyhow;



pub fn enter_loop(_lua: &mlua::Lua, update: &mlua::Function) -> Result<()> {
    //let globals = lua.globals();

    let _lua_update_result = update.call::<_, ()>( mlua::Value::Integer(-1) )?;

    let mut update_result: mlua::Table;
    let mut target_pos: mlua::Table;
    let mut target_x: i64;
    let mut target_y: i64;

    for time in 0..5 {
        update_result = update.call::<_, mlua::Table>( mlua::Value::Integer(time) )?;
        target_pos = update_result.get("target")?;
        target_x = target_pos.get("x")?;
        target_y = target_pos.get("y")?;

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
        user_input::get_it()?;
        //apply_input()?;
        //break;
        let _ = stdin().read(&mut [0_u8])?;
    }

        let _ = stdin().read(&mut [0_u8])?;
    Ok(())
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


//  //  //  //  //  //  //  //
mod user_input {
    use super::*;
    use colored::Colorize;

    pub(super) fn get_it() -> Result<()>{
        let src_input_string = match input_string() {
            Err(e) => {
                eprintln!("{}{}", "\n\nE: ".bold().red(), e.to_string().red() );
                sleep( Duration::from_millis(300) );
                return Ok(());
            },
            Ok(s) => {
                s
            },
        };

        println!("\ninput content: \n<{}>",src_input_string.green() );

        let ex = expand_string( &src_input_string )?;
        println!("\nexpanded: \n<{}>",ex.blue() );
        sleep( Duration::from_millis(1000) );

        Ok(())
    }

    fn input_string() -> Result<String>{
        let mut res_string = String::new();
        let mut buf = [43_u8;128];
        loop{
            let n = stdin().read(&mut buf)?;
            if n == 0 {
                break;
            }
            res_string += std::str::from_utf8( &buf[..(n)] )?;
            if n < 10 {
                break;
            }
        }
        Ok( res_string )
    }

    fn expand_string(s: &str) -> Result<String>{
        let mut res = String::new();
        let mut mult: Option<u32> = None;

        for ch in s.chars() {
            match ch {
                '\n' => break,
                '\t' => res.push_str( "<tab>" ),
                '0'..'9' => {
                    let num:u32 = (ch as u32) - 48;
                    if let Some(m) = mult {
                        mult = Some( num + m*10 );
                    }else{
                        mult = Some(num);
                    }
                    println!("<#{:?}>", mult);
                },
                'h' | 'j' | 'k' | 'l' => {
                    let final_num = match mult {
                        None => 1,
                        Some(n) => n,
                    };
                    for _ in 0..final_num {
                        res.push( ch );
                    }
                    mult = None;
                },
                _ => return Err( anyhow!("unsupported character") ),
            };
            //res.push( ch );
        }

        Ok(res)
    }
}


