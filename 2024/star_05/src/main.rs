#![feature(iter_map_windows)]

use regex::Regex;
use std::fs;

fn main() {
    let contents =
        fs::read_to_string("input.txt").expect("Should have been able to read the file");

    let regex_mul = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    println!(
        "{}",
        regex_mul
            .captures_iter(&contents)
            .map(|cap| cap[1].parse::<u64>().unwrap_or(0) * cap[2].parse::<u64>().unwrap_or(0))
            .sum::<u64>()
    );
}
