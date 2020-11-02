fn filter(string: &str, character: char) -> usize {
  let filter: Vec<_> = string.chars().filter(|&val| val != character).collect();
  filter.len()
}

fn xo(string: &'static str) -> bool {
  let lower = &string.to_lowercase();
  let (exes, ohs) = (filter(lower, 'x'), filter(lower, 'o'));
  exes == ohs
}

// returns true only if the amount of x and o is the same
fn main() {
  assert_eq!(xo("xoxo"), true);
  assert_eq!(xo("x"), true);
  assert_eq!(xo("oooox"), false);
}