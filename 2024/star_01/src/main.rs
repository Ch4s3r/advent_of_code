use aoc_parse::{parser, prelude::*};
use std::fs;
use std::ops::Sub;

fn main() {
    let contents =
        fs::read_to_string("input.txt").expect("Should have been able to read the file");

    let parser = parser!(lines(u64 "   " u64));
    let input = parser.parse(&contents).unwrap();
    let (mut left, mut right): (Vec<_>, Vec<_>) = input.iter().cloned().unzip();
    left.sort();
    right.sort();
    let difference: Vec<_> = left.into_iter().zip(right).collect();
    let diff: Vec<_> = difference
        .into_iter()
        .map(|(x, y): (u64, u64)| x.abs_diff(y))
        .collect();
    let total: u64= diff.iter().sum();
    println!("{:?}", total)
}
