use std::collections::HashMap;
use std::fs;
use std::path::Path;

#[derive(Debug, Clone)]
pub struct Dictionary {
    words: std::collections::HashSet<String>,
}

impl Dictionary {
    pub fn load_from_file(path: impl AsRef<Path>) -> Result<Self, std::io::Error> {
        let content = fs::read_to_string(path)?;
        let words = content
            .lines()
            .map(str::trim)
            .filter(|line| !line.is_empty())
            .map(|line| normalize_word(line))
            .collect();

        Ok(Self { words })
    }

    pub fn from_words<I, S>(words: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        let words = words
            .into_iter()
            .map(|value| normalize_word(&value.into()))
            .collect();

        Self { words }
    }

    pub fn contains(&self, word: &str) -> bool {
        self.words.contains(&normalize_word(word))
    }

    pub fn suggestions(&self, word: &str, limit: usize) -> Vec<String> {
        let target = normalize_word(word);
        if target.is_empty() {
            return Vec::new();
        }

        let mut matches = Vec::new();
        let mut seen = std::collections::HashSet::new();

        for candidate in &self.words {
            if candidate == &target {
                continue;
            }

            let distance = levenshtein_distance(&target, candidate);
            let length_similarity = (candidate.len() as i32 - target.len() as i32).abs() <= 2;

            if distance <= 2 && length_similarity {
                if seen.insert(candidate.clone()) {
                    matches.push(candidate.clone());
                }
            }

            if matches.len() >= limit {
                break;
            }
        }

        matches
    }
}

pub fn normalize_word(word: &str) -> String {
    word.trim()
        .to_lowercase()
        .chars()
        .filter(|ch| ch.is_alphanumeric())
        .collect()
}

fn levenshtein_distance(a: &str, b: &str) -> usize {
    let a_chars: Vec<char> = a.chars().collect();
    let b_chars: Vec<char> = b.chars().collect();
    let mut dp = vec![vec![0usize; b_chars.len() + 1]; a_chars.len() + 1];

    for i in 0..=a_chars.len() {
        dp[i][0] = i;
    }
    for j in 0..=b_chars.len() {
        dp[0][j] = j;
    }

    for i in 1..=a_chars.len() {
        for j in 1..=b_chars.len() {
            let cost = if a_chars[i - 1] == b_chars[j - 1] { 0 } else { 1 };
            let deletion = dp[i - 1][j] + 1;
            let insertion = dp[i][j - 1] + 1;
            let substitution = dp[i - 1][j - 1] + cost;
            dp[i][j] = deletion.min(insertion).min(substitution);
        }
    }

    dp[a_chars.len()][b_chars.len()]
}

pub fn default_dictionary() -> Dictionary {
    let words = vec![
        "hello",
        "world",
        "rust",
        "libreoffice",
        "spell",
        "checker",
        "grammar",
        "language",
        "document",
        "version",
        "correct",
        "corrector",
        "text",
        "sentence",
        "word",
        "which",
        "that",
        "this",
        "there",
        "their",
        "they",
        "them",
        "with",
        "without",
        "from",
        "into",
        "before",
        "after",
        "because",
        "through",
        "under",
        "over",
        "above",
        "below",
        "where",
        "when",
        "why",
        "what",
        "how",
        "many",
        "most",
        "some",
        "any",
        "all",
        "the",
        "and",
        "or",
        "is",
        "are",
        "was",
        "were",
        "be",
        "been",
        "being",
        "have",
        "has",
        "had",
        "do",
        "does",
        "did",
        "can",
        "could",
        "should",
        "would",
        "may",
        "might",
        "must",
        "will",
        "you",
        "your",
        "yours",
        "i",
        "me",
        "my",
        "mine",
        "we",
        "us",
        "our",
        "ours",
        "they",
        "them",
        "their",
        "theirs",
        "he",
        "she",
        "it",
        "his",
        "her",
        "hers",
        "its",
        "a",
        "an",
        "to",
        "of",
        "in",
        "on",
        "at",
        "for",
        "by",
        "as",
        "if",
        "then",
        "than",
        "not",
        "too",
        "also",
        "very",
        "more",
        "less",
        "much",
        "good",
        "great",
        "error",
        "errors",
        "improve",
        "improves",
        "improved",
        "writing",
        "check",
        "checking",
        "checked",
        "project",
        "example",
        "module",
        "function",
        "interface",
        "api",
        "package",
        "software",
        "engine",
        "code",
        "application",
        "extension",
        "uno",
        "office",
        "libre",
        "writer",
        "calc",
        "impress",
        "tools",
        "script",
        "python",
        "rust",
        "bridge",
        "dictionary",
        "corrector",
        "sentence",
        "paragraph",
        "rule",
        "rules",
        "checking",
        "language",
        "grammar",
    ];

    Dictionary::from_words(words)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dictionary_normalizes_words() {
        let dict = Dictionary::from_words(vec!["Hello", "World"]);
        assert!(dict.contains("hello"));
        assert!(dict.contains("world"));
    }

    #[test]
    fn suggestions_work() {
        let dict = Dictionary::from_words(vec!["hello", "world", "rust"]);
        let suggestions = dict.suggestions("hallo", 3);
        assert!(suggestions.contains(&"hello".to_string()));
    }
}
