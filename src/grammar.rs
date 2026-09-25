use crate::dictionary::Dictionary;

#[derive(Debug, Clone)]
pub struct GrammarCorrector {
    dictionary: Dictionary,
}

impl GrammarCorrector {
    pub fn new(dictionary: Dictionary) -> Self {
        Self { dictionary }
    }

    pub fn correct(&self, text: &str) -> String {
        let mut result = normalize_spacing(text);
        result = fix_repeated_words(&result);
        result = fix_common_mistakes(&result);
        result = fix_sentence_capitalization(&result);
        result = fix_punctuation(&result);
        result
    }
}

fn normalize_spacing(text: &str) -> String {
    let trimmed = text.trim();
    trimmed
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
}

fn fix_repeated_words(text: &str) -> String {
    let mut words: Vec<String> = text
        .split_whitespace()
        .map(|part| part.to_string())
        .collect();

    let mut corrected = Vec::new();
    for word in words {
        if let Some(last) = corrected.last() {
            if last.eq_ignore_ascii_case(&word) {
                continue;
            }
        }
        corrected.push(word);
    }

    corrected.join(" ")
}

fn fix_common_mistakes(text: &str) -> String {
    let replacements = [
        ("teh", "the"),
        ("thier", "their"),
        ("recieve", "receive"),
        ("seperate", "separate"),
        ("occured", "occurred"),
        ("definately", "definitely"),
        ("wierd", "weird"),
        ("adn", "and"),
        ("dont", "don't"),
        ("cant", "can't"),
        ("wont", "won't"),
        ("im", "I'm"),
        ("ive", "I've"),
    ];

    let mut result = text.to_string();
    for (wrong, right) in replacements {
        result = result.replace(wrong, right);
    }

    result
}

fn fix_sentence_capitalization(text: &str) -> String {
    let mut chars = text.chars().peekable();
    let mut out = String::new();
    let mut at_sentence_start = true;

    while let Some(ch) = chars.next() {
        if ch == '.' || ch == '!' || ch == '?' {
            out.push(ch);
            at_sentence_start = true;
            if let Some(next) = chars.peek() {
                if next.is_whitespace() {
                    out.push(' ');
                }
            }
            continue;
        }

        if at_sentence_start && !ch.is_whitespace() {
            out.extend(ch.to_uppercase());
            at_sentence_start = false;
            continue;
        }

        if ch == 'i' && at_sentence_start {
            out.push('I');
            at_sentence_start = false;
            continue;
        }

        out.push(ch);
        if ch.is_whitespace() {
            at_sentence_start = true;
        }
    }

    out
}

fn fix_punctuation(text: &str) -> String {
    let mut result = text.to_string();
    result = result.replace(" ,", ",");
    result = result.replace(" .", ".");
    result = result.replace(" !", "!");
    result = result.replace(" ?", "?");
    result = result.replace(" ;", ";");

    if !result.is_empty() && !result.ends_with(['.', '!', '?']) {
        result.push('.');
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn repeated_words_are_removed() {
        let c = GrammarCorrector::new(Dictionary::from_words(vec!["hello", "world"]));
        let corrected = c.correct("hello hello world");
        assert_eq!(corrected, "Hello world.");
    }

    #[test]
    fn common_typos_are_fixed() {
        let c = GrammarCorrector::new(Dictionary::from_words(vec!["the", "their", "receive"]));
        let corrected = c.correct("teh thing");
        assert!(corrected.contains("The"));
    }
}
