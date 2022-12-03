use anyhow::{Context, Result};
use itertools::Itertools;
use nom::character::complete::{alphanumeric1, newline};
use nom::multi::separated_list1;
use nom::IResult;

fn main() -> Result<()> {
    let input = include_str!("../data/input.txt");
    app(input)?;
    Ok(())
}

fn app(input: &str) -> Result<i32> {
    let backpacks = parse_input(input).unwrap().1;
    let chars = dbg!(backpacks
        .iter()
        .map(|backpack| -> Result<String> {
            let char = backpack
                .left_compartment
                .chars()
                .filter(|char| backpack.right_compartment.chars().contains(char))
                .collect_vec();
            Ok(char.first().context("No element found")?.to_string())
        })
        .collect::<Result<String>>()?);
    let sum = chars.chars().map(|char| to_pritory(char)).sum();
    dbg!(Ok(sum))
}

fn to_pritory(value: char) -> i32 {
    if value.is_uppercase() {
        value as i32 - 38
    } else {
        value as i32 - 96
    }
}

#[derive(Debug)]
struct Backpack {
    left_compartment: String,
    right_compartment: String,
}

fn parse_input(input: &str) -> IResult<&str, Vec<Backpack>> {
    let (input, lines) = separated_list1(newline, alphanumeric1)(input)?;
    let backpacks = lines
        .iter()
        .map(|line| {
            let (left, right) = line.split_at(line.len() / 2);
            Backpack {
                left_compartment: left.to_string(),
                right_compartment: right.to_string(),
            }
        })
        .collect_vec();
    Ok((input, backpacks))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() -> Result<()> {
        let input = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";
        let actual = app(input)?;
        assert_eq!(157, actual);
        Ok(())
    }
}
