mod aoc2;
mod util;
use std::fs;

fn aoc_01_part_1() {
    let file = fs::read_to_string("src/input/aoc_01.txt").expect("error");
    let mut floor: i32 = 0;
    file.trim().split("").into_iter().for_each(|item| {
        // if item == "(" {
        //     floor += 1;
        // } else if item == ")" {
        //     floor -= 1;
        // }
        match item {
            "(" => floor += 1,
            ")" => floor -= 1,
            _ => {}
        };
    });

    println!("{}", floor);
}

fn aoc_01_part_2() {
    let file = fs::read_to_string("src/input/aoc_01.txt").expect("error");
    let mut floor: i32 = 0;
    for (index, item) in file.trim().split("").into_iter().enumerate() {
        match item {
            "(" => floor += 1,
            ")" => floor -= 1,
            _ => {}
        };
        if floor == -1 {
            println!("went into -1 at {}", index);
            break;
        }
    }
}

fn main() {
    aoc_01_part_1();
    aoc_01_part_2();

    aoc2::answer();
}
