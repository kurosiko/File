mod util;
mod terminal;
use std::io::{Error, Result};
use terminal::windows::ClearType;

use crate::terminal::{
    windows::Terminal
};
fn main() -> Result<()>{
    let mut screen = Terminal::new()?;
    screen.test()?;
    
    Ok(())
}
