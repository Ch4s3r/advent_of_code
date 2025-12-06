mod parser;

use parser::parse_input_grid;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Failed to read input file");
    let power_banks = parse_input_grid(&input);

    dbg!(power_banks.par_iter().map(max_jolt).sum::<u64>());
}

fn max_jolt(power_bank: &parser::PowerBank) -> u64 {
    let digits: Vec<u32> = power_bank.jolts.iter().map(|j| j.0).collect();
    let target_length = 12;
    let mut to_remove = digits.len() - target_length;
    let mut stack: Vec<u32> = Vec::new();
    
    for &digit in &digits {
        // Remove smaller digits from stack if we still have removals left
        // and the current digit is larger than the top of stack
        while !stack.is_empty() && to_remove > 0 && stack[stack.len() - 1] < digit {
            stack.pop();
            to_remove -= 1;
        }
        stack.push(digit);
    }
    
    // If we still need to remove digits, remove from the end
    while to_remove > 0 {
        stack.pop();
        to_remove -= 1;
    }
    
    // Convert stack to number
    let mut result = 0u64;
    for &digit in &stack {
        result = result * 10 + digit as u64;
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_987654321111111() {
        let power_banks = parse_input_grid("987654321111111\n");
        let max = max_jolt(&power_banks[0]);
        assert_eq!(max, 987654321111);
    }

    #[test]
    fn test_81111111111111911119111111111111911111111() {
        let power_banks = parse_input_grid("811111111111119111191111111111119111111112\n");
        let max = max_jolt(&power_banks[0]);
        assert_eq!(max, 999111111112);
    }

    #[test]
    fn test_234234234234278() {
        let power_banks = parse_input_grid("234234234234278\n");
        let max = max_jolt(&power_banks[0]);
        assert_eq!(max, 434234234278);
    }
}
