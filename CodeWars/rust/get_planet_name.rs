// https://www.codewars.com/kata/515e188a311df01cba000003/rust

fn get_planet_name(id: u32) -> String {
  let planet: String = match id {
      1 => "Mercury".to_string(),
      2 => "Venus".to_string(),
      3 => "Earth".to_string(),
      4 => "Mars".to_string(),
      5 => "Jupiter".to_string(),
      6 => "Saturn".to_string(),
      7 => "Uranus".to_string(),
      8 => "Neptune".to_string(),
      _ => unreachable!(),
  };
  return planet;
}

fn main() {
  let planet = get_planet_name(1);
  println!("The name of this planet is {}", planet);
  
  tests();
}

fn tests() {
  assert_eq!("Mercury", get_planet_name(1));
  assert_eq!("Venus", get_planet_name(2));
  assert_eq!("Earth", get_planet_name(3));
  assert_eq!("Mars", get_planet_name(4));
  assert_eq!("Jupiter", get_planet_name(5));
  assert_eq!("Saturn", get_planet_name(6));
  assert_eq!("Uranus", get_planet_name(7));
  assert_eq!("Neptune", get_planet_name(8));
}