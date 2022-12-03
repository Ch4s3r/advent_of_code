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
    let group_backpacks = parse_input(input).unwrap().1?;
    let chars = group_backpacks
        .iter()
        .map(|group_backpack| -> Result<char> { group_backpack.get_common_item() })
        .collect::<Result<Vec<char>>>()?;
    let sum = chars.iter().map(|char| to_pritory(char.to_owned())).sum();
    dbg!(Ok(sum))
}

impl GroupBackpack {
    pub fn get_common_item(&self) -> Result<char> {
        Ok(self
            .first
            .chars()
            .filter(|char| self.second.contains(*char) && self.third.contains(*char))
            .collect_vec()
            .first()
            .context("No items match in all groups")?
            .to_owned())
    }
}

fn to_pritory(value: char) -> i32 {
    if value.is_uppercase() {
        value as i32 - 38
    } else {
        value as i32 - 96
    }
}

#[derive(Debug)]
struct GroupBackpack {
    first: String,
    second: String,
    third: String,
}

fn parse_input(input: &str) -> IResult<&str, Result<Vec<GroupBackpack>>> {
    dbg!(input);
    let (input, lines) = dbg!(separated_list1(newline, alphanumeric1)(input))?;

    let grouped_lines = dbg!(lines
        .chunks(3)
        .map(|group| -> Result<GroupBackpack> {
            let (first, second, third): (&&str, &&str, &&str) = group
                .iter()
                .collect_tuple()
                .context("Failed to collect tuple of 3 groups")?;
            Ok(GroupBackpack {
                first: first.to_string(),
                second: second.to_string(),
                third: third.to_string(),
            })
        })
        .collect());
    Ok((input, grouped_lines))
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
        assert_eq!(70, actual);
        Ok(())
    }
}
