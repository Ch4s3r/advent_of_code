use nom::{
    branch::alt,
    character::complete::{char, digit1, multispace0, multispace1},
    multi::separated_list1,
    IResult, Parser,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Operation {
    Add,
    Multiply,
}

#[derive(Debug, Clone)]
pub struct Vertical {
    pub numbers: Vec<u64>,
    pub operation: Operation,
}

#[derive(Debug)]
pub struct Input {
    pub verticals: Vec<Vertical>,
}

/// Parse a single operation (+, *)
fn parse_operation(input: &str) -> IResult<&str, Operation> {
    alt((
        char('+').map(|_| Operation::Add),
        char('*').map(|_| Operation::Multiply),
    ))
    .parse(input)
}

/// Parse a number
fn parse_number(input: &str) -> IResult<&str, u64> {
    digit1
        .map(|s: &str| s.parse::<u64>().unwrap())
        .parse(input)
}

/// Parse a vertical line (numbers separated by spaces)
fn parse_vertical_numbers(input: &str) -> IResult<&str, Vec<u64>> {
    let (input, _) = multispace0(input)?;
    separated_list1(multispace1, parse_number).parse(input)
}

/// Parse the entire input
pub fn parse_input(input: &str) -> IResult<&str, Input> {
    let lines: Vec<&str> = input.lines().collect();

    if lines.len() < 2 {
        return Err(nom::Err::Error(nom::error::Error::new(
            input,
            nom::error::ErrorKind::Eof,
        )));
    }

    // Parse numbers from all but the last line
    let number_lines = &lines[..lines.len() - 1];
    let operations_line = lines[lines.len() - 1];

    // Parse the numbers from each line
    let mut number_matrix: Vec<Vec<u64>> = Vec::new();
    for line in number_lines {
        if line.trim().is_empty() {
            continue;
        }
        let (_, numbers) = parse_vertical_numbers(line)?;
        number_matrix.push(numbers);
    }

    if number_matrix.is_empty() {
        return Err(nom::Err::Error(nom::error::Error::new(
            input,
            nom::error::ErrorKind::Eof,
        )));
    }

    // Parse operations from the last line
    let (_, operations) = separated_list1(multispace1, parse_operation)
        .parse(operations_line)?;

    // Get the number of columns
    let num_columns = number_matrix[0].len();

    if operations.len() != num_columns {
        return Err(nom::Err::Error(nom::error::Error::new(
            input,
            nom::error::ErrorKind::TooLarge,
        )));
    }

    // Build verticals from the matrix
    let mut verticals = Vec::new();
    for col in 0..num_columns {
        let mut column_numbers = Vec::new();
        for row in 0..number_matrix.len() {
            column_numbers.push(number_matrix[row][col]);
        }
        verticals.push(Vertical {
            numbers: column_numbers,
            operation: operations[col],
        });
    }

    Ok((input, Input { verticals }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_operation() {
        assert_eq!(parse_operation("+"), Ok(("", Operation::Add)));
        assert_eq!(parse_operation("*"), Ok(("", Operation::Multiply)));
    }

    #[test]
    fn test_parse_number() {
        assert_eq!(parse_number("123"), Ok(("", 123u64)));
        assert_eq!(parse_number("42"), Ok(("", 42u64)));
    }
}
