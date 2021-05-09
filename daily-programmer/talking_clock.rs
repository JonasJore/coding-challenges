// https://www.reddit.com/r/dailyprogrammer/comments/6jr76h/20170627_challenge_321_easy_talking_clock/
use std::collections::HashMap;

fn main() {
    let hours_map: HashMap<&str, &str> = [
        ("01", "one"),
        ("02", "two"),
        ("03", "three"),
        ("04", "four"),
        ("05", "five"),
        ("06", "six"),
        ("07", "seven"),
        ("08", "eight"),
        ("09", "nine"),
        ("10", "ten"),
        ("11", "eleven"),
        ("12", "twelve"),
    ]
    .iter()
    .cloned()
    .collect();

    let minutes_map: HashMap<&str, &str> = [
        ("00", ""),
        ("01", "one"),
        ("02", "two"),
        ("03", "three"),
        ("04", "four"),
        ("05", "five"),
        ("06", "six"),
        ("07", "seven"),
        ("08", "eight"),
        ("09", "nine"),
        ("10", "ten"),
        ("11", "eleven"),
        ("12", "twelve"),
        ("13", "thirteen"),
        ("14", "fourteen"),
        ("15", "fifteen"),
        ("16", "sixteen"),
        ("17", "seventeen"),
        ("18", "eighteen"),
        ("19", "nineteen"),
    ]
    .iter()
    .cloned()
    .collect();

    let minute_tens_map: HashMap<&str, &str> = [
        ("1", "ten"),
        ("2", "twenty"),
        ("3", "thirty"),
        ("4", "forty"),
        ("5", "fifty"),
    ]
    .iter()
    .cloned()
    .collect();

    let input = "09:00";

    let mut split_input: Vec<&str> = input.split(':').collect();
    let hours_to_words = match split_input[0].parse::<i32>().unwrap() {
        hour if hour < 10 => hours_map[&format!("0{}", hour).to_string().as_str()],
        hour if hour <= 24 && hour > 9 => {
            hours_map[&format!("0{}", (hour - 12)).to_string().as_str()]
        }
        _ => "not valid hour",
    };

    let minute_vect: Vec<char> = split_input[1].chars().collect();

    let minute_ten_indicator = minute_vect[0].to_digit(10).unwrap();

    let tens = match minute_ten_indicator {
        ten if ten > 0 => minute_tens_map[ten.to_string().as_str()],
        _ => "not valid",
    };

    let minute_to_text = minutes_map[split_input[1]];

    println!("{}", minute_to_text);
    let am_pm = match split_input[0].parse::<i32>().unwrap() {
        h if h > 12 => "pm",
        _ => "am",
    };

    let mut timestamp: String = String::new();

    let minute_indicator = split_input[1].parse::<i32>().unwrap();

    // happycase covered!
    if minute_to_text == "" {
        timestamp = format!("It's {} {}", hours_to_words, am_pm);
    }

    println!("{}", timestamp);
}
