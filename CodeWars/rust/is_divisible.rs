// https://www.codewars.com/kata/5545f109004975ea66000086/rust

fn is_divisible(n: i32, x: i32, y: i32) -> bool {
  (n % x, n % y) == (0, 0)
}