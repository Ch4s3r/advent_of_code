use std::fs::File;
use std::io::{BufReader, BufRead};
use std::error::Error;
use std::convert::{TryFrom, TryInto};
use std::string::ParseError;
use std::option::NoneError;

#[derive(Debug)]
struct Seat {
    row: i32,
    col: i32,
    id: i32,
}

impl TryFrom<&str> for Seat {
    type Error = NoneError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let char = value.chars().nth(0)?;
        let x = if char == 'F' {
            1
        } else {
            0
        };
        Ok(
            Seat {
                row: x,
                col: 0,
                id: 0,
            }
        )
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("data/input.txt")?;
    let reader = BufReader::new(file);

    // let mut inputs: Vec<Seat> = Vec::new();
    // for seat in reader.lines().filter_map(Result::ok) {
    //    println!("{}", seat)
    // }

    dbg!(Seat::try_from("FBFBBFFRLR"));

    Ok(())
}
// FBFBBFFRLR