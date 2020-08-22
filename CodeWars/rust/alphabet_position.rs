// https://www.codewars.com/kata/546f922b54af40e1e90001da/rust
fn alphabet_position(text: &str) -> String {
  let alpha = String::from("abcdefghijklmnopqrstuvwxyz");
  let mut new_string = String::from("");

  for character in text.to_lowercase().as_str().chars() {
    for (index, value) in alpha.chars().enumerate() {
      if character == value {
        new_string += (index + 1).to_string().as_str();
        new_string += " ";
      }
    }
  }
  new_string.trim_end().to_string()
}

fn main() {
  // tests:
  assert_eq!(
    alphabet_position("The sunset sets at twelve o' clock."),
    "20 8 5 19 21 14 19 5 20 19 5 20 19 1 20 20 23 5 12 22 5 15 3 12 15 3 11".to_string()
    );
  assert_eq!(
    alphabet_position("The narwhal bacons at midnight."),
    "20 8 5 14 1 18 23 8 1 12 2 1 3 15 14 19 1 20 13 9 4 14 9 7 8 20".to_string()
    );
}