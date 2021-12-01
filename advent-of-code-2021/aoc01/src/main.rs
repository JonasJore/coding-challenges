use std::cmp::PartialOrd;
use std::fs;

fn read_file_into_vector() -> Vec<i32> {
    let file_content = fs::read_to_string("src/input.txt").expect("failed to read");

    let vector: Vec<i32> = file_content
        .lines()
        .map(|line| line.parse::<i32>().unwrap())
        .collect();
    return vector;
}

fn count_increased_depths<N: PartialOrd>(vec: &[N]) -> i32 {
    let mut count_of_increases = 0;
    for (index, current) in vec.iter().enumerate() {
        if index > 0 {
            if current > &vec[index - 1] {
                count_of_increases += 1;
            } else {
            }
        } else {
        }
    }
    return count_of_increases;
}

fn solution_part_1(vec: &[i32]) {
    let count_of_increases = count_increased_depths(&vec);
    println!("Part 1: {} increases", count_of_increases);
}

fn solution_part_2(vec: &[i32]) {
    let length_3_meassurements: Vec<i32> =
        vec.windows(3).map(|numbers| numbers.iter().sum()).collect();
    let count_of_increases = count_increased_depths(&length_3_meassurements);
    println!("Part 2: {} increases", count_of_increases);
}

fn main() {
    let input: Vec<i32> = read_file_into_vector();
    solution_part_1(&input);
    solution_part_2(&input);
}

#[test]
fn will_count_increases_in_depth() {
    let data = vec![1, 2, 3, 4];
    let under_test = count_increased_depths(&data);
    assert_eq!(under_test, 3);
}

// is uphill ocean a realistic test case?
#[test]
fn count_increases_when_only_decreasing() {
    let data = vec![4, 3, 2, 1];
    let under_test = count_increased_depths(&data);
    assert_eq!(under_test, 0);
}

#[test]
fn count_increases_big_numbers() {
    let data: Vec<i128> = vec![
        4345345245654654654654546446665465461,
        333333333333453453453453454345345,
        233453454354534545675676575676575675,
        99992349999567657567676577576765777791,
    ];
    let under_test = count_increased_depths(&data);
    assert_eq!(under_test, 2);
}
