mod parser;

use itertools::Itertools;
use parser::parse_input_grid;
use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Failed to read input file");
    let power_banks = parse_input_grid(&input);

    dbg!(power_banks.iter().map(max_jolt).sum::<u32>());
}

fn max_jolt(power_bank: &parser::PowerBank) -> u32 {
        power_bank
            .jolts
            .iter()
            .combinations(2)
            .map(|jolts| format!("{}{}", jolts.first().unwrap().0.to_string(), jolts.last().unwrap().0.to_string()).parse().unwrap())
            .max()
            .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_name() {
        let power_banks = parse_input_grid("987654321111111\n");
        let max = max_jolt(&power_banks[0]);
        assert_eq!(max, 17);
    }
}
