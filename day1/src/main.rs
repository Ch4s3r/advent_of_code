use std::fs::File;
use std::io::{BufReader, BufRead};
use std::error::Error;

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

    for (index, number1) in numbers.iter().enumerate() {
        for number2 in &numbers[index + 1..] {
            count += 1;
            if number1 + number2 == 2020 {
                println!("{} * {} = {}", number1, number2, number1 * number2)
            }
        }
    }

    println!("iterations: {}", count);

    Ok(())
}
