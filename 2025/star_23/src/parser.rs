use nom::{
    branch::alt,
    bytes::complete::take_while1,
    character::complete::{char, digit1, line_ending, space0},
    combinator::{eof, map, peek},
    multi::{count, many0, many_till, separated_list1},
    sequence::{preceded, terminated},
    IResult, Parser,
};

#[derive(Debug, Clone)]
pub struct Shape {
    pub id: usize,
    pub grid: Vec<String>,
}

#[derive(Debug)]
pub struct Container {
    pub width: usize,
    pub height: usize,
    pub shapes: Vec<usize>,
}

#[derive(Debug)]
pub struct Input {
    pub shapes: Vec<Shape>,
    pub containers: Vec<Container>,
}

// Parse a complete shape with its ID and 3x3 grid
fn parse_shape(input: &str) -> IResult<&str, Shape> {
    let (input, id) = terminated(digit1, char(':')).parse(input)?;
    let id: usize = id.parse().unwrap();
    let (input, _) = line_ending.parse(input)?;

    let (input, grid) = count(
        terminated(
            map(
                take_while1(|c: char| c == '#' || c == '.'),
                |s: &str| s.to_string(),
            ),
            line_ending,
        ),
        3,
    )
    .parse(input)?;

    Ok((input, Shape { id, grid }))
}

// Parse dimension line (e.g., "4x4")
fn parse_dimension(input: &str) -> IResult<&str, (usize, usize)> {
    let (input, (width, height)) = (terminated(digit1, char('x')), digit1).parse(input)?;
    let width: usize = width.parse().unwrap();
    let height: usize = height.parse().unwrap();
    Ok((input, (width, height)))
}

// Parse shape sequence (e.g., "0 0 0 0 2 0")
fn parse_shape_sequence(input: &str) -> IResult<&str, Vec<usize>> {
    separated_list1(
        char(' '),
        map(digit1, |s: &str| s.parse::<usize>().unwrap()),
    )
    .parse(input)
}

// Parse a container specification (e.g., "4x4: 0 0 0 0 2 0")
fn parse_container(input: &str) -> IResult<&str, Container> {
    let (input, (width, height)) = parse_dimension(input)?;
    let (input, _) = terminated(char(':'), space0).parse(input)?;
    let (input, shapes) = parse_shape_sequence(input)?;
    let (input, _) = space0.parse(input)?;
    let (input, _) = alt((line_ending, eof)).parse(input)?;

    Ok((
        input,
        Container {
            width,
            height,
            shapes,
        },
    ))
}

// Parse entire input file
fn parse_all(input: &str) -> IResult<&str, Input> {
    let (input, (shapes, _)) = many_till(
        preceded(many0(line_ending), parse_shape),
        peek(preceded(many0(line_ending), (digit1, char('x')))),
    )
    .parse(input)?;

    let (input, _) = many0(line_ending).parse(input)?;
    let (input, containers) = many0(parse_container).parse(input)?;

    Ok((input, Input { shapes, containers }))
}

pub fn parse_input(input: &str) -> Result<Input, String> {
    match parse_all(input) {
        Ok((_, parsed)) => Ok(parsed),
        Err(e) => Err(format!("Parse error: {:?}", e)),
    }
}
