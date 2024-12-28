use aoc_parse::{parser, Parser};
use itertools::Itertools;
use std::fs::read_to_string;
use std::io::repeat;
use aoc_parse::prelude::{digit, line};

fn main() {
    let contents = read_to_string("input.txt").expect("Should have been able to read the file");
    let parser = parser!(line(digit+));
    let input = parser.parse(&contents).unwrap();

    let mut decompressed: Vec<Option<usize>> = Vec::new();
    let mut file_index = 0;
    for (index, number) in input.into_iter().enumerate() {
        let is_file = index % 2 == 0;
        if is_file {
            for x in 0..number {
                decompressed.push(Some(file_index))
            }
            file_index += 1;
        } else {
            for x in 0..number {
                decompressed.push(None)
            }
        }
    }
    // dbg!(&decompressed);

    let mut decompressed_vec = decompressed;
    for index in 0..decompressed_vec.len() {
        if *decompressed_vec.get(index).unwrap() == None {
            // dbg!(&decompressed_vec.iter().join(""));
            let last_number = decompressed_vec
                .iter()
                .rposition(|&char| char != None)
                .unwrap();
            if index < last_number {
                decompressed_vec.swap(index, last_number);
            } else {
                break;
            }
        }
    }

    dbg!(&decompressed_vec);

    let mut checksum = 0;
    for (index, &number) in decompressed_vec.iter().enumerate() {
        if number.is_none() {
            break;
        }
        let number: usize = number.unwrap().to_string().parse().unwrap();
        checksum += index * number;
    }
    dbg!(checksum);
}
