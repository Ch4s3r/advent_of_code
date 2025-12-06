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

    // Find the maximum line length
    let max_len = number_lines.iter().map(|line| line.len()).max().unwrap_or(0);

    // Pad all lines to the same length
    let padded_lines: Vec<String> = number_lines
        .iter()
        .map(|line| {
            let mut s = line.to_string();
            while s.len() < max_len {
                s.push(' ');
            }
            s
        })
        .collect();

    // Find operation positions in the operations line
    let mut op_positions = Vec::new();
    for (i, ch) in operations_line.chars().enumerate() {
        if ch == '+' || ch == '*' {
            op_positions.push((i, if ch == '+' { Operation::Add } else { Operation::Multiply }));
        }
    }

    if op_positions.is_empty() {
        return Err(nom::Err::Error(nom::error::Error::new(
            input,
            nom::error::ErrorKind::Eof,
        )));
    }

    // Determine column ranges for each operation
    // Each operation covers columns from its position up to the next operation position (or end)
    let mut col_ranges = Vec::new();
    for i in 0..op_positions.len() {
        let start = op_positions[i].0;
        let end = if i + 1 < op_positions.len() {
            op_positions[i + 1].0 - 1
        } else {
            max_len - 1
        };
        col_ranges.push((start, end, op_positions[i].1));
    }

    // For each column range, read vertically and create numbers
    let mut verticals = Vec::new();

    for (start_col, end_col, op) in col_ranges {
        let mut numbers = Vec::new();
        
        // Read each column in this range from top to bottom
        for col in start_col..=end_col {
            let mut column_number_str = String::new();
            
            // Read this column from top to bottom
            for line in &padded_lines {
                let chars: Vec<char> = line.chars().collect();
                if col < chars.len() && chars[col].is_numeric() {
                    column_number_str.push(chars[col]);
                }
            }

            // Parse the collected digits into a number
            if !column_number_str.is_empty() {
                if let Ok(num) = column_number_str.parse::<u64>() {
                    numbers.push(num);
                }
            }
        }

        if !numbers.is_empty() {
            verticals.push(Vertical {
                numbers,
                operation: op,
            });
        }
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
