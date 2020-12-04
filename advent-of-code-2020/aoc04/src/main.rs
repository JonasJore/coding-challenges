use std::io::{ self, Read };
use regex::Regex;

type Res<T> = ::std::result::Result<T, Box<dyn ::std::error::Error>>;

fn parse_file(input: &str) -> Vec<&str> {
    input.lines().collect()
}

fn main() -> Res<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;



    Ok(())
}