use anyhow::Result;

use mlua::prelude::*;
use mlua::{Variadic,Value};
use mlua::Function;


// // // // // // // //
pub fn main_lua(main_lua_code: &str) -> Result<()> {
    let lua = internal_utils::init_lua(main_lua_code, main_printer )?;
    let globals = lua.globals();

    //
    //let setup_params = lua.create_table()?;
    //let call_lua_setup: Function = globals.get("setup")?;
    //let _lua_setup_result = call_lua_setup.call::<_, ()>(setup_params)?;


    enter_loop(&lua, &globals)?;

    Ok(())
}

fn main_printer(txt_list: Vec<String>) {
    use colored::Colorize;
    println!( "{} {}", "LUA:".blue().bold(), format!("{:?}",txt_list).green() );
}


// // // // // // // //
mod internal_utils {
    use super::*;

    pub(super) fn init_lua( main_lua_code: &str, printer: fn(Vec<String>)->() ) -> Result< Lua > {
        let lua = Lua::new();

        set_printer(&lua, printer)?;

        // compile and run once
        lua.load( main_lua_code ).exec()?;

        Ok( lua )
    }

    fn set_printer(lua: &Lua, printer: fn(Vec<String>)->() ) -> Result<()> {
        let lua_print = lua.create_function( move |_, lua_args: Variadic<Value>| {
            internal_utils::lua_printer(&lua_args, printer );
            Ok(())
        })?;
        lua.globals().set("print", lua_print)?;
        Ok(())
    }

    fn lua_printer( print_args: &Variadic<Value>, printer: fn(Vec<String>)->() ) {
        let mut arg_list: Vec<String> = Vec::new();
        for item in print_args.iter() {
            arg_list.push( match item.to_string() {
                Ok(s) => s,
                Err(_) => String::from("<error>"),
            });
        }
        printer(arg_list)
    }
}



// // // // // // // //
fn enter_loop(_lua: &Lua, globals: &mlua::Table) -> Result<()> {
    let call_lua_update: Function = globals.get("update")?;

    for time in 1..5 {
        let txt = call_lua_update.call::<_, String>(time)?;
        println!("time = {} : {}", time, txt);
    }

    Ok(())
}


//  //  //  //  //  //  //  //  //  //
//          TEST                    //
//  //  //  //  //  //  //  //  //  //
#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::anyhow;

    #[test]
    fn basic_creating() -> Result<()> {
        let code = "-- g o o d  c o d e";
        let _ = internal_utils::init_lua(code, |_|{})?;
        Ok(())
    }

    #[test]
    fn basic_fail_loading() -> Result<()> {
        let code = "b r o k e n  c o d e";
        let ilua = internal_utils::init_lua(code, |_|{});
        match ilua {
            Err(_) => return Ok(()),
            Ok(_) => return Err( anyhow!("Must be a Lua syntax Error") ),
        }
    }

    static LOGGER_BUF: std::sync::Mutex<String> = std::sync::Mutex::new(String::new());
    #[test]
    fn basic_printer() -> Result<()> {
        let code = "print('simple', 2, nil, 'another')";
        let ss = r#"["simple", "2", "nil", "another"]"#;
        {
            let _ = internal_utils::init_lua(code,
                |arg_list| {
                    let mut ns = LOGGER_BUF.lock().unwrap();
                    *ns = format!( "{:?}", arg_list );
                } )?;
        }
        assert!( ss == *LOGGER_BUF.lock().unwrap() ); 
        Ok(())
    }
}

