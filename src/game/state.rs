pub struct GameState {
    pub target_words: Vec<String>,
    pub current_word: u16,
    pub state: bool,
}

pub fn update_game_state(state: &mut GameState, new_state: bool) -> &mut GameState {
    state.state = new_state;
    return state;
}

pub fn start_game(state: &mut GameState) -> &mut GameState {
    let target_words = 
    return &mut GameState {
        target_words
    }
}
