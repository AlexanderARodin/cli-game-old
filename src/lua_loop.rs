use std::io::{stdin,Read};
use std::{thread::sleep, time::Duration};

use anyhow::Result;



pub fn lua_enter_loop(_lua: &mlua::Lua, update: &mlua::Function) -> Result<()> {
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
        sleep( Duration::from_millis(100) );
    }

    let _ = stdin().read(&mut [0u8])?;
    Ok(())
}
