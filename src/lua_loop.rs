use std::thread::sleep_ms;

use anyhow::Result;



pub fn lua_enter_loop(lua: &mlua::Lua, update: &mlua::Function) -> Result<()> {
    //let globals = lua.globals();

    let _lua_update_result = update.call::<_, ()>( mlua::Value::Integer(-1) )?;

    let mut update_result: String;
    for time in 0..5 {
        update_result = update.call::<_, String>( mlua::Value::Integer(time) )?;
        println!( "p\t{}", update_result );

        //  //  //  //
        if cfg!(test) {
            println!("AFTER LOOPer");
            return Ok(());
        }
        sleep_ms(100);
    }

    Ok(())
}
