pub fn reply(text: &str) -> &str {
    if text == "" || text.chars().all(char::is_whitespace) {
        return "Fine. Be that way!";
    }

    if text.trim_end().ends_with('?') {
        let question = text.chars().filter(|c| c.is_alphabetic());
        let is_all_uppercase = question.clone().all(char::is_uppercase);
        let char_count = question.count();
        if is_all_uppercase && char_count > 0 {
            return "Calm down, I know what I'm doing!";
        } else {
            return "Sure.";
        }
    }

    let alphabets = text.chars().filter(|c| c.is_alphabetic());
    if alphabets.clone().count() > 0 && alphabets.clone().all(char::is_uppercase) {
        return "Whoa, chill out!";
    }

    "Whatever."
}
