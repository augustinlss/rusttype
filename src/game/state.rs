use std::io;

use super::words::{generate_target_words, WORDS_FILE_PATH};

pub struct Game {
    pub target_words: Vec<String>,
    pub current_word: u16,
    pub state: usize,
}

pub fn start_game(words_count: usize) -> Result<Game, io::Error> {
    let target_words = generate_target_words(words_count, WORDS_FILE_PATH)?;

    Ok(Game {
        target_words,
        current_word: 0,
        state: 0,
    })
}

impl Game {
    pub fn update_game_state(&mut self, new_state: usize) -> &mut Game {
        self.state = new_state;
        return self;
    }
}
