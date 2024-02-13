use crate::util::input_utils;

fn find_surface_of_box(present: &str) -> i32 {
    // println!("present: {}", present);
    let meassures = present.split("x").collect::<Vec<&str>>();
    let l = meassures[0].parse::<i32>().unwrap();
    let w = meassures[1].parse::<i32>().unwrap();
    let h = meassures[2].parse::<i32>().unwrap();

    let surface_area = 2 * (l * w) + 2 * (w * h) + (2 * (h * l));
    let smallest_area = *vec![l * w, l * h, w * h].iter().min().unwrap();

    return surface_area + smallest_area;
}

fn aoc_02_part_1() {
    let file_content = input_utils::read_file_to_string("src/input/aoc_02.txt");
    let file: Vec<&str> = file_content.trim().split("\n").collect::<Vec<&str>>();

    let sum: i32 = file
        .iter()
        .map(|present| find_surface_of_box(present))
        .reduce(|acc, surface| acc + surface)
        .unwrap();

    println!("part 1: {}", sum);
}

fn ribbon_to_order(present: &str) -> i32 {
    let meassures = present.split("x").collect::<Vec<&str>>();
    let meassuresi32: Vec<i32> = meassures
        .iter()
        .map(|&side| side.parse::<i32>().unwrap())
        .collect();

    let mut smallest = meassuresi32[0];
    let mut next_smallest = meassuresi32[1];

    if meassuresi32[1] < meassuresi32[0] {
        smallest = meassuresi32[1];
        next_smallest = meassuresi32[0];
    }

    if meassuresi32[2] < smallest {
        next_smallest = smallest;
        smallest = meassuresi32[2];
    } else if meassuresi32[2] < next_smallest {
        next_smallest = meassuresi32[2];
    }

    // let (smallest, next_smallest) =
    //     meassuresi32
    //         .iter()
    //         .fold((i32::MAX, i32::MAX), |(sm, sm2), &val| {
    //             if val < sm {
    //                 (val, sm)
    //             } else if val < sm2 {
    //                 (sm, val)
    //             } else {
    //                 (sm, sm2)
    //             }
    //         });

    let feat = smallest * 2 + next_smallest * 2;
    let bow = meassuresi32[0] * meassuresi32[1] * meassuresi32[2];

    return feat + bow;
}

fn aoc_02_part_2() {
    let file_content = input_utils::read_file_to_string("src/input/aoc_02.txt");
    let file: Vec<&str> = file_content.trim().split("\n").collect::<Vec<&str>>();
    let sum: i32 = file.iter().map(|&present| ribbon_to_order(present)).sum();

    println!("part 2: {}", sum);
}

pub fn answer() {
    aoc_02_part_1();
    aoc_02_part_2();
}
