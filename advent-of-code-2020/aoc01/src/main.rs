use std::io::{ self, Read };

type Res<T> = ::std::result::Result<T, Box<dyn ::std::error::Error>>;

fn parse_file(input: &str) -> Result<Vec<u32>, ::std::num::ParseIntError> {
    input.lines()
        .flat_map(|line| line.split_whitespace())
        .map(|number| {
            number.parse::<u32>()
        }).collect()
}

fn main() -> Res<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let vector: Vec<u32> = parse_file(&input).unwrap();

    println!("answer: part 1: {}", part1(&vector));
    println!("answer: part 2: {}", part2(&vector));

    Ok(())
}

fn part1(vec: &Vec<u32>) -> u32 {
    for i in 0..vec.len() {
        for j in i + 1..vec.len() {
            if vec[i] + vec[j] == 2020 {
                return vec[i] * vec[j];
            }
        }
    }
    0
}

fn part2(vec: &Vec<u32>) -> u32 {
    for i in 0..vec.len() {
        for j in i + 1..vec.len() {
            for k in j+1..vec.len() {
                if vec[i] + vec[j] + vec[k] == 2020 {
                    return vec[i] * vec[j] * vec[k];
                }
            }
        }
    }
    0
}


