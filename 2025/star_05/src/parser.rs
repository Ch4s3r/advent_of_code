use nom::IResult;
use nom::bytes::complete::tag;
use nom::character::complete::{digit1, newline};
use nom::sequence::terminated;
use nom::multi::many1;

/// Represents jolts (voltage) for a power bank
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Jolt(pub u32);

/// Represents a power bank with its jolt values
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct PowerBank {
    pub jolts: Vec<Jolt>,
}

impl PowerBank {
    /// Create a new power bank from a vector of jolt values
    pub fn new(jolt_values: Vec<u32>) -> Self {
        PowerBank {
            jolts: jolt_values.into_iter().map(Jolt).collect(),
        }
    }
}

/// Parse a single line of digits
fn parse_line(input: &str) -> IResult<&str, Vec<u32>> {
    let (input, digits) = digit1(input)?;
    let numbers: Vec<u32> = digits
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect();
    Ok((input, numbers))
}

/// Parse the entire input file and return power banks
pub fn parse_input_grid(input: &str) -> Vec<PowerBank> {
    match many1(terminated(parse_line, newline))(input) {
        Ok((_, data)) => data.into_iter().map(PowerBank::new).collect(),
        Err(_) => Vec::new(),
    }
}

/// Parse the entire input file
pub fn parse_input(input: &str) -> IResult<&str, Vec<Vec<u32>>> {
    many1(terminated(parse_line, newline))(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_line() {
        let input = "123456789\n";
        let (remaining, result) = parse_line("123456789").unwrap();
        assert_eq!(result, vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
        assert_eq!(remaining, "");
    }

    #[test]
    fn test_parse_input() {
        let input = "987654321\n811111111\n";
        let (remaining, result) = parse_input(input).unwrap();
        assert_eq!(result.len(), 2);
        assert_eq!(result[0], vec![9, 8, 7, 6, 5, 4, 3, 2, 1]);
        assert_eq!(result[1], vec![8, 1, 1, 1, 1, 1, 1, 1, 1]);
    }
}
