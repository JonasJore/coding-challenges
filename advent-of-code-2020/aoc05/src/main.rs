// TODO: not done

use std::fs::{ read_to_string };


type Res<T> = ::std::result::Result<T, Box<dyn ::std::error::Error>>;

fn parse_input(input_vector: Vec<&str>) -> Vec<usize> {
    input_vector.iter()
        .map(|seat| {
            print!("{}", seat);
            seat.chars()
                .map(|c| match c {
                    'B' | 'R' => '1',
                    'F' | 'L' => '0',
                    s => unreachable!("dude where is my seat"),
                })
                .collect::<String>()
        })
        .map(|num| usize::from_str_radix(&num, 2).unwrap())
        .collect()
}

fn part1(inp: Vec<usize>) -> usize {
    inp.into_iter().max().unwrap()
}

fn main() -> Res<()> {
    let input: String = read_to_string("input/input.txt").unwrap();

    let input2 = input.split("\n").collect();

    let part_1 = parse_input(input2);

    println!("part 1: {}", part1(part_1));

    Ok(())
}
