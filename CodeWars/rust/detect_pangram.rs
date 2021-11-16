use std::collections::HashSet;

fn is_pangram(sentence: &str) -> bool {
  return sentence
    .split_whitespace()
    .collect::<String>()
    .to_ascii_lowercase()
    .chars()
    .collect::<HashSet<char>>().len() == 26;
}

fn main() {
  let sentence = "The quick brown fox jumps over the lazy dog";

  println!("{}", is_pangram(sentence));
}

#[test]
fn test_pangram_all_chars_needed_is_present() {
  let sentence = "The quick brown fox jumps over the lazy dog";
  assert_eq!(is_pangram(sentence), true);
}

#[test]
fn test_pangram_all_chars_needed_is_not_present() {
  let sentence = "Quick fox is quick";
  assert_eq!(is_pangram(sentence), false);
}