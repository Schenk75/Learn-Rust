pub fn abbreviate(phrase: &str) -> String {
    let mut value = String::new();
    for word in phrase.trim().split_whitespace() {
        let array = word.split(|c: char| c == '_' || c == '-' || c == ':');
        for w in array {
            if w.is_empty() {
                continue;
            }

            value.push(w.chars().nth(0).unwrap().to_ascii_uppercase());

            if !w.chars().all(|c| c.is_ascii_uppercase()) {
                let caps: String = w
                    .chars()
                    .filter(|c| c.is_ascii_uppercase())
                    .skip(1)
                    .collect();
                value += &caps;
            }
        }
    }

    value
}