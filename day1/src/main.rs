use std::fs::File;
use std::io::{BufReader, BufRead};
use std::error::Error;
use std::time::Instant;

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("data/input.txt")?;
    let reader = BufReader::new(file);

    let mut numbers: Vec<i32> = Vec::new();

    for line in reader.lines() {
        let number = line?.parse()?;
        numbers.push(number);
    }

    println!("size: {}", numbers.len());

    let mut count = 0;

    let start = Instant::now();

    for number1 in &numbers {
        for number2 in &numbers {
            for number3 in &numbers {
                count += 1;
                if number1 + number2 + number3 == 2020 {
                    println!("{} * {} * {} = {}", number1, number2, number3, number1 * number2 * number3)
                }
            }
        }
    }
    let duration = start.elapsed();
    println!("duration: {:?}", duration);
    println!("iterations: {}", count);

    Ok(())
}
