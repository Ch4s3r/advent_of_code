#![feature(slice_take)]

use aoc_parse::parser;
use aoc_parse::prelude::*;
use itertools::{Itertools, Permutations, repeat_n};
use std::fs;
use std::ptr::eq;

fn main() {
    let contents =
        fs::read_to_string("input.txt").expect("Should have been able to read the file");

    let parser = parser!(lines(u64 ": " repeat_sep(u64, " ")));
    let input = parser.parse(&contents).unwrap();
    let symbols = ['*', '+', '|'];

    let score: u64 = input
        .into_iter()
        .map(|(expected, numbers)| {
            dbg!(expected);
            let symbols_needed = numbers.len() - 1;
            let symbol_permutations = repeat_n(symbols.iter(), symbols_needed)
                .multi_cartesian_product()
                .collect_vec();
            
            let mut score = 0;

            for symbol_permutation in symbol_permutations {
                let mut symbol_permuation_iter = symbol_permutation.iter();
                let result = numbers.clone().into_iter().reduce(|result, number| {
                    let symbol = **symbol_permuation_iter.next().unwrap();
                    match symbol {
                        '*' => result * number,
                        '+' => result + number,
                        '|' => (result.to_string() + &number.to_string()).parse().unwrap_or(0),
                        _ => {
                            panic!("symbol unknown")
                        }
                    }
                }).unwrap();
                if result == expected {
                    score = expected;
                    break;
                }
            }

            // let mut numbers_iter = numbers.iter();
            // let mut symbols_iter = symbol_permutations.iter();
            // let result = numbers_iter.next().unwrap();
            // for symbols_permutation in symbols_iter {
            //     let number = numbers.iter().next().unwrap();
            //     for symbol in symbols_permutation {
            //         match symbol {
            //             '*' => result * number,
            //             '+' => result + number,
            //             _ => {
            //                 panic!("symbol unknown")
            //             }
            //         };
            //     }
            // }

            score
        })
        .sum();
    dbg!(score);
}
