use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

use anyhow::Context;
use strum_macros::{EnumString, EnumVariantNames};

use Direction::{DOWN, UP, FORWARD};

#[derive(Debug)]
struct Input {
    direction: Direction,
    units: u32,
}

#[derive(Debug, PartialEq, EnumString, EnumVariantNames)]
#[strum(serialize_all = "lowercase")]
enum Direction {
    FORWARD,
    DOWN,
    UP,
}

fn parse_line(input: &str) -> anyhow::Result<Input> {
    let (direction, units) = input.split_once(" ").context("failed to split")?;
    Ok(Input { direction: Direction::from_str(direction)?, units: units.parse()? })
}

fn main() -> anyhow::Result<()> {
    let file = File::open("data/input.txt")?;
    let reader = BufReader::new(file);

    let mut horizontal: u32 = 0;
    let mut depth: u32 = 0;
    let mut aim: u32 = 0;

    reader
        .lines()
        .filter_map(|result| { result.ok() })
        .filter_map(|line| { parse_line(line.as_str()).ok() })
        .for_each(|input| {
            match input.direction {
                FORWARD => {
                    horizontal += input.units;
                    depth += aim * input.units;
                }
                DOWN => { aim += input.units; }
                UP => { aim -= input.units; }
            }
        });

    dbg!(horizontal, depth, horizontal * depth);
    Ok(())
}
