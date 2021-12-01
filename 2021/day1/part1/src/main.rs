use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::Instant;

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("../data/input.txt")?;
    let reader = BufReader::new(file);

    let mut numbers: Vec<i32> = Vec::new();

    for line in reader.lines() {
        let number = line?.parse()?;
        numbers.push(number);
    }

    let mut count = 0;

    for window in numbers.windows(2) {
        if window[0] < window[1] { count += 1; }
    }
    println!("count: {}", count);
    Ok(())
}
