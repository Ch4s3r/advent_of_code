use std::collections::HashMap;
use std::fmt::{Debug, Formatter};
use std::fs::read_to_string;

use nom::bytes::complete::tag;
use nom::character::complete::{line_ending, space0, u64 as parse_u64};
use nom::combinator::opt;
use nom::multi::{many0, many1};
use nom::sequence::{delimited, terminated};
use nom::IResult;

fn main() -> anyhow::Result<()> {
    let input = read_to_string("data/input.txt")?;
    // todo fix this unwrap
    let (_input, parsed_input) = parse_input(&input).unwrap();
    dbg!(&parsed_input);

    parsed_input
        .chosen_numbers
        .iter()
        .for_each(|chosen_number| {
            parsed_input
                .bingo_boards
                .iter()
                .for_each(|bingo_board| bingo_board.number_checklist)
        });

    Ok(())
}

fn parse_input(input: &str) -> IResult<&str, ParsedInput> {
    let (input, chosen_numbers) = chosen_numbers(input)?;
    let (input, bingo_boards) = bingo_boards(input)?;
    let parsed_input = ParsedInput {
        chosen_numbers,
        bingo_boards,
    };
    dbg!(&parsed_input);
    Ok((input, parsed_input))
}

fn chosen_numbers(input: &str) -> IResult<&str, Vec<u64>> {
    terminated(
        many1(terminated(parse_u64, opt(tag(",")))),
        many0(line_ending),
    )(input)
}

fn bingo_boards(input: &str) -> IResult<&str, Vec<BingoBoard>> {
    many1(terminated(bingo_board, many0(line_ending)))(input)
}

fn bingo_board(input: &str) -> IResult<&str, BingoBoard> {
    let (input, rows) = many1(terminated(
        many1(delimited(space0, parse_u64, space0)),
        opt(line_ending),
    ))(input)?;

    let mut number_checklist: HashMap<String, Vec<u64>> = HashMap::new();
    rows.iter().enumerate().for_each(|(index_row, row)| {
        row.iter().enumerate().for_each(|(index_col, number)| {
            vec![("row", index_row), ("col", index_col)]
                .iter()
                .for_each(|(key, index)| {
                    number_checklist
                        .entry(key.to_string() + &index.to_string())
                        .or_insert_with(Vec::new)
                        .push(*number);
                })
        })
    });

    Ok((
        input,
        BingoBoard {
            rows,
            number_checklist,
        },
    ))
}

#[derive(Debug, PartialEq)]
pub struct ParsedInput {
    chosen_numbers: Vec<u64>,
    bingo_boards: Vec<BingoBoard>,
}

#[derive(PartialEq)]
pub struct BingoBoard {
    rows: Vec<Vec<u64>>,
    number_checklist: HashMap<String, Vec<u64>>,
}

impl Debug for BingoBoard {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        let text = self
            .rows
            .iter()
            .map(|row| {
                row.iter()
                    .map(|number| number.to_string() + "\t")
                    .collect::<String>()
                    + "\r\n"
            })
            .collect::<String>();
        write!(
            formatter,
            "----BingoBoard----
{}
------------------
",
            &text.trim()
        )
    }
}
