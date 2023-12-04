use anyhow::{Context, Result};

fn recover_calibration_number(input: &str) -> Result<i32> {
    let input_lines = input.split_ascii_whitespace().collect::<Vec<_>>();
    let string_to_number_mappings = vec![
        ("1", "1"),
        ("2", "2"),
        ("3", "3"),
        ("4", "4"),
        ("5", "5"),
        ("6", "6"),
        ("7", "7"),
        ("8", "8"),
        ("9", "9"),
        ("one", "1"),
        ("two", "2"),
        ("three", "3"),
        ("four", "4"),
        ("five", "5"),
        ("six", "6"),
        ("seven", "7"),
        ("eight", "8"),
        ("nine", "9"),
    ];
    input_lines
        .iter()
        .map(|line| {
            let ranked_mapping = string_to_number_mappings
                .iter()
                .map(|mapping| (line.find(mapping.0), mapping.1))
                .collect::<Vec<_>>();
            let first = ranked_mapping
                .iter()
                .filter(|x| x.0.is_some())
                .min_by_key(|x| x.0)
                .context("min not found")?;

            let line_reverse = &line.chars().rev().collect::<String>();
            let ranked_mapping_reverse = string_to_number_mappings
                .iter()
                .map(|mapping| {
                    (
                        line_reverse.find(&mapping.0.chars().rev().collect::<String>()),
                        mapping.1,
                    )
                })
                .collect::<Vec<_>>();
            let last = ranked_mapping_reverse
                .iter()
                .filter(|x| x.0.is_some())
                .min_by_key(|x| x.0)
                .context("min not found")?;

            // dbg!(&ranked_mapping, &ranked_mapping_reverse);
            // dbg!(&first, &last);
            Ok((first.1.to_string() + last.1).parse::<i32>()?)
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() -> Result<()> {
        let input: &str = include_str!("../input.txt");
        assert_eq!(recover_calibration_number(input)?, 281);
        Ok(())
    }

    #[test]
    fn real() -> Result<()> {
        let input: &str = include_str!("../input_real.txt");
        assert_eq!(recover_calibration_number(input)?, 54875);
        Ok(())
    }

    #[test]
    fn edgecase() -> Result<()> {
        let input: &str = "twoeighthree";
        assert_eq!(recover_calibration_number(input)?, 23);
        Ok(())
    }
}
