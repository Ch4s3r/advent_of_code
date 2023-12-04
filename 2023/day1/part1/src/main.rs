use anyhow::{Context, Result};

fn recover_calibration_number(input: &str) -> Result<i32> {
    let input_lines = input.split_ascii_whitespace();
    Ok(input_lines
        .map(|line| {
            let number_string = dbg!(line.chars().filter(|x| x.is_numeric()).collect::<String>());
            let two_digit_string = number_string
                .chars()
                .nth(0)
                .context("failed to get 0")?
                .to_string()
                + &number_string
                    .chars()
                    .nth_back(0)
                    .context("failed to get 0")?
                    .to_string();
            Ok(dbg!(two_digit_string.parse::<i32>()?))
        })
        .collect::<Result<Vec<_>>>()?
        .iter()
        .sum::<i32>())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() -> Result<()> {
        let input: &str = include_str!("../input.txt");
        assert_eq!(recover_calibration_number(input)?, 142);
        Ok(())
    }

    #[test]
    fn real() -> Result<()> {
        let input: &str = include_str!("../input_real.txt");
        assert_eq!(recover_calibration_number(input)?, 55538);
        Ok(())
    }
}
