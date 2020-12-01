use std::io::{ self, Read };

type Resulted<T> = ::std::result::Result<T, Box<dyn ::std::error::Error>>;

fn main() -> Resulted<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    for line in input.lines() {
        println!("{}", &line);
    }

    Ok(())
}

fn part1(input: &str) -> Resulted<()> {Ok(())}
fn part2(input: &str) -> Resulted<()> {Ok(())}


