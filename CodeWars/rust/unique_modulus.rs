// takes in a vector of `i32`, mods all numbers and counts number of unique values..

use std::collections::HashSet;

fn unique_modulus_solution(vector: &Vec<i32>) -> bool {
  let unique_modulus_numbers: HashSet<i32> = vector.iter()
    .map(|item| {
      return *item % 42;
    })
    .collect::<HashSet<i32>>();

  println!("unique modulus-numbers: {}", unique_modulus_numbers.len());

  vector.len() == unique_modulus_numbers.len()
}


fn main() {
  let vec_of_ints: Vec<i32> = vec!(1,2,3,4,5,6,1,2,3);
  unique_modulus_solution(&vec_of_ints);
}

#[test]
fn test_unique_number_of_mods_is_correct() {
  let under_test = vec!(1,2,3,1,2,3);
  assert_eq!(unique_modulus_solution(under_test, false);
}

#[test]
fn test_number_of_unique_numbers_is_the_same_as_input() {
  let under_test = vec!(1,2,3);
  assert_eq!(unique_modulus_solution(under_test, true);
}