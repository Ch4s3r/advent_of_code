use aoc_parse::prelude::{digit, line};
use aoc_parse::{Parser, parser};
use itertools::Itertools;
use std::fs::read_to_string;
use std::io::repeat;

fn main() {
    let contents =
        read_to_string("input.txt").expect("Should have been able to read the file");
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
    // dbg!(
    //     &decompressed
    //         .iter()
    //         .map(|&number_option| number_option
    //             .map(|number| number.to_string())
    //             .unwrap_or(".".to_string()))
    //         .join("")
    // );

    let mut decompressed_vec = decompressed;
    // dbg!(&decompressed_vec);
    let mut number_end_index = decompressed_vec.len() - 1;
    while number_end_index > 0 {
        let number = *decompressed_vec.get(number_end_index).unwrap();
        if number.is_none() {
            number_end_index -= 1;
            continue;
        }
        let mut number_begin_index = number_end_index;
        loop {
            if number_begin_index - 1 > 0
                && *decompressed_vec.get(number_begin_index - 1).unwrap() == number
            {
                number_begin_index -= 1;
            } else {
                break;
            }
        }
        let number_count = number_end_index - number_begin_index + 1;
        // dbg!(number_count);
        let mut empty_begin_index = 0;
        while empty_begin_index < decompressed_vec.len() {
            let field = *decompressed_vec.get(empty_begin_index).unwrap();
            if field.is_some() {
                empty_begin_index += 1;
                continue;
            }
            let mut empty_end_index = empty_begin_index;
            loop {
                if decompressed_vec
                    .get(empty_end_index + 1)
                    .unwrap_or(&Some(0usize))
                    .is_none()
                {
                    empty_end_index += 1;
                } else {
                    break;
                }
            }
            if empty_end_index > number_begin_index {
                break
            }
            let empty_count = empty_end_index - empty_begin_index + 1;
            // dbg!(empty_count);

            if number_count <= empty_count {
                // dbg!(
                //     &decompressed_vec
                //         .iter()
                //         .map(|&number_option| number_option
                //             .map(|number| number.to_string())
                //             .unwrap_or(".".to_string()))
                //         .join("")
                // );
                for i in 0..number_count {
                    decompressed_vec.swap(empty_begin_index + i, number_begin_index + i)
                }
                // dbg!(
                //     &decompressed_vec
                //         .iter()
                //         .map(|&number_option| number_option
                //             .map(|number| number.to_string())
                //             .unwrap_or(".".to_string()))
                //         .join("")
                // );
                
                break;
            } else {
                empty_begin_index += 1;
                if empty_begin_index > decompressed_vec.len() {
                    break;
                }
            }
        }
        number_end_index -= number_count;
    }
    // dbg!(
    //     &decompressed_vec
    //         .iter()
    //         .map(|&number_option| number_option
    //             .map(|number| number.to_string())
    //             .unwrap_or(".".to_string()))
    //         .join("")
    // );

    let mut checksum = 0;
    for (index, &number) in decompressed_vec.iter().enumerate() {
        if number.is_none() {
            continue;
        }
        let number: usize = number.unwrap().to_string().parse().unwrap();
        checksum += index * number;
    }
    dbg!(checksum);
}
