use std::io::{ self, Read };
use regex::Regex;
use std::convert::TryFrom;

type Res<T> = ::std::result::Result<T, Box<dyn ::std::error::Error>>;

struct PasswordLine {
    min: u32,
    max: u32,
    character: char,
    password: String,
}

impl PasswordLine {
    fn valid_part2(&self) -> bool {
        let min = self.password.as_bytes()[usize::try_from(self.min - 1).unwrap()] as char;
        let max = self.password.as_bytes()[usize::try_from(self.max - 1).unwrap()] as char;
        return min == self.character && max != self.character || min != self.character && max == self.character;
    }
}

fn parse_file(input: &str) -> Vec<&str> {
    input.lines().collect()
}

fn main() -> Res<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    
    let vector: Vec<&str> = parse_file(&input);
    part1(&vector)?;
    part2(vector.clone())?;

    Ok(())
}

fn parse_line(line: &str) -> PasswordLine {
    let regexp: Regex = Regex::new(r"(\d+)-(\d+) (\w+): (.*)").unwrap();
    let reg_matches = regexp.captures(line).unwrap();
    return PasswordLine {
        min: reg_matches.get(1).unwrap().as_str().parse().unwrap(),
        max: reg_matches.get(2).unwrap().as_str().parse().unwrap(),
        character:  reg_matches.get(3).unwrap().as_str().parse().unwrap(),
        password: reg_matches.get(4).unwrap().as_str().parse().unwrap(),
    };
}

fn count_occurance(character: char, string: &str) -> u32 {
    let mut count = 0;
    for i   in string.chars() {
        if i == character {
            count +=1;
        }
    }

    return count;
}

fn part1(vector: &Vec<&str>) -> Res<()> {
    let mut valid = 0;
    for line in vector {
        let parsed_password = parse_line(line);
        let count = count_occurance(parsed_password.character, &parsed_password.password);
        if count >= parsed_password.min && count <= parsed_password.max {
            valid += 1;
        }
    }

    println!("part1: valid passwords: {} ", valid);

    Ok(())
}

fn part2(vector: Vec<&str>) -> Res<()> {
    let filtered_vec: Vec<&str> = vector
        .into_iter()
        .filter(|&line| { parse_line(line).valid_part2() })
        .collect();

    println!("part2: valid passwords: {}", filtered_vec.len());
    Ok(())
}