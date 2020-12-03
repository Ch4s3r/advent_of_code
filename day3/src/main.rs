#![feature(iterator_fold_self)]

use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
struct Point {
    x: usize,
    y: usize,
}

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("data/input.txt")?;
    let reader = BufReader::new(file);

    let mut forest_map = Vec::new();

    for line in reader.lines().map(|x| x.unwrap()) {
        forest_map.push(line.chars().collect::<Vec<char>>());
    }


    let steps = vec![(1, 1), (1, 3), (1, 5), (1, 7), (2, 1)];
    let mut counts = Vec::new();
    for (step_x, step_y) in steps {
        let mut pos = Point { x: 0, y: 0 };
        let mut count = 0u64;
        let width = forest_map[0].len();
        let height = forest_map.len();
        loop {
            pos.x += step_x;
            pos.y = (pos.y + step_y) % width;
            if pos.x >= height {
                break;
            }
            if forest_map[pos.x][pos.y] == '#' {
                count += 1;
            }
        }
        counts.push(count);
    }

    dbg!(counts.iter().fold(1, |acc, x| acc * x));
    Ok(())
}
