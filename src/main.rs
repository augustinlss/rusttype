pub struct State {
    pub target_words: Vec<String>,
    pub typed_input: String,
    pub word_index: usize,
}

impl State {
    pub fn new(words: Vec<String>) -> Self {
        State {
            target_words: words,
            typed_input: String::new(),
            word_index: 0,
        }
    }

    pub fn current_word(&self) -> Option<&str> {
        self.target_words.get(self.word_index).map(|s| s.as_str())
    }
}
