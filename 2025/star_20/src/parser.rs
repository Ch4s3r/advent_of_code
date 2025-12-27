use nom::{
    bytes::complete::take_while1,
    character::complete::{char, digit1, space1},
    multi::separated_list1,
    sequence::delimited,
    IResult, Parser,
};
use std::fs;

#[derive(Debug, Clone)]
pub struct Machine {
    pub button_schematics: Vec<Vec<u32>>,
    pub joltage_requirements: Vec<u32>,
}

// Parse light diagram [...] and skip it
fn skip_light_diagram(input: &str) -> IResult<&str, ()> {
    let (input, _) = delimited(
        char('['),
        take_while1(|c: char| c != ']'),
        char(']'),
    )
    .parse(input)?;
    Ok((input, ()))
}

// Parse a single number
fn parse_number(input: &str) -> IResult<&str, u32> {
    digit1.map(|s: &str| s.parse::<u32>().unwrap()).parse(input)
}

// Parse comma-separated numbers
fn parse_number_list(input: &str) -> IResult<&str, Vec<u32>> {
    separated_list1(char(','), parse_number).parse(input)
}

// Parse button schematic (...)
fn parse_button_schematic(input: &str) -> IResult<&str, Vec<u32>> {
    delimited(char('('), parse_number_list, char(')')).parse(input)
}

// Parse button schematics (one or more)
fn parse_button_schematics(input: &str) -> IResult<&str, Vec<Vec<u32>>> {
    let (input, first) = parse_button_schematic(input)?;
    let mut schematics = vec![first];
    let mut remaining = input;

    while let Ok((next_remaining, schematic)) = parse_button_schematic(remaining.trim_start()) {
        schematics.push(schematic);
        remaining = next_remaining;
    }

    Ok((remaining, schematics))
}

// Parse joltage requirements {...}
fn parse_joltage(input: &str) -> IResult<&str, Vec<u32>> {
    delimited(char('{'), parse_number_list, char('}')).parse(input)
}

// Parse a complete line
pub fn parse_line(input: &str) -> IResult<&str, Machine> {
    let input = input.trim();
    let (input, _) = skip_light_diagram(input)?;
    let (input, _) = space1(input)?;
    let (input, button_schematics) = parse_button_schematics(input)?;
    let (input, _) = space1(input)?;
    let (input, joltage_requirements) = parse_joltage(input)?;

    Ok((
        input,
        Machine {
            button_schematics,
            joltage_requirements,
        },
    ))
}

// Parse all machines from a file
pub fn parse_all(file_path: &str) -> Vec<Machine> {
    let contents = fs::read_to_string(file_path).expect("Failed to read input file");
    
    contents
        .lines()
        .filter(|line| !line.trim().is_empty())
        .filter_map(|line| parse_line(line).ok().map(|(_, machine)| machine))
        .collect()
}
