use anyhow::Result;
use anyhow::anyhow;


pub fn expand(src: &str) -> Result<String>{
    let mut res = String::new();
    let mut mult: Option<u32> = None;

    for ch in src.chars() {
        match ch {
            '\n' => break,
            '\r' => break,
            'q' => {
                return Ok( "q".to_owned() );
            },
            '0'..'9' => {
                let num:u32 = (ch as u32) - 48;
                if let Some(m) = mult {
                    mult = Some( num + m*10 );
                }else{
                    mult = Some(num);
                }
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
    }
    Ok(res)
}



//  //  //  //  //  //  //  //  //  //
//          TEST                    //
//  //  //  //  //  //  //  //  //  //
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simplest() -> Result<()> {
        assert!( expand("\n\n\n\nsdfasfasdf")? == "" );
        assert!( expand("\nsdfasfasdf")? == "" );
        Ok(())
    }

    #[test]
    fn simple_moving() -> Result<()> {
        let src = "hjkl";
        let res = expand(&src)?;

        assert!( res == "hjkl" );

        Ok(())
    }

    #[test]
    fn simple_moving_2() -> Result<()> {
        let src = "hjkl0h0j0k0l";
        let res = expand(&src)?;

        assert!( res == "hjkl" );

        Ok(())
    }

    #[test]
    fn ext_moving() -> Result<()> {
        let src = "hjkl1h1j1k1l2h2j2k2l";
        let res = expand(&src)?;

        assert!( res == "hjklhjklhhjjkkll" );

        Ok(())
    }

    #[test]
    fn ext_moving_2() -> Result<()> {
        let src = "12h13j11k15l";
        let res = expand(&src)?;

        assert!( res == "hhhhhhhhhhhhjjjjjjjjjjjjjkkkkkkkkkkklllllllllllllll" );

        Ok(())
    }

    #[test]
    fn quiting() -> Result<()> {

        assert!( expand("q")? == "q" );
        assert!( expand("q\n")? == "q" );
        assert!( expand("qhjkl")? == "q" );
        assert!( expand("hjklq")? == "q" );
        assert!( expand("hjqkl")? == "q" );

        Ok(())
    }

    #[test]
    fn break_on_enter() -> Result<()> {

        assert!( expand("1h2j\n3k55l")? == "hjj" );

        Ok(())
    }

    #[test]
    fn should_error() -> Result<()> {

        assert!( expand("y1h2j\n3k55l").is_err() );
        assert!( expand("1h2j\t\n3k55l").is_err() );
        assert!( expand(" ").is_err() );

        Ok(())
    }

}
