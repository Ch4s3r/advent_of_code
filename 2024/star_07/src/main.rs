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
    let contents =
        fs::read_to_string("input.txt").expect("Should have been able to read the file");

    let parser = parser!(lines(repeat_sep(any_char, "")));
    let input = parser.parse(&contents).unwrap();

    let height = input.len().to_i64().unwrap();
    let width = input[0].len().to_i64().unwrap();
    let mut score = 0;

    for y in 0..height {
        for x in 0..width {
            if get_field(&input, x, y) == 'X' {
                // ?   ?   ?
                //   ? ? ?
                // x x x x x
                //   x x x
                // x   x   x
                if get_field(&input, x - 1, y - 1) == 'M'
                    && get_field(&input, x - 2, y - 2) == 'A'
                    && get_field(&input, x - 3, y - 3) == 'S'
                {
                    score += 1;
                }
                if get_field(&input, x, y - 1) == 'M'
                    && get_field(&input, x, y - 2) == 'A'
                    && get_field(&input, x, y - 3) == 'S'
                {
                    score += 1;
                }
                if get_field(&input, x + 1, y - 1) == 'M'
                    && get_field(&input, x + 2, y - 2) == 'A'
                    && get_field(&input, x + 3, y - 3) == 'S'
                {
                    score += 1;
                }

                // x   x   x
                //   x x x
                // ? ? X ? ?
                //   x x x
                // x   x   x
                if get_field(&input, x - 1, y) == 'M'
                    && get_field(&input, x - 2, y) == 'A'
                    && get_field(&input, x - 3, y) == 'S'
                {
                    score += 1;
                }
                if get_field(&input, x + 1, y) == 'M'
                    && get_field(&input, x + 2, y) == 'A'
                    && get_field(&input, x + 3, y) == 'S'
                {
                    score += 1;
                }

                // x   x   x
                //   x x x
                // x x x x x
                //   ? ? ?
                // ?   ?   ?
                if get_field(&input, x - 1, y + 1) == 'M'
                    && get_field(&input, x - 2, y + 2) == 'A'
                    && get_field(&input, x - 3, y + 3) == 'S'
                {
                    score += 1;
                }
                if get_field(&input, x, y + 1) == 'M'
                    && get_field(&input, x, y + 2) == 'A'
                    && get_field(&input, x, y + 3) == 'S'
                {
                    score += 1;
                }
                if get_field(&input, x + 1, y + 1) == 'M'
                    && get_field(&input, x + 2, y + 2) == 'A'
                    && get_field(&input, x + 3, y + 3) == 'S'
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
