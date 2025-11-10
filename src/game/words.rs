use std::{fs, io};

use rand::seq::IteratorRandom;

const WORDS_FILE_PATH: &str = "../word_list.txt";

pub fn generate_target_words(quantity: usize) -> Result<Vec<String>, io::Error> {
    let contents = fs::read_to_string(WORDS_FILE_PATH)?;

    let mut rng = rand::thread_rng();
    let words: Vec<String> = contents
        .lines()
        .filter(|s| !s.trim().is_empty())
        .map(|s| s.to_string())
        .choose_multiple(&mut rng, quantity)
        .into_iter()
        .collect();

    if words.is_empty() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "There was a problem generating words.",
        ));
    }

    return Ok(words);
}
