pub fn reply(message: &str) -> &str {
    if message.trim().chars().all(|letter| {
        if letter.is_ascii_alphabetic() {
            letter.is_ascii_uppercase()
        } else {
            true
        }
    }) && message.trim().ends_with('?')
        && message.trim().len() != 0
        && message
            .trim()
            .chars()
            .any(|alpha| alpha.is_ascii_alphabetic())
    {
        return &"Calm down, I know what I'm doing!";
    }

    if message.trim().chars().all(|letter| {
        if letter.is_ascii_alphabetic() {
            letter.is_ascii_uppercase()
        } else {
            true
        }
    }) && message.trim().len() != 0
        && message
            .trim()
            .chars()
            .any(|alpha| alpha.is_ascii_alphabetic())
    {
        return &"Whoa, chill out!";
    }

    if message.trim().ends_with('?') && message.trim().len() != 0 {
        return &"Sure.";
    }

    if message.trim() == "" {
        return &"Fine. Be that way!";
    }

    return &"Whatever.";
}
