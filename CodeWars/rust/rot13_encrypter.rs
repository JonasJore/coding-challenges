fn rotate_char(c: char, n: u32) -> char {
	match c {
			'a'...'m' | 'A'...'M' => ((c as u8) + 13) as char,
			'n'...'z' | 'N'...'Z' => ((c as u8) - 13) as char,
			_ => c
	}
}

fn rot13_encrypt(s: &str) -> String {
	s.chars().map(|c| rotate_char(c, 13)).collect()
}

fn main() {
  println!("{}", rot13_encrypt("wbanf"));
}

#[cfg(test)]
mod tests {
	use rot13_encrypt;

	#[test]
	fn simple_test() {
		assert_eq!("wbanf", rot13_encrypt("jonas"));
	}

	#[test]
	fn test_non_latin_letters() {
		assert_eq("whyrteøg", rot13_encrypt("julegrøt"));
	}

  #[test]
	fn test_numbers() {
		assert_eq("1234", rot13_encrypt("1234"));
  }
  
  #[test]
	fn reversed_if_applied_twice() {
		assert_eq!("wbanf", rot13_encrypt(&rot13_encrypt("wbanf")));
  }

  #[test]
  fn test_non_ascii_characters() {
    assert_eq!("hval 🐋 hval", rot13_encrypt("uiny 🐋 uiny"));
  }
}