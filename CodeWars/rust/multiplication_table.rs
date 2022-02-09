// source: https://www.codewars.com/kata/534d2f5b5371ecf8d2000a08/rust

fn multiplication_table(len: usize) -> Vec<Vec<usize>> {
  let mut table: Vec<Vec<usize>> = vec![];
  for i in 1..len + 1 {
    let mut sub_vec: Vec<usize> = vec![];
    for j in 1..len + 1 {
      sub_vec.push(i * j);
    }
    table.push(sub_vec);
  }
  return table;
}

fn print_table(table: Vec<Vec<usize>>) {
  for sub in table {
    for item in sub {
      print!("{}", item);
    }
    println!("");
  }
}

fn basic_test() {
  let subject = multiplication_table(3);
    assert_eq!(subject, [[1,2,3], [2,4,6], [3,6,9]]);
}

fn test_last_sub_vec_should_be_the_same_as_len() {
  let subject = multiplication_table(10).last().unwrap().len();
  assert_eq!(subject, 10);
}

fn main() {
  basic_test();
  test_last_sub_vec_should_be_the_same_as_len();

  let table = multiplication_table(10);
  print_table(table);
}
