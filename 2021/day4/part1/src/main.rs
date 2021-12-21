use std::collections::{BTreeMap, HashMap};
use std::fs::{read_to_string, File};
use std::ptr::hash;
use std::str::FromStr;

use nom::bytes::complete::tag;
use nom::character::complete::{digit1, newline};
use nom::combinator::opt;
use nom::multi::{many0, many1};
use nom::sequence::{delimited, terminated, tuple};
use nom::{IResult, Parser};

fn main() -> anyhow::Result<()> {
    let input = read_to_string("data/input.txt")?;
    // todo fix this unwrap
    let (input, parsed_input) = parse_input(&input).ok().unwrap();
    dbg!(parsed_input);
    Ok(())
}

fn parse_input(input: &str) -> IResult<&str, ParsedInput> {
    let (input, chosen_numbers) =
        terminated(many1(terminated(digit1, tag(","))), opt(newline))(input)?;
    let (input, bingo_boards) = many1(terminated(parse_bingo_board, opt(newline)))(input)?;

    let chosen_numbers = chosen_numbers
        .into_iter()
        .map(|digit| u64::from_str(digit).unwrap())
        .collect();
    dbg!(&chosen_numbers);

    let parsed_input = ParsedInput {
        chosen_numbers,
        bingo_boards,
    };
    Ok((input, parsed_input))
}

fn parse_bingo_board(input: &str) -> IResult<&str, BingoBoard> {
    dbg!(input);
    todo!()
}

#[derive(Debug, PartialEq)]
pub struct ParsedInput {
    chosen_numbers: Vec<u64>,
    bingo_boards: Vec<BingoBoard>,
}

#[derive(Debug, PartialEq)]
pub struct BingoBoard {
    rows: Vec<Vec<u64>>,
}
