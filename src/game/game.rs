use std::io::{self, stdout, Write};

use anyhow::Result;
use crossterm::{
    cursor,
    event::{self, Event, KeyCode},
    style::{Color, Print, SetForegroundColor},
    ExecutableCommand,
};

use super::words::generate_target_words;

pub struct Game {
    pub target_words: Vec<String>,
    pub typed_words: Vec<String>,
    pub current_word_typed: String,
    pub current_word_idx: usize,
    pub state: GameState,
}

pub enum GameState {
    WaitingToStart,
    Running,
    Finished,
}

pub fn create_game(words_count: usize) -> Result<Game, io::Error> {
    let target_words = generate_target_words(words_count);

    Ok(Game {
        target_words,
        typed_words: vec![],
        current_word_typed: String::new(),
        current_word_idx: 0,
        state: GameState::WaitingToStart,
    })
}

impl Game {
    pub fn update_game_state(&mut self, new_state: GameState) -> &mut Game {
        self.state = new_state;
        return self;
    }

    pub fn start(&mut self) -> Result<()> {
        let mut stdout = stdout();

        stdout
            .execute(SetForegroundColor(Color::DarkGrey))?
            .execute(Print(self.target_words.join(" ")))?;

        stdout.flush()?;

        loop {
            if event::poll(std::time::Duration::from_millis(10))? {
                if let Event::Key(key) = event::read()? {
                    if let KeyCode::Char(c) = key.code {
                        self.state = GameState::Running;
                        return self.run(c);
                    }
                }
            }
        }
    }

    pub fn run(&mut self, first_char: char) -> Result<()> {
        self.typed_words.push(first_char.to_string());
        self.draw_progress()?;

        loop {
            if event::poll(std::time::Duration::from_millis(10))? {
                if let Event::Key(key) = event::read()? {
                    match key.code {
                        KeyCode::Char(c) => {
                            self.typed_words.push(c.to_string());
                            self.draw_progress()?
                        }
                        KeyCode::Backspace => {
                            self.typed_words.pop();
                            self.draw_progress()?;
                        }
                        KeyCode::Esc => {
                            self.state = GameState::Finished;
                            break;
                        }
                        _ => {}
                    }
                }
            }

            if self.typed_words.len() >= self.target_words.len() {
                self.state = GameState::Finished;
                break;
            }
        }

        Ok(())
    }

    pub fn draw_progress(&self) -> Result<()> {
        let mut stdout = stdout();
        stdout.execute(cursor::MoveTo(0, 0))?;

        for (i, target_char) in self.target_words.join(" ").chars().enumerate() {
            let color = if self.typed_words.join(" ").chars().nth(i) == Some(target_char) {
                Color::Green
            } else if i < self.typed_words.join(" ").len() {
                Color::Red
            } else {
                Color::DarkGrey
            };

            stdout
                .execute(SetForegroundColor(color))?
                .execute(Print(target_char))?;
        }

        stdout.flush()?;
        Ok(())
    }
}
