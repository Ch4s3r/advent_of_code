use aoc_parse::{parser, prelude::*};
use polars::prelude::*;
use std::collections::HashSet;
use std::fs;

fn main() {
    let contents =
        fs::read_to_string("input.txt").expect("Should have been able to read the file");
    let split: Vec<&str> = contents.split("\n\n").collect();
    let parser_rules = parser!(
        lines(u64 "|" u64)
    );
    let parser_ordering = parser!(lines(repeat_sep(u64, ",")));
    let input_rules = parser_rules.parse(split[0]).unwrap();
    let input_ordering = parser_ordering.parse(split[1]).unwrap();
    // dbg!(&input_ordering);

    let score: u64 = input_ordering
        .iter()
        .map(|ordering| {
            dbg!(ordering);
            let ordering_correct = ordering.into_iter().enumerate().all(|(index, page)| {
                let not_before_current = input_rules
                    .iter()
                    .filter_map(|(key, value)| if key == page { Some(value) } else { None })
                    .collect::<HashSet<_>>();
                let pages_before: HashSet<_> = HashSet::from_iter(&ordering[0..index]);
                // dbg!(&not_before_current);
                // dbg!(&pages_before);
                let correct = pages_before
                    .intersection(&not_before_current)
                    .collect::<HashSet<_>>()
                    .is_empty();
                // dbg!(correct)
                correct
            });
            dbg!(ordering_correct);
            if ordering_correct {
                let median = ordering.len() / 2;
                let median_element = ordering[median];
                median_element
            } else {
                0
            }
        })
        .sum();
    dbg!(score);
}
