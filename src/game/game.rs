use std::io::{self, stdout, Write};

use anyhow::Result;
use crossterm::{
    cursor,
    event::{self, Event, KeyCode},
    style::{Color, Print, SetForegroundColor},
    ExecutableCommand,
};

use super::cursor as game_cursor;
use super::words::generate_target_words;

pub struct Game {
    pub target_words: Vec<String>,
    pub typed_buffer: String,
    pub state: GameState,
    pub start_pos: (u16, u16),
}

pub enum GameState {
    WaitingToStart,
    Running,
    Finished,
    Quit,
}

pub fn create_game(words_count: usize) -> Result<Game, io::Error> {
    let target_words = generate_target_words(words_count);

    Ok(Game {
        target_words,
        typed_buffer: String::new(),
        state: GameState::WaitingToStart,
        start_pos: (0, 0),
    })
}

impl Game {
    pub fn update_game_state(&mut self, new_state: GameState) -> &mut Game {
        self.state = new_state;
        return self;
    }

    pub fn start(&mut self) -> Result<()> {
        let mut stdout = stdout();

        stdout.execute(Print("\r\n"))?;

        self.start_pos = cursor::position()?;

        game_cursor::init()?;

        // Initial cursor position
        game_cursor::move_to(self.start_pos.0, self.start_pos.1)?;

        stdout
            .execute(SetForegroundColor(Color::DarkGrey))?
            .execute(Print(self.target_words.join(" ")))?;

        stdout.flush()?;

        loop {
            if event::poll(std::time::Duration::from_millis(10))? {
                if let Event::Key(key) = event::read()? {
                    match key.code {
                        KeyCode::Char(c) => return self.run(c),
                        KeyCode::Esc => {
                            self.update_game_state(GameState::Quit);
                            return Ok(());
                        }
                        _ => {}
                    }
                }
            }
        }
    }

    pub fn run(&mut self, first_char: char) -> Result<()> {
        self.typed_buffer.push(first_char);
        self.draw_progress()?;

        self.update_game_state(GameState::Running);

        loop {
            if event::poll(std::time::Duration::from_millis(10))? {
                if let Event::Key(key) = event::read()? {
                    match key.code {
                        KeyCode::Char(c) => {
                            self.typed_buffer.push(c);
                            self.draw_progress()?
                        }
                        KeyCode::Backspace => {
                            self.typed_buffer.pop();
                            self.draw_progress()?;
                        }
                        KeyCode::Esc => {
                            self.update_game_state(GameState::Quit);
                            break;
                        }
                        _ => {}
                    }
                }
            }

            if self.typed_buffer.len() >= self.target_words.join(" ").len() {
                self.update_game_state(GameState::Finished);
                break;
            }
        }

        Ok(())
    }

    pub fn draw_progress(&self) -> Result<()> {
        let mut stdout = stdout();
        stdout.execute(cursor::MoveTo(self.start_pos.0, self.start_pos.1))?;

        for (i, target_char) in self.target_words.join(" ").chars().enumerate() {
            let color = if self.typed_buffer.chars().nth(i) == Some(target_char) {
                Color::Green
            } else if i < self.typed_buffer.len() {
                Color::Red
            } else {
                Color::DarkGrey
            };

            stdout
                .execute(SetForegroundColor(color))?
                .execute(Print(target_char))?;
        }

        let cursor_x = self.start_pos.0 + self.typed_buffer.len() as u16;
        game_cursor::move_to(cursor_x, self.start_pos.1)?;

        stdout.flush()?;
        Ok(())
    }
}
