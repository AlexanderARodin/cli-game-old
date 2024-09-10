use std::io::{stdin,BufRead};

use anyhow::Result;


//  //  //  //  //  //  //  //
pub fn read_line() -> Result<String> {
    let mut line = String::new();

    let _n = stdin()
        .lock()
        .read_line( &mut line )
        .expect("read_line error");

    Ok(line)
}

