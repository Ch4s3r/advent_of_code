use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use regex::Regex;
use std::str::FromStr;
use std::string::ParseError;
use std::time::Instant;
use lazy_static::lazy_static;
use std::borrow::Borrow;

#[derive(Debug)]
struct Input {
    min: u8,
    max: u8,
    character: char,
    text: String,
}

lazy_static! {
    static ref inputRegex: Regex = Regex::new(r"(\d+)-(\d+) (\w): (\w+)").unwrap();
}

// impl FromStr for Input<'_> {
//     type Err = ParseError;
//
//
//     fn from_str(text: &str) -> Result<Self, Self::Err> {
//         let cap = inputRegex.captures(text)?;
//         Ok(
//             Input {
//                 min: cap[1].parse::<u8>()?,
//                 max: cap[2].parse::<u8>()?,
//                 character: cap[3].parse()?,
//                 text: &cap[4].to_string(),
//             }
//         )
//     }
// }


fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("data/input.txt")?;
    let reader = BufReader::new(file);

    let mut inputs: Vec<Input> = Vec::new();

    for line in reader.lines().map(|x| x.unwrap()) {
        let cap = inputRegex.captures(&line).unwrap();
        let input = Input {
            min: cap[1].parse()?,
            max: cap[2].parse()?,
            character: cap[3].parse()?,
            text: cap[4].parse()?,
        };
        inputs.push(input);
    }

    println!("size: {}", &inputs.len());


    let start = Instant::now();

    let mut count = 0;
    for input in &inputs {
        let mut char_count = 0;
        for char in input.text.chars() {
            if char == input.character {
                char_count += 1;
                // println!("{}", char_count);
            }
        }
        if (input.min..=input.max).contains(&char_count) {
            count += 1;
        }
    }

    let duration = start.elapsed();
    println!("duration: {:?}", duration);
    println!("count: {}", count);

    Ok(())
}
