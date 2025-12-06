mod parser;

use parser::parse_input;
use std::fs;

use crate::parser::Operation::{Add, Multiply};

fn main() {
    // Load the input file
    let input_content =
        fs::read_to_string("input.txt").expect("Failed to read input.txt");

    // Parse the input
    let (_, parsed) = parse_input(&input_content).expect("Failed to parse input");

    // Print the parsed data
    dbg!(&parsed);
    dbg!(
        parsed
            .verticals
            .iter()
            .map(|vertical| {
                match vertical.operation {
                    Add => vertical.numbers.iter().fold(0, |acc, &num| acc + num),
                    Multiply => vertical.numbers.iter().fold(1, |acc, &num| acc * num),
                }
            })
            .sum::<u64>()
    );
}
