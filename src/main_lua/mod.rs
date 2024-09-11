use anyhow::Result;

use mlua::prelude::*;
use mlua::{Variadic,Value};
use mlua::Function;


mod lua_loop;

// // // // // // // //
pub fn main_lua(main_lua_code: &str) -> Result<()> {
    let lua = internal_utils::init_lua(main_lua_code,
        |txt_list: Vec<String>| {
            use crossterm::style::Stylize;
            let mut the_first = true;
            for item in txt_list.iter() {
                let item_txt = String::from(item);
                if the_first {
                    the_first = false;
                    print!( "{} {}", "LUA:".bold().green(), item_txt.magenta() );
                }else{
                    print!( "\t{}", item_txt.magenta() );
                }
            }
            println!();
        }
    )?;

    let call_lua_update: Function = lua.globals().get("update")?;
    let res = lua_loop::enter_loop(&lua, &call_lua_update);

    res
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



//  //  //  //  //  //  //  //  //  //
//          TEST                    //
//  //  //  //  //  //  //  //  //  //
#[cfg(test)]
mod tests {
    use super::*;

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
        assert!( ilua.is_err(), "Must be a Lua syntax Error" );
        Ok(())
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

    #[test]
    fn ok_loading() -> Result<()> {
        let code = "function update() return {target={x=2,y=3}} end";
        let _ = main_lua(code)?;
        Ok(())
    }

    #[test]
    fn fail_loading() -> Result<()> {
        let code = "-- there is no UPDATE function";
        let ilua = main_lua(code);
        assert!( ilua.is_err(), "Must be a runtime error of abscent UPDATE function" );
        Ok(())
    }
}

