#![feature(try_trait)]

use std::fs::File;
use std::io::{BufReader, BufRead};
use std::error::Error;
use std::convert::{TryFrom, TryInto};
use std::string::ParseError;
use std::option::NoneError;
use std::ops::Shl;
use std::num::ParseIntError;

#[derive(Debug)]
struct Seat {
    row: i32,
    col: i32,
    id: i32,
}

impl TryFrom<String> for Seat {
    type Error = ParseIntError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let str_row = &value[0..7];
        let str_col = &value[7..];

        // dbg!(str_row, str_col);

        let binary_str_row: String = str_row.chars().map(|c| if c == 'F' { '0' } else { '1' }).collect();
        let binary_str_col: String = str_col.chars().map(|c| if c == 'L' { '0' } else { '1' }).collect();

        // dbg!(&binary_str_row, &binary_str_col);

        let row = i32::from_str_radix(&binary_str_row, 2)?;
        let col = i32::from_str_radix(&binary_str_col, 2)?;

        // dbg!(row, col);

        Ok(
            Seat {
                row,
                col,
                id: row * 8 + col,
            }
        )
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("data/input.txt")?;
    let reader = BufReader::new(file);

    let mut max_id = 0;
    for seat in reader.lines().filter_map(Result::ok) {
        let parsed_seat = Seat::try_from(seat)?;
        max_id = max_id.max(parsed_seat.id);
        // dbg!(seat.id);
    }
    dbg!(max_id);


    Ok(())
}
// FBFBBFFRLR