use std::fs::{read_to_string};
use anyhow::Context;

fn main() -> anyhow::Result<()> {
    let input = read_to_string("data/input.txt")?;
    let lines = input.lines().collect::<Vec<_>>();

    let mut oxygen_input = lines.clone();
    let mut count = 0;
    while oxygen_input.len() > 1 {
        let (ones, zeroes): (Vec<_>, Vec<_>) = oxygen_input.iter().partition(|line| {
            line.chars().nth(count) == Some('1')
        });
        count += 1;
        oxygen_input = if ones.len() >= zeroes.len() { ones } else { zeroes };
    }
    let oxygen_generator_rating = oxygen_input.get(0).context("no element left for oxygen")?;

    let mut co2_scrubber_input = lines;
    let mut count = 0;
    while co2_scrubber_input.len() > 1 {
        let (ones, zeroes): (Vec<_>, Vec<_>) = co2_scrubber_input.iter().partition(|line| {
            line.chars().nth(count) == Some('1')
        });
        count += 1;
        co2_scrubber_input = if ones.len() < zeroes.len() { ones } else { zeroes };
    }
    let co2_scrubber_rating = co2_scrubber_input.get(0).context("no element left for co2")?;

    let oxygen_int = isize::from_str_radix(oxygen_generator_rating, 2).unwrap();
    let co2_int = isize::from_str_radix(co2_scrubber_rating, 2).unwrap();
    dbg!(oxygen_int, co2_int, oxygen_int * co2_int);
    Ok(())
}
