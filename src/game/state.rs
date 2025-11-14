use std::io;

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

    pub fn run(&mut self) {
        loop {}
    }
}
