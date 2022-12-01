use anyhow::{Context, Result};
use nom::character::complete;
use nom::character::complete::newline;
use nom::multi::separated_list1;
use nom::sequence::tuple;
use nom::IResult;

fn main() -> Result<()> {
    let input = include_str!("../data/input.txt");
    let numbers = dbg!(parse_input(input))?.1;
    let highest_calories = numbers
        .into_iter()
        .map(|number_block| number_block.iter().fold(0, |acc, x| acc + x))
        .reduce(i32::max)
        .context("failed")?;
    println!("{}", highest_calories);
    Ok(())
}

fn parse_input(input: &str) -> IResult<&str, Vec<Vec<i32>>> {
    separated_list1(tuple((newline, newline)), parse_numbers)(input)
}

fn parse_numbers(input: &str) -> IResult<&str, Vec<i32>> {
    separated_list1(newline, complete::i32)(input)
}
