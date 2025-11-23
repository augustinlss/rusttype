use std::io::{self, stdout, Write};

use anyhow::Result;
use crossterm::{
    cursor,
    event::{self, Event, KeyCode},
    style::{Color, Print, SetForegroundColor},
    terminal, ExecutableCommand,
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
        let (cols, _) = terminal::size()?;

        // Calculate how many lines the text occupies
        let target_text = self.target_words.join(" ");
        let text_len = target_text.len();
        let start_x = self.start_pos.0 as usize;
        let total_chars = start_x + text_len;
        let lines_needed = (total_chars / cols as usize) + 1;

        // Clear each line
        for line_offset in 0..lines_needed {
            stdout.execute(cursor::MoveTo(0, self.start_pos.1 + line_offset as u16))?;
            stdout.execute(terminal::Clear(terminal::ClearType::CurrentLine))?;
        }

        // Move back to start and draw
        stdout.execute(cursor::MoveTo(self.start_pos.0, self.start_pos.1))?;

        for (i, target_char) in target_text.chars().enumerate() {
            let color = if i < self.typed_buffer.len() {
                if self.typed_buffer.chars().nth(i) == Some(target_char) {
                    Color::Green
                } else {
                    Color::Red
                }
            } else {
                Color::DarkGrey
            };
            stdout
                .execute(SetForegroundColor(color))?
                .execute(Print(target_char))?;
        }

        // Position cursor at the typing position
        let typed_len = self.typed_buffer.len();
        let cursor_offset = start_x + typed_len;
        let cursor_y = self.start_pos.1 + (cursor_offset / cols as usize) as u16;
        let cursor_x = (cursor_offset % cols as usize) as u16;
        
        game_cursor::move_to(cursor_x, cursor_y)?;

        stdout.flush()?;
        Ok(())
    }
}
