// https://www.reddit.com/r/dailyprogrammer/comments/6jr76h/20170627_challenge_321_easy_talking_clock/
use std::collections::HashMap;

struct Clock<'a> {
    hours: &'a str,
    minutes: &'a str,
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
    ]
    .iter()
    .cloned()
    .collect();

    let minutes_map: HashMap<&str, &str> = [
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

    let input = "00:00";

    let input_split: Vec<&str> = input.split(':').collect();

    let clock = Clock {
        hours: &input_split[0],
        minutes: &input_split[1],
    };

    let hours_as_int = clock.hours.parse::<i32>().unwrap();

    let parsed_hours = match hours_as_int {
        hour if hour > 12 => Some(hours_map[&format!("0{}", (hour - 12)).to_string().as_str()]),
        hour if hour == 0 => Some(hours_map[(hour + 12).to_string().as_str()]),
        _ => None,
    };

    // hele klokkeslett er første case
    // deretter klokkeslett som It's one thirty am
    // også dette it's twelve thirty four am (kombinerer både tier og enkeltminutt)

    let am_pm = if hours_as_int < 12 { "am" } else { "pm" };

    // happycase covered!
    println!("It's {} {}", parsed_hours.unwrap(), am_pm);
}
