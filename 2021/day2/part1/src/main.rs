use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

use anyhow::Context;
use strum_macros::{EnumString, EnumVariantNames};

use Direction::{DOWN, UP};

use crate::Direction::FORWARD;

#[derive(Debug)]
struct Input {
    direction: Direction,
    length: u32,
}

#[derive(Debug, PartialEq, EnumString, EnumVariantNames)]
#[strum(serialize_all = "lowercase")]
enum Direction {
    FORWARD,
    DOWN,
    UP,
}


fn parse_line(input: &str) -> anyhow::Result<Input> {
    let (direction, length) = input.split_once(" ").context("failed to split")?;
    Ok(Input { direction: Direction::from_str(direction)?, length: length.parse()? })
}


fn main() -> anyhow::Result<()> {
    let file = File::open("data/input.txt")?;
    let reader = BufReader::new(file);

    let mut horizontal: u32 = 0;
    let mut depth: u32 = 0;

    reader
        .lines()
        .filter_map(|result| { result.ok() })
        .filter_map(|line| { parse_line(line.as_str()).ok() })
        .for_each(|input| {
            match input.direction {
                FORWARD => { horizontal += input.length }
                DOWN => { depth += input.length }
                UP => { depth -= input.length }
            }
        });

    dbg!(horizontal, depth, horizontal * depth);
    Ok(())
}
