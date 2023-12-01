use std::fs;

fn aoc_01() {
    let file = fs::read_to_string("src/input_01.txt").expect("error");
    let contents = file
        .as_str()
        .lines()
        .map(|x| x.to_string())
        .map(|string| {
            string
                .chars()
                .filter(|&char| char.is_ascii_digit() || char.is_whitespace())
                .collect()
        })
        .collect::<Vec<String>>();

    let answer_aoc1: i32 = contents
        .into_iter()
        .map(|string: String| {
            format!(
                "{}{}",
                string.chars().next().unwrap(),
                string.chars().last().unwrap()
            )
        })
        .map(|number_string| number_string.parse::<i32>().unwrap())
        .collect::<Vec<i32>>()
        .iter()
        .sum();

    println!("sum: {}", answer_aoc1);
}

fn main() {
    println!("Hello, world!");
    aoc_01();
}
