use std::io::{stdin,Read};
use std::{thread::sleep, time::Duration};

use colored::Colorize;
use anyhow::Result;


//  //  //  //  //  //  //  //
pub fn read_line() -> Result<String>{
    let line = match input_string() {
        Err(e) => {
            eprintln!("{}{}", "\n\nE: ".bold().red(), e.to_string().red() );
            sleep( Duration::from_millis(300) );
            return Ok("".to_owned());
        },
        Ok(s) => {
            s
        },
    };


    Ok( line )
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

