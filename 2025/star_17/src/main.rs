mod parser;

use itertools::Itertools;
use parser::{Point, parse_points};
use std::{
    cmp::{max, min},
    fs,
};

fn main() {
    let input = fs::read_to_string("input.txt").expect("Failed to read input file");
    let points: Vec<Point> = parse_points(&input).expect("Failed to parse points").1;

    points
        .iter()
        .combinations(2)
        .map(|pair| {
            let p1 = &pair[0];
            let p2 = &pair[1];
            let x: u64 = (max(p1.x, p2.x) - min(p1.x, p2.x) + 1).into();
            let y: u64 = (max(p1.y, p2.y) - min(p1.y, p2.y) + 1).into();
            x * y
        })
        .max()
        .map(|area| {
            println!("Max area: {}", area);
        });
}
