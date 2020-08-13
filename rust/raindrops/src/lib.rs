pub fn raindrops(num: i32) -> String {
  let mut result = String::new();

  if num % 3 == 0 {
    result.push_str("Pling");
  }

  if num % 5 == 0 {
    result.push_str("Plang");
  }

  if num % 7 == 0 {
    result.push_str("Plong");
  }

  if result.len() == 0 {
    return num.to_string();
  }

  result
}