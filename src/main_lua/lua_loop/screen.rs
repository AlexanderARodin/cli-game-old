use std::io::{stdout,Stdout,Write};

use anyhow::Result;


use crossterm::{queue, execute, cursor};
use crossterm::terminal::*;
use crossterm::style::{*, Print};


//  //  //  //  //  //  //  //
pub struct AltScreen {
    stdout: Stdout,
}

impl AltScreen {
    pub fn new() -> Result<Self> {
        let mut new_stdout = stdout();

        new_stdout.flush()?;
        if !cfg!(test) {
            crossterm::execute!(new_stdout, EnterAlternateScreen)?;
            crossterm::execute!( new_stdout, Clear(ClearType::All) )?;
        }else{
            println!("--> EnterAlternateScreen");
        }

        Ok(
            Self {
                stdout: new_stdout,
            }
        )
    }
}

impl Drop for AltScreen {
    fn drop(&mut self) {
        if !cfg!(test) {
            let _ = crossterm::execute!(self.stdout, LeaveAlternateScreen);
        }else{
            println!("<-- LeaveAlternateScreen");
        }
        let _ = self.stdout.flush();
    }
}

//  //  //  //  //  //  //  //
fn redraw_items(stdout: &mut Stdout, items: &[(u16,u16,String)] ) -> Result<()>{
    for (x,y,s) in items.iter() {
        print_on_pos(stdout, *x,*y,s)?;
    }
    Ok(())
}

impl AltScreen {
    pub fn clean(&mut self) -> Result<()> {
        queue!(
            self.stdout,
            Clear(ClearType::All),
            cursor::Hide,
        )?;
        Ok(())
    }

    pub fn show_state(&mut self, items: &[(u16,u16,String)], is_prompt: bool) -> Result<()> {
        self.stdout.flush()?;
        self.begin_synchro()?;
        {
            redraw_background(&mut self.stdout)?;

            redraw_items(&mut self.stdout, items)?;

            if is_prompt {
                show_prompt(&mut self.stdout)?;
            }
        }
        self.end_synchro()?;
        self.stdout.flush()?;
        Ok(())
    }

    fn begin_synchro(&mut self) -> Result<()> {
        execute!(
            self.stdout,
            BeginSynchronizedUpdate,
        )?;
        Ok(())
    }
    fn end_synchro(&mut self) -> Result<()> {
        execute!(
            self.stdout,
            EndSynchronizedUpdate,
        )?;
        Ok(())
    }

}

//  //  //  //  //  //  //  //
fn show_prompt(stdout: &mut Stdout) -> Result<()>{
    queue!(
        stdout,
        cursor::MoveTo(0,38),
        Print( ">" ),
        cursor::Show,
    )?;
    Ok(())
}

fn redraw_background(stdout: &mut Stdout) -> Result<()>{
    queue!(
        stdout,
        cursor::MoveTo(2,0),
        Print( X_AXIS ),
        Print( Y_AXIS ),
    )?;
    for xx in 0..16 {
        for yy in 0..16 {
            print_on_pos(stdout, xx, yy, &get_empty_item_text() )?;
        }
    }
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

//  //  //  //  //  //  //  //
fn get_empty_item_text() -> String {
    let styled = "   "
        .with(Color::Rgb{r:128,g:128,b:128})
        .on(Color::Rgb{r:32,g:32,b:32});

    styled.to_string()
}


//  //  //  //  //  //  //  //
static X_AXIS: &str =  "  0:0 0:1 0:2 0:3 0:4 0:5 0:6 0:7 0:8 0:9 0:A 0:B 0:C 0:D 0:E 0:F\n";
static Y_AXIS: &str = "\n0:0\n\n1:0\n\n2:0\n\n3:0\n\n4:0\n\n5:0\n\n6:0\n\n7:0\n\n8:0\n\n9:0\n\
                       \nA:0\n\nB:0\n\nC:0\n\nD:0\n\nE:0\n\nF:0\n";
