pub fn reply(text: &str) -> &str {
  if text == "" {
    return "Fine. Be that way!";
  } else if text.ends_with('?') {
    return "Sure.";
  }

  let lowercases: Vec<&str> = text.matches(char::is_lowercase).collect();
  if lowercases.len() == 0 {
    return "Whoa, chill out!";
  }

  "Whatever."
}