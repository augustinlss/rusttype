use std::io::{stdout, Write};

use anyhow::Result;
use crossterm::{
    cursor::MoveTo,
    event::{self, Event, KeyCode},
    style::{Color, Print, SetForegroundColor},
    terminal::{Clear, ClearType},
    ExecutableCommand,
};

pub enum MenuAction {
    StartGame(usize), // word_count
    Quit,
}

pub struct Menu {
    selected_index: usize,
    options: Vec<(&'static str, usize)>,
}

impl Menu {
    pub fn new() -> Self {
        Self {
            selected_index: 0,
            options: vec![
                ("Word Test (10 words)", 10),
                ("Word Test (25 words)", 25),
                ("Word Test (50 words)", 50),
                ("Time Test (Coming Soon)", 0), // 0 acts as placeholder/no-op for now
                ("Quote Test (Coming Soon)", 0),
                ("Exit", 999), // 999 for quit
            ],
        }
    }

    pub fn run(&mut self) -> Result<MenuAction> {
        self.draw()?;

        loop {
            if event::poll(std::time::Duration::from_millis(100))? {
                if let Event::Key(key) = event::read()? {
                    match key.code {
                        KeyCode::Up => {
                            if self.selected_index > 0 {
                                self.selected_index -= 1;
                                self.draw()?;
                            }
                        }
                        KeyCode::Down => {
                            if self.selected_index < self.options.len() - 1 {
                                self.selected_index += 1;
                                self.draw()?;
                            }
                        }
                        KeyCode::Enter => {
                            let (_, val) = self.options[self.selected_index];
                            if val == 999 {
                                return Ok(MenuAction::Quit);
                            } else if val == 0 {
                                // Placeholder, do nothing
                            } else {
                                return Ok(MenuAction::StartGame(val));
                            }
                        }
                        KeyCode::Esc => {
                            return Ok(MenuAction::Quit);
                        }
                        _ => {}
                    }
                }
            }
        }
    }

    fn draw(&self) -> Result<()> {
        let mut stdout = stdout();
        stdout.execute(Clear(ClearType::All))?;
        stdout.execute(MoveTo(0, 0))?;

        stdout.execute(SetForegroundColor(Color::Cyan))?;
        stdout.execute(Print("rusTType - Select Mode\n\n"))?;

        for (i, (text, _)) in self.options.iter().enumerate() {
            if i == self.selected_index {
                stdout.execute(SetForegroundColor(Color::Green))?;
                stdout.execute(Print(format!("> {}\n", text)))?;
            } else {
                stdout.execute(SetForegroundColor(Color::DarkGrey))?;
                stdout.execute(Print(format!("  {}\n", text)))?;
            }
        }

        stdout.flush()?;
        Ok(())
    }
}
