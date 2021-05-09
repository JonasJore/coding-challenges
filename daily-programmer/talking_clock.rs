// https://www.reddit.com/r/dailyprogrammer/comments/6jr76h/20170627_challenge_321_easy_talking_clock/

use std::collections::HashMap;

struct Clock<'a> {
    hours: &'a str,
    minutes: &'a str,
}

fn get_first_digit(digit: i32) -> i32 {
    let mut copy: i32 = digit;
    if copy < 10 {
        return copy;
    }
    loop {
        if copy < 9 {
            break;
        }
        copy /= 10;
    }
    copy
}

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
        ("00", "twelve"),
    ]
    .iter()
    .cloned()
    .collect();

    let minutes_map: HashMap<&str, &str> = [
        ("00", "zero"),
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

    // let input = "00:00";
    // let input = "01:30";
    // let input = "12:05";
    // let input = "14:01";
    // let input = "20:29";
    let input = "21:00";

    let input_split: Vec<&str> = input.split(':').collect();

    let clock = Clock {
        hours: &input_split[0],
        minutes: &input_split[1],
    };

    let hours_as_int = clock.hours.parse::<i32>().unwrap();
    println!("{} hours as int", hours_as_int);
    let parsed_hours = match hours_as_int {
        hour if hour >= 12 => Some(hours_map[&format!("0{}", (hour - 12)).to_string().as_str()]),
        hour if hour == 0 => Some(hours_map[(hour + 12).to_string().as_str()]),
        hour if hour < 10 => Some(hours_map[&format!("0{}", hour).to_string().as_str()]),
        _ => None,
    };

    let minutes_as_int = clock.minutes.parse::<i32>().unwrap();
    let last_digit = minutes_as_int % 10;
    let first_digit = get_first_digit(minutes_as_int);
    let mut parsed_minute_tens = "";

    if first_digit > 1 && minutes_as_int > 9 {
        parsed_minute_tens = minute_tens_map[format!("{}", first_digit).as_str()];
    }

    let mut parsed_minute_ = "";
    parsed_minute_ = minutes_map[format!("0{}", last_digit).as_str()];

    let am_pm = if hours_as_int < 12 { "am" } else { "pm" };

    // driver-code
    if minutes_as_int == 0 || parsed_minute_ == "zero" {
        if parsed_minute_tens != "" {
            println!(
                "It's {} {} {}",
                parsed_hours.unwrap(),
                parsed_minute_tens,
                am_pm
            );
        } else {
            println!("It's {} {}", parsed_hours.unwrap(), am_pm);
        }
    } else if parsed_minute_tens != "" && minutes_as_int > 9 {
        println!(
            "It's {} {} {} {}",
            parsed_hours.unwrap(),
            parsed_minute_tens,
            parsed_minute_,
            am_pm
        );
    } else if minutes_as_int < 10 {
        println!(
            "It's {} oh {} {}",
            parsed_hours.unwrap(),
            parsed_minute_,
            am_pm
        );
    }
}
