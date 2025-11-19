use std::io::{self, stdout};

use crossterm::{
    cursor::{self, SetCursorStyle},
    ExecutableCommand,
};

pub fn init() -> io::Result<()> {
    let mut stdout = stdout();
    stdout.execute(cursor::Show)?;
    stdout.execute(SetCursorStyle::SteadyBar)?;
    Ok(())
}
