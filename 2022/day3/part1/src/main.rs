use anyhow::Result;
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
    let sum = 0;
    Ok(sum)
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
