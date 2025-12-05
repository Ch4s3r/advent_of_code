use nom::{
    bytes::complete::{tag, take_while1},
    IResult,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Range {
    start: u64,
    end: u64,
}

fn parse_number(input: &str) -> IResult<&str, u64> {
    let (input, num_str) = take_while1(|c: char| c.is_ascii_digit())(input)?;
    Ok((input, num_str.parse().unwrap()))
}

fn parse_range(input: &str) -> IResult<&str, Range> {
    let (input, start) = parse_number(input)?;
    let (input, _) = tag("-")(input)?;
    let (input, end) = parse_number(input)?;
    Ok((input, Range { start, end }))
}

fn parse_ranges(input: &str) -> Vec<Range> {
    let lines: Vec<&str> = input.trim().lines().collect();
    let mut result = Vec::new();

    for line in lines {
        let ranges: Vec<Range> = line
            .split(',')
            .filter_map(|s| parse_range(s.trim()).ok().map(|(_, r)| r))
            .collect();
        result.extend(ranges);
    }

    result
}

fn main() {
    let input = include_str!("../input.txt");
    let ranges = parse_ranges(input);
    let mut count = 0;

    for range in &ranges {
        println!("Range: {}-{}", range.start, range.end);
        for id in range.start..=range.end {
            let id_str = id.to_string();
            if check_valid_id(&id_str) == false {
                println!("Invalid ID: {}", id_str);
                count += id;
            }
        }
    }
    println!("Total count: {}", count);
}

fn check_valid_id(id: &str) -> bool {
    let len = id.len();
    if len % 2 != 0 {
        return true;
    }
    let chars = id.chars().collect::<Vec<char>>();
    for i in 0..len / 2 {
        if chars[i] != chars[i + len / 2] {
            return true;
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn invalid_ids() {
        assert_eq!(check_valid_id("11"), false);
        assert_eq!(check_valid_id("22"), false);
        assert_eq!(check_valid_id("99"), false);
        assert_eq!(check_valid_id("1010"), false);
        assert_eq!(check_valid_id("1188511885"), false);
        assert_eq!(check_valid_id("222222"), false);
        assert_eq!(check_valid_id("446446"), false);
        assert_eq!(check_valid_id("38593859"), false);
        assert_eq!(check_valid_id("12"), true);
        assert_eq!(check_valid_id("1122"), true);
        assert_eq!(check_valid_id("95"), true);
        assert_eq!(check_valid_id("1188511880"), true);
        assert_eq!(check_valid_id("1698522"), true);
    }
}
