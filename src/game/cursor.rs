use std::io::{self, stdout};

use crossterm::{
    cursor::{self, SetCursorStyle},
    ExecutableCommand,
};

pub fn init() -> io::Result<()> {
    let mut stdout = stdout();
    stdout.execute(cursor::Show)?;
    stdout.execute(cursor::MoveTo(0, 0))?;
    stdout.execute(SetCursorStyle::SteadyBar)?;
    Ok(())
}

pub fn move_to(x: u16, y: u16) -> io::Result<()> {
    stdout().execute(cursor::MoveTo(x, y))?;
    Ok(())
}
