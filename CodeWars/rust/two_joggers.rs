// https://www.codewars.com/kata/5274d9d3ebc3030802000165
fn nbr_of_laps (x: u16, y: u16) -> (u16, u16) {
  let mut laps_ran: u16 = x;

  while laps_ran % y != 0 {
    laps_ran += x;
  }

  return (laps_ran / x, laps_ran / y);
}

fn main() {
  println!("{:?}", nbr_of_laps(5, 3));
  println!("{:?}", nbr_of_laps(4, 6));
  println!("{:?}", nbr_of_laps(5, 5));
}
