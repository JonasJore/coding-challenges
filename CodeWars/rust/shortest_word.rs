fn find_short(s: &str) -> u32 {
  let mut words: Vec<&str> = s.split(" ").collect();
  let mut shortest = 99999;

  for word in words {
    if word.len() < shortest {
      shortest = word.len();
    }
  }
  
  shortest as u32
}