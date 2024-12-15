use std::collections::HashMap;
use aoc_parse::{parser, prelude::*};
use std::fs;
use std::ops::Sub;

fn main() {
    let contents =
        fs::read_to_string("input.txt").expect("Should have been able to read the file");

    let parser = parser!(lines(u64 "   " u64));
    let input = parser.parse(&contents).unwrap();
    let (mut left, mut right): (Vec<_>, Vec<_>) = input.iter().cloned().unzip();
    let right_counted = right.into_iter().fold(HashMap::new(), |mut acc, c| {
        *acc.entry(c).or_insert(0) += 1;
        acc
    });
    let diffs: Vec<_> = left.into_iter().map(|x| x * right_counted.get(&x).unwrap_or(&0u64)).collect();
    let total: u64 = diffs.into_iter().sum();
    println!("{:?}", total)
}
