use rand::seq::SliceRandom;
use rand::thread_rng;

pub const WORD_LIST: &[&str] = &[
    "the",
    "and",
    "but",
    "our",
    "new",
    "just",
    "time",
    "also",
    "good",
    "most",
    "work",
    "well",
    "life",
    "day",
    "may",
    "way",
    "see",
    "use",
    "look",
    "like",
    "make",
    "only",
    "into",
    "back",
    "must",
    "here",
    "need",
    "then",
    "know",
    "when",
    "down",
    "take",
    "year",
    "give",
    "call",
    "high",
    "same",
    "feel",
    "right",
    "thing",
    "even",
    "point",
    "state",
    "great",
    "still",
    "world",
    "might",
    "home",
    "hand",
    "never",
    "small",
    "place",
    "learn",
    "party",
    "think",
    "power",
    "stand",
    "water",
    "study",
    "every",
    "under",
    "while",
    "leave",
    "start",
    "later",
    "long",
    "show",
    "again",
    "since",
    "light",
    "money",
    "until",
    "large",
    "group",
    "first",
    "being",
    "found",
    "write",
    "report",
    "value",
    "system",
    "market",
    "design",
    "client",
    "people",
    "service",
    "energy",
    "change",
    "reason",
    "access",
    "common",
    "public",
    "source",
    "create",
    "manage",
    "simply",
    "enough",
    "across",
    "always",
    "become",
    "around",
    "trying",
    "process",
    "control",
    "theory",
    "major",
    "model",
    "effort",
    "moment",
    "matter",
    "social",
    "policy",
    "nation",
    "action",
    "future",
    "office",
    "health",
    "result",
    "school",
    "better",
    "toward",
    "concern",
    "present",
    "several",
    "certain",
    "popular",
    "whether",
    "quickly",
    "believe",
    "within",
    "provide",
    "product",
    "quality",
    "various",
    "example",
    "happen",
    "general",
    "material",
    "specific",
    "industry",
    "analysis",
    "company",
    "growth",
    "program",
    "history",
    "culture",
    "official",
    "security",
    "success",
    "address",
    "project",
    "resource",
    "strategy",
    "position",
    "development",
    "maintain",
    "essential",
    "imagine",
    "remember",
    "excellent",
    "fantastic",
    "wonderful",
    "amazing",
    "incredible",
    "beautiful",
    "dangerous",
    "important",
    "necessary",
    "available",
    "remember",
];

pub fn generate_target_words(count: usize) -> Vec<String> {
    let mut rng = thread_rng();

    WORD_LIST
        .choose_multiple(&mut rng, count)
        .cloned() // &str → &str
        .map(|w| w.to_string()) // &str → String
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_correct_number_of_words() {
        let words = generate_target_words(10);
        assert_eq!(words.len(), 10);
    }

    #[test]
    fn test_words_are_from_list() {
        let words = generate_target_words(20);

        for w in &words {
            assert!(
                WORD_LIST.contains(&w.as_str()),
                "Generated word '{}' not in WORD_LIST",
                w
            );
        }
    }

    #[test]
    fn test_no_duplicates_when_under_list_size() {
        let sample_size = 50; // Less than WORD_LIST length
        let words = generate_target_words(sample_size);

        let mut unique = words.clone();
        unique.sort();
        unique.dedup();

        assert_eq!(
            words.len(),
            unique.len(),
            "Duplicate words detected when sample size <= word list length"
        );
    }

    #[test]
    fn test_zero_words() {
        let words = generate_target_words(0);
        assert!(words.is_empty());
    }

    #[test]
    fn test_large_request_is_filled_with_unique_words() {
        // If count > WORD_LIST length, SliceRandom will NOT repeat —
        // it only samples unique words. So final len must be min(count, WORD_LIST.len())
        let requested = WORD_LIST.len() + 50;
        let words = generate_target_words(requested);

        assert_eq!(
            words.len(),
            WORD_LIST.len(),
            "generate_target_words should not produce duplicates even for large requests"
        );
    }
}
