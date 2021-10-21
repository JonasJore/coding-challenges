fn sequence_sum(start: i32, end: i32, step: usize) -> i32 {
  if start > end {
    return 0;
  }
  
  let mut sum: i32 = 0;

  for number in (start..end).step_by(step) {
    sum += number;
  }
  println!("sum: {}", sum);
  return sum;
}

fn main() {
  sequence_sum(1, 16, 2);
}

#[test]
fn test_sums_sequence_correctly_when_given_step_to_increment_by() {
  assert_eq!(sequence_sum(1, 16, 2), 64);
}

#[test]
fn test_start_greater_than_end() {
  assert_eq!(sequence_sum(3, 1, 1), 0);
}