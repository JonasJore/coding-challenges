use std::io::{ self, Read };
use std::collections::HashSet;
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
    let mut seen: HashSet<u32> = HashSet::new();
    let mut answer = 0;
    for i in 0..vec.len() {
        seen.insert(vec[i]);
        if seen.contains(&(2020 - vec[i])) {
            answer = vec[i] * (2020 - vec[i]);
        }
    }
    answer
}

fn part2(vec: &Vec<u32>) -> u32 {
    for i in 0..vec.len()-1 {
        for j in i..vec.len()-2 {
            for k in j+1..vec.len()-3 {
                if vec[i] + vec[j] + vec[k] == 2020 {
                    return vec[i] * vec[j] * vec[k];
                }
            }
        }
    }
    return 0;
}


