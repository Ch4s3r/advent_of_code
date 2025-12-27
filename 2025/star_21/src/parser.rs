use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, line_ending, space1},
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
    Parser,
};
use std::collections::HashMap;

pub type Graph = HashMap<String, Vec<String>>;

fn parse_line(input: &str) -> IResult<&str, (String, Vec<String>)> {
    let (input, (node, neighbors)) = separated_pair(
        alpha1,
        tag(": "),
        separated_list1(space1, alpha1),
    ).parse(input)?;
    
    let neighbors = neighbors.into_iter().map(|s: &str| s.to_string()).collect();
    Ok((input, (node.to_string(), neighbors)))
}

pub fn parse_input(input: &str) -> Graph {
    let (_input, lines) = separated_list1(line_ending, parse_line).parse(input).expect("Failed to parse input");
    lines.into_iter().collect()
}
