use rand::prelude::IteratorRandom;
use std::{fs, io};

pub const WORDS_FILE_PATH: &str = "../word_list.txt";

pub fn generate_target_words(
    word_count: usize,
    word_file_path: &str,
) -> Result<Vec<String>, io::Error> {
    let contents = fs::read_to_string(word_file_path)?;

    let mut rng = rand::thread_rng();
    let target_words: Vec<String> = contents
        .lines()
        .filter(|s| !s.trim().is_empty())
        .map(|s| s.to_string())
        .choose_multiple(&mut rng, word_count)
        .into_iter()
        .collect();

    if target_words.is_empty() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "there was a problem generating words.",
        ));
    }

    return Ok(target_words);
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::io::{self, Write};

    const TEST_FILE_PATH: &str = "test_word_list.txt";

    fn cleanup_test_file() {
        let _ = fs::remove_file(TEST_FILE_PATH);
    }

    struct TestFixture;
    impl Drop for TestFixture {
        fn drop(&mut self) {
            cleanup_test_file();
        }
    }

    fn setup_test(content: &str) -> io::Result<TestFixture> {
        cleanup_test_file();
        let mut file = fs::File::create(TEST_FILE_PATH)?;
        file.write_all(content.as_bytes())?;
        Ok(TestFixture)
    }

    #[test]
    fn test_successful_generation() {
        let content = "apple\nbanana\ncherry\ndate\neggplant\nfig\ngrape";

        let _fixture = setup_test(content).expect("Failed to create test file");
        let word_count = 3;

        let result = generate_target_words(word_count, TEST_FILE_PATH);

        assert!(result.is_ok());
        let words = result.unwrap();
        assert_eq!(words.len(), word_count);
    }

    #[test]
    fn test_file_not_found() {
        let non_existent_path = "non_existent_file_12345.txt";
        let word_count = 5;

        let result = generate_target_words(word_count, non_existent_path);

        assert!(result.is_err());
        let error_kind = result.unwrap_err().kind();
        assert_eq!(error_kind, io::ErrorKind::NotFound);
    }
}
