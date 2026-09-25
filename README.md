use crate::dictionary::Dictionary;

#[derive(Debug, Clone)]
pub struct SpellChecker {
    dictionary: Dictionary,
}

impl SpellChecker {
    pub fn new(dictionary: Dictionary) -> Self {
        Self { dictionary }
    }

    pub fn check_word(&self, word: &str) -> bool {
        self.dictionary.contains(word)
    }

    pub fn suggest(&self, word: &str, limit: usize) -> Vec<String> {
        self.dictionary.suggestions(word, limit)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dictionary::Dictionary;

    #[test]
    fn known_word_is_valid() {
        let checker = SpellChecker::new(Dictionary::from_words(vec!["hello", "world"]));
        assert!(checker.check_word("hello"));
        assert!(!checker.check_word("hallo"));
    }
}
