use std::fs::read_to_string;

use anyhow::Context;

use crate::RatingType::{CO2, OXYGEN};

enum RatingType {
    OXYGEN,
    CO2,
}

fn main() -> anyhow::Result<()> {
    let input = read_to_string("data/input.txt")?;
    let lines = input.lines().collect::<Vec<_>>();

    let oxygen_generator_rating = calculate(lines.clone(), OXYGEN)?;
    let co2_scrubber_rating = calculate(lines.clone(), CO2)?;

    let oxygen_int = isize::from_str_radix(oxygen_generator_rating, 2)?;
    let co2_int = isize::from_str_radix(co2_scrubber_rating, 2)?;
    dbg!(oxygen_int, co2_int, oxygen_int * co2_int);
    Ok(())
}

fn calculate(mut input: Vec<&str>, rating_type: RatingType) -> anyhow::Result<&str> {
    for i in 0.. {
        let (ones, zeroes): (Vec<_>, Vec<_>) = input
            .iter()
            .partition(|line| line.chars().nth(i) == Some('1'));
        let comparator = match rating_type {
            OXYGEN => usize::ge,
            CO2 => usize::lt,
        };
        input = if comparator(&ones.len(), &zeroes.len()) {
            ones
        } else {
            zeroes
        };
        if input.len() == 1 {
            break;
        }
    }
    Ok(input.get(0).context("no element left")?)
}
