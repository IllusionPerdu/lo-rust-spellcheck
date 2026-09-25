use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::sync::Mutex;

mod dictionary;
mod grammar;
mod spellcheck;

use dictionary::{default_dictionary, Dictionary};
use grammar::GrammarCorrector;
use spellcheck::SpellChecker;

lazy_static::lazy_static! {
    static ref GLOBAL_DICTIONARY: Mutex<Option<Dictionary>> = Mutex::new(None);
    static ref GLOBAL_SPELLCHECKER: Mutex<Option<SpellChecker>> = Mutex::new(None);
    static ref GLOBAL_GRAMMAR: Mutex<Option<GrammarCorrector>> = Mutex::new(None);
}

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

    let _ = GLOBAL_DICTIONARY.lock().map(|mut d| *d = Some(dict.clone()));
    let _ = GLOBAL_SPELLCHECKER.lock().map(|mut s| *s = Some(SpellChecker::new(dict.clone())));
    let _ = GLOBAL_GRAMMAR.lock().map(|mut g| *g = Some(GrammarCorrector::new(dict)));

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

    GLOBAL_SPELLCHECKER
        .lock()
        .ok()
        .and_then(|checker| checker.as_ref().map(|c| c.check_word(word_str)))
        .unwrap_or(false)
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

    let suggestions = GLOBAL_SPELLCHECKER
        .lock()
        .ok()
        .and_then(|checker| checker.as_ref().map(|c| c.suggest(word_str, limit)))
        .unwrap_or_default();

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

    let corrected = GLOBAL_GRAMMAR
        .lock()
        .ok()
        .and_then(|grammar| grammar.as_ref().map(|g| g.correct(sentence_str)))
        .unwrap_or_else(|| sentence_str.to_string());

    match CString::new(corrected) {
        Ok(s) => s.into_raw(),
        Err(_) => std::ptr::null_mut(),
    }
}

#[no_mangle]
pub extern "C" fn lo_free_string(ptr: *mut c_char) {
    if !ptr.is_null() {
        unsafe {
            let _ = CString::from_raw(ptr);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn spellcheck_works_with_default_dictionary() {
        let _ = lo_spellcheck_init(std::ptr::null());
        assert!(lo_spellcheck_word(c"hello".as_ptr() as *const c_char));
        assert!(!lo_spellcheck_word(c"zzzzzz".as_ptr() as *const c_char));
    }
}
