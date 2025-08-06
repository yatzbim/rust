// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

use std::collections::HashMap;

pub fn can_construct_note(magazine: &[&str], note: &[&str]) -> bool {
    let mut magazine_words = HashMap::new();
    for word in magazine {
        if let Some(count) = magazine_words.get(word) {
            magazine_words.insert(word, count + 1);
        } else {
            magazine_words.insert(word, 1);
        }
    }

    for word in note {
        let v = magazine_words.get(word);
        match v {
            Some(count) => {
                if *count == 1 {
                    magazine_words.remove(word);
                } else {
                    magazine_words.insert(word, count - 1);
                }
            }
            None => return false,
        }
    }

    true
}
