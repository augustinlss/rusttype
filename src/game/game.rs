use std::io::{self, stdout, Write};

use anyhow::Result;
use crossterm::{
    event::{self, Event, KeyCode},
    style::{Color, Print, SetForegroundColor},
    ExecutableCommand,
};

use super::words::generate_target_words;

pub struct Game {
    pub target_words: Vec<String>,
    pub current_word: usize,
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
        current_word: 0,
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

        stdout.flush();

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
        let mut stdout = stdout();

        Ok(())
    }
}
