use std::fs;

#[derive(Debug)]
struct Navigateur {
    direction: String,
    distance: u32,
}

const FORWARD: &str = "forward";
const DOWN: &str = "down";
const UP: &str = "up";

fn read_file() -> Vec<Navigateur> {
    let file_lines = fs::read_to_string("src/input.txt").expect("failed to read");

    let vec: Vec<Navigateur> = file_lines
        .lines()
        .map(|line| {
            let mut split = line.split(" ");
            let (first, second) = (split.next().unwrap(), split.next().unwrap());

            Navigateur {
                direction: first.to_string(),
                distance: second.parse::<u32>().unwrap(),
            }
        })
        .collect();

    vec
}

fn solution_part_1(vector: &[Navigateur]) {
    let mut depth: u32 = 0;
    let mut distance: u32 = 0;
    
    vector.iter().for_each(|navi| {
        match navi.direction.as_ref() {
            FORWARD => distance += navi.distance,
            DOWN => depth += navi.distance,
            UP => depth -= navi.distance,
            _ => print!("unknown input"),
        }
    });
    println!("Final position of sub: {}", distance * depth);
}

fn solution_part_2(vector: &[Navigateur]) {
    println!("{}", "TODO");
}

fn main() {
    let input = read_file();
    solution_part_1(&input);
    solution_part_2(&input);
}
