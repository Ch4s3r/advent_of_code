#![feature(iter_map_windows)]

use aoc_parse::{parser, prelude::*};
use std::collections::HashMap;
use std::fs;
use std::ops::Sub;

fn main() {
    let contents =
        fs::read_to_string("input.txt").expect("Should have been able to read the file");

    let parser = parser!(lines(repeat_sep(u64, " ")));
    let input = parser.parse(&contents).unwrap();
    let mut total = 0;
    for report in input {
        let sorted_asc = report.iter().is_sorted_by(|a, b| a < b);
        let sorted_desc = report.iter().is_sorted_by(|a, b| a > b);
        let diff = report
            .iter()
            .map_windows(|&[&x, &y]| x.abs_diff(y))
            .collect::<Vec<_>>();
        let between = diff.iter().all(|x| (&1u64..=&3u64).contains(&x));
        let safe = (sorted_asc | sorted_desc) && between;
        if safe {
            total += 1;
        }
    }
    println!("{}", total);
}
