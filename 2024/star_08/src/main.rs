use aoc_parse::{parser, prelude::*};
use polars::export::num::ToPrimitive;
use polars::prelude::*;
use std::fs;

fn get_field(input: &Vec<Vec<char>>, x: i64, y: i64) -> char {
    let Some(x) = x.to_usize() else { return '.' };
    let Some(y) = y.to_usize() else { return '.' };
    let Some(y) = input.get(y) else { return '.' };
    let Some(x) = y.get(x) else { return '.' };
    x.to_owned()
}

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Should have been able to read the file");

    let parser = parser!(lines(repeat_sep(any_char, "")));
    let input = parser.parse(&contents).unwrap();

    let height = input.len().to_i64().unwrap();
    let width = input[0].len().to_i64().unwrap();
    let mut score = 0;

    for y in 0..height {
        for x in 0..width {
            if get_field(&input, x, y) == 'A' {
                // M.S
                // .A.
                // M.S
                if get_field(&input, x - 1, y - 1) == 'M'
                    && get_field(&input, x + 1, y - 1) == 'S'
                    && get_field(&input, x - 1, y + 1) == 'M'
                    && get_field(&input, x + 1, y + 1) == 'S'
                {
                    score += 1;
                }

                // S.S
                // .A.
                // M.M
                if get_field(&input, x - 1, y - 1) == 'S'
                    && get_field(&input, x + 1, y - 1) == 'S'
                    && get_field(&input, x - 1, y + 1) == 'M'
                    && get_field(&input, x + 1, y + 1) == 'M'
                {
                    score += 1;
                }

                // S.M
                // .A.
                // S.M
                if get_field(&input, x - 1, y - 1) == 'S'
                    && get_field(&input, x + 1, y - 1) == 'M'
                    && get_field(&input, x - 1, y + 1) == 'S'
                    && get_field(&input, x + 1, y + 1) == 'M'
                {
                    score += 1;
                }

                // M.M
                // .A.
                // S.S
                if get_field(&input, x - 1, y - 1) == 'M'
                    && get_field(&input, x + 1, y - 1) == 'M'
                    && get_field(&input, x - 1, y + 1) == 'S'
                    && get_field(&input, x + 1, y + 1) == 'S'
                {
                    score += 1;
                }
            }
        }
    }
    println!("{:?}", score);
}
// x   x   x
//   x x x
// x x x x x
//   x x x
// x   x   x
