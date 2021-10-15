// https://www.codewars.com/kata/5168bb5dfe9a00b126000018/train/rust

fn solution(phrase: &str) -> String {
    if phrase.len() <= 1 {
        return phrase.to_string();
    }

    return phrase.chars().rev().collect::<String>();
}

fn main() {
    let solution = solution("dev: jonas");
    println!("solution: {}", solution);
}

#[test]
fn test_reversed_hello_world() {
    let under_test: String = solution("hello world");

    assert_eq!(under_test, "dlrow olleh");
}

#[test]
fn test_reversing_len_less_than_2() {
    let under_test: String = solution("j");

    assert_eq!(under_test, "j");
}
