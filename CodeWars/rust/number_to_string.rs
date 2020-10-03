fn number_to_string(i: i32) -> String {
  i.to_string()
}

fn main() {
  assert_eq!(number_to_string(67), "67");
  assert_eq!(number_to_string(1+2), "3");
}