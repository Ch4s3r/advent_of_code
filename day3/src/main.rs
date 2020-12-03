use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use regex::Regex;
use std::str::FromStr;
use std::string::ParseError;
use std::time::Instant;
use lazy_static::lazy_static;

#[derive(Debug)]
struct Map {
    x: usize,
    y: usize,
    width: usize,
    height: usize,
}

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("data/input.txt")?;
    let reader = BufReader::new(file);

    let mut forest_map = Vec::new();

    for line in reader.lines().map(|x| x.unwrap()) {
        forest_map.push(line.chars().collect::<Vec<char>>());
    }

    let mut pos = Map { x: 0, y: 0, width: forest_map[0].len(), height: forest_map.len() };

    let mut count = 0;

    loop {
        pos.x += 1;
        pos.y = (pos.y + 3) % pos.width;
        if pos.x == pos.height {
            break;
        }
        if forest_map[pos.x][pos.y] == '#' {
            count += 1;
        }
    }
    dbg!(count);

    Ok(())
}
