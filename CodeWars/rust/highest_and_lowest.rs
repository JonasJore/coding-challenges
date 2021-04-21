// https://www.codewars.com/kata/554b4ac871d6813a03000035/rust
fn high_and_low(numbers: &str) -> String {
  let split: Vec<&str> = numbers.split(' ').collect();
  let mut highest = 0;
  let mut lowest  = 0;
  
  for i in &split {
      let current = i.to_string().parse::<i32>().unwrap();
      
      if current > highest || highest == 0 {
          highest = current;
      }
      
      if current < lowest || lowest == 0 {
          lowest = current;
      }
  }
  
  return format!("{} {}", highest, lowest);
}

#[test]
fn sample_test() {
assert_eq!("42 -9", high_and_low("8 3 -5 42 -1 0 0 -9 4 7 4 -4"));
}

#[test]
fn should_find_highest_and_lowest() {
  assert_eq!("542 -214", high_and_low("4 5 29 54 4 0 -214 542 -64 1 -3 6 -6"));
}

#[test]
fn double_negative_numbers_test() {
  assert_eq!("-1 -1", high_and_low("-1 -1"));
}
