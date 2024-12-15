#![feature(iter_map_windows)]

use std::fmt::format;
use regex::Regex;
use std::fs;

fn main() {
    let contents =
        fs::read_to_string("input.txt").expect("Should have been able to read the file");

    let regex_mul = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let regex_dont = Regex::new(r"don't\(\).*?do\(\)").unwrap();
    let content_without_donts = regex_dont.replace_all(&contents, "");
    println!("{}", &content_without_donts);
    println!(
        "{}",
        regex_mul
            .captures_iter(&content_without_donts)
            .map(|cap| cap[1].parse::<u64>().unwrap_or(0) * cap[2].parse::<u64>().unwrap_or(0))
            .sum::<u64>()
    );
}
