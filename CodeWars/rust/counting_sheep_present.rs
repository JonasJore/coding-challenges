fn count_sheep(sheep: &[bool]) -> u8 {
  let mut sheep_count: u8 = 0;
  for &i in sheep {
    if i {
      sheep_count += 1;
    }
  }
  
  sheep_count
}
  
fn main() {
    assert_eq!(count_sheep(&[false]), 0);
    assert_eq!(count_sheep(&[true]), 1);
    assert_eq!(count_sheep(&[true, false]), 1);
}