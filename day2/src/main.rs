use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use regex::Regex;
use std::str::FromStr;
use std::string::ParseError;
use std::time::Instant;
use lazy_static::lazy_static;

#[derive(Debug)]
struct Input {
    min: usize,
    max: usize,
    character: char,
    text: String,
}

lazy_static! {
    static ref INPUT_REGEX: Regex = Regex::new(r"(\d+)-(\d+) (\w): (\w+)").unwrap();
}

impl FromStr for Input {
    type Err = ParseError;


    fn from_str(text: &str) -> Result<Self, Self::Err> {
        let cap = INPUT_REGEX.captures(text).unwrap();
        Ok(
            Input {
                min: cap[1].parse().unwrap(),
                max: cap[2].parse().unwrap(),
                character: cap[3].parse().unwrap(),
                text: cap[4].parse().unwrap(),
            }
        )
    }
}


fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("data/input.txt")?;
    let reader = BufReader::new(file);

    let mut inputs: Vec<Input> = Vec::new();

    for line in reader.lines().map(|x| x.unwrap()) {
        inputs.push(Input::from_str(&line)?);
    }

    println!("size: {}", &inputs.len());


    let start = Instant::now();

    let mut count = 0;
    for input in &inputs {
        let b1 = input.text.chars().nth(input.min - 1).unwrap_or(' ') == input.character;
        let b2 = input.text.chars().nth(input.max - 1).unwrap_or(' ') == input.character;

        if b1 ^ b2 {
            count += 1;
        }
    }

    let duration = start.elapsed();
    println!("duration: {:?}", duration);
    println!("count: {}", count);

    Ok(())
}
