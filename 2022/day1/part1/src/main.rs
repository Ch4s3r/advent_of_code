use anyhow::{Context, Result};
use nom::character::complete;
use nom::character::complete::newline;
use nom::combinator::opt;
use nom::multi::{many0, separated_list0};
use nom::sequence::terminated;
use nom::IResult;

fn main() -> Result<()> {
    let input = include_str!("../data/input_test.txt");
    let numbers = parse_input(input)?.1;
    let highest_calories = numbers
        .into_iter()
        .map(|number_block| number_block.iter().fold(0, |acc, x| acc + x))
        .reduce(i32::max)
        .context("failed")?;
    println!("{}", highest_calories);
    Ok(())
}

fn parse_input(input: &str) -> IResult<&str, Vec<Vec<i32>>> {
    separated_list0(newline, parse_numbers)(input)
}

fn parse_numbers(input: &str) -> IResult<&str, Vec<i32>> {
    many0(terminated(complete::i32, opt(newline)))(input)
}
