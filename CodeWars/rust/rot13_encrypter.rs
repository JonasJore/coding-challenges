// todo: need to be able to shift 13 positions to the left if
// right rotation will go out of range
fn rotate_char(c: char, n: u32) -> char {
	std::char::from_u32(c as u32 + n).unwrap_or(c)
}

fn rot13_encrypt(s: &str) -> String {
	s.chars().map(|c| rotate_char(c, 13)).collect()
}

fn main() {
  println!("{}", rot13_encrypt("a"));
}

#[test]
fn test() {
    assert_eq!(rot13_encrypt("aaaa"), "nnnn"); // succeeds
    assert_eq!(rot13_encrypt("jonas"), "wbanf"); // fails, goes out of range
}