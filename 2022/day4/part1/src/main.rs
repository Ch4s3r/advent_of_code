use std::ops::RangeInclusive;

use anyhow::Result;
use itertools::Itertools;
use nom::bytes::complete::tag;
use nom::character::complete;
use nom::character::complete::newline;
use nom::multi::separated_list1;
use nom::sequence::tuple;
use nom::IResult;

fn main() -> Result<()> {
    let input = include_str!("../data/input.txt");
    app(input)?;
    Ok(())
}

fn app(input: &str) -> Result<i32> {
    let elfgroups = parse_input(input).unwrap().1;
    let overlapping_groups_count = elfgroups
        .iter()
        .filter(|elfgroup| elfgroup.assignments_overlap())
        .count();
    dbg!(Ok(overlapping_groups_count as i32))
}

#[derive(Debug)]
struct ElfGroup {
    first_assignment: RangeInclusive<i32>,
    second_assignment: RangeInclusive<i32>,
}

impl ElfGroup {
    fn assignments_overlap(&self) -> bool {
        let first = dbg!(self.first_assignment.clone().collect_vec());
        let second = self.second_assignment.clone().collect_vec();
        let (bigger, smaller) = if first.len() > second.len() {
            (first, second)
        } else {
            (second, first)
        };
        smaller.iter().all(|number| bigger.contains(number))
    }
}

fn parse_input(input: &str) -> IResult<&str, Vec<ElfGroup>> {
    let (input, elfgroups) = dbg!(separated_list1(newline, elf_group)(input)?);
    Ok((input, elfgroups))
}

fn elf_group(input: &str) -> IResult<&str, ElfGroup> {
    let (input, (first, _, second)) = tuple((int_range, tag(","), int_range))(input)?;
    Ok((
        input,
        ElfGroup {
            first_assignment: first,
            second_assignment: second,
        },
    ))
}

fn int_range(input: &str) -> IResult<&str, RangeInclusive<i32>> {
    let (input, (first, _, second)) = tuple((complete::i32, tag("-"), complete::i32))(input)?;
    Ok((input, first..=second))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() -> Result<()> {
        let input = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";
        let actual = app(input)?;
        assert_eq!(2, actual);
        Ok(())
    }
}
