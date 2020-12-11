use std::collections::HashMap;

pub fn check(candidate: &str) -> bool {
    let mut map = HashMap::new();
    for ch in candidate.to_ascii_lowercase().chars() {
        if ch.is_alphabetic() {
            match map.insert(ch, 1) {
                Some(_) => return false,
                None => (),
            }
        }
    }

    true
}
