use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{digit1, newline, space0},
    combinator::{map, opt},
    multi::separated_list1,
    sequence::{separated_pair, terminated},
    IResult, Parser,
};

#[derive(Debug, Clone)]
pub struct Point {
    pub x: u32,
    pub y: u32,
}

fn parse_number(input: &str) -> IResult<&str, u32> {
    digit1
        .map(|s: &str| s.parse::<u32>().unwrap())
        .parse(input)
}

fn parse_point(input: &str) -> IResult<&str, Point> {
    let (input, (x, y)) = separated_pair(parse_number, tag(","), parse_number).parse(input)?;
    Ok((input, Point { x, y }))
}

pub fn parse_points(input: &str) -> IResult<&str, Vec<Point>> {
    let (input, points) = separated_list1(newline, parse_point).parse(input)?;
    let (input, _) = opt(newline).parse(input)?;
    Ok((input, points))
}
