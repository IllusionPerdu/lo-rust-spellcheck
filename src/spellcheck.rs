use std::ffi::{CStr, CString};
use std::os::raw::c_char;

mod dictionary;
mod grammar;
mod spellcheck;

use dictionary::{default_dictionary, Dictionary};
use grammar::GrammarCorrector;
use spellcheck::SpellChecker;

static mut GLOBAL_DICTIONARY: Option<Dictionary> = None;
static mut GLOBAL_SPELLCHECKER: Option<SpellChecker> = None;
static mut GLOBAL_GRAMMAR: Option<GrammarCorrector> = None;

#[no_mangle]
pub extern "C" fn lo_spellcheck_init(dict_path: *const c_char) -> i32 {
    let dict = if dict_path.is_null() {
        default_dictionary()
    } else {
        let path = unsafe { CStr::from_ptr(dict_path) };
        let path_str = match path.to_str() {
            Ok(value) => value,
            Err(_) => return -1,
        };

        match Dictionary::load_from_file(path_str) {
            Ok(dictionary) => dictionary,
            Err(_) => default_dictionary(),
        }
    };

    unsafe {
        GLOBAL_DICTIONARY = Some(dict.clone());
        GLOBAL_SPELLCHECKER = Some(SpellChecker::new(dict.clone()));
        GLOBAL_GRAMMAR = Some(GrammarCorrector::new(dict));
    }

    0
}

#[no_mangle]
pub extern "C" fn lo_spellcheck_word(word: *const c_char) -> bool {
    let value = unsafe {
        if word.is_null() {
            return false;
        }
        CStr::from_ptr(word)
    };

    let word_str = match value.to_str() {
        Ok(v) => v,
        Err(_) => return false,
    };

    unsafe {
        match &GLOBAL_SPELLCHECKER {
            Some(checker) => checker.check_word(word_str),
            None => false,
        }
    }
}

#[no_mangle]
pub extern "C" fn lo_spellcheck_suggest(word: *const c_char, max_results: i32) -> *mut c_char {
    let value = unsafe {
        if word.is_null() {
            return std::ptr::null_mut();
        }
        CStr::from_ptr(word)
    };

    let word_str = match value.to_str() {
        Ok(v) => v,
        Err(_) => return std::ptr::null_mut(),
    };

    let limit = if max_results <= 0 { 5 } else { max_results as usize };

    let suggestions = unsafe {
        match &GLOBAL_SPELLCHECKER {
            Some(checker) => checker.suggest(word_str, limit),
            None => Vec::new(),
        }
    };

    let joined = suggestions.join(",");
    match CString::new(joined) {
        Ok(s) => s.into_raw(),
        Err(_) => std::ptr::null_mut(),
    }
}

#[no_mangle]
pub extern "C" fn lo_grammar_correct(sentence: *const c_char) -> *mut c_char {
    let value = unsafe {
        if sentence.is_null() {
            return std::ptr::null_mut();
        }
        CStr::from_ptr(sentence)
    };

    let sentence_str = match value.to_str() {
        Ok(v) => v,
        Err(_) => return std::ptr::null_mut(),
    };

    let corrected = unsafe {
        match &GLOBAL_GRAMMAR {
            Some(grammar) => grammar.correct(sentence_str),
            None => sentence_str.to_string(),
        }
    };

    match CString::new(corrected) {
        Ok(s) => s.into_raw(),
        Err(_) => std::ptr::null_mut(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn spellcheck_works_with_default_dictionary() {
        unsafe {
            GLOBAL_SPELLCHECKER = Some(SpellChecker::new(default_dictionary()));
        }

        let result = unsafe {
            match &GLOBAL_SPELLCHECKER {
                Some(checker) => checker.check_word("hello"),
                None => false,
            }
        };

        assert!(result);
    }
}
