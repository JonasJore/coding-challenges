use crate::util::input_utils;

fn find_surface_of_box(present: &str) -> i32 {
    println!("present: {}", present);
    let meassures = present.split("x").collect::<Vec<&str>>();
    let l = meassures[0].parse::<i32>().unwrap();
    let w = meassures[1].parse::<i32>().unwrap();
    let h = meassures[2].parse::<i32>().unwrap();

    let surface_area = 2 * (l * w) + 2 * (w * h) + (2 * (h * l));
    let smallest_area = *vec![l * w, l * h, w * h].iter().min().unwrap();

    return surface_area + smallest_area;
}

pub fn aoc_02_part_1() {
    let file_content = input_utils::read_file_to_string("src/input/aoc_02.txt");
    let file: Vec<&str> = file_content.trim().split("\n").collect::<Vec<&str>>();

    let s: i32 = file
        .iter()
        .map(|present| find_surface_of_box(present))
        .reduce(|acc, surface| acc + surface)
        .unwrap();

    println!("sum: {}", s);
}
