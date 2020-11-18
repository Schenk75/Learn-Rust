pub fn reply(message: &str) -> &str {
    // "Fine. Be that way!"
    // "Whatever."
    // "Sure."
    // "Whoa, chill out!"
    // "Calm down, I know what I'm doing!"
    if message.trim().is_empty() { 
        return "Fine. Be that way!"; 
    } else if message.to_uppercase() == message && message.to_lowercase() != message {
        if message.trim().ends_with("?") {
            return "Calm down, I know what I'm doing!";
        } else {
            return "Whoa, chill out!";
        }
    } else if message.trim().ends_with("?") {
        return "Sure.";
    } else {
        return "Whatever.";
    }
}
