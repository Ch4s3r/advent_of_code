use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashSet;

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("data/input.txt")?;
    let reader = BufReader::new(file);

    let mut combined_lines: Vec<String> = Vec::new();
    let mut collect_line = String::new();
    for line in reader.lines().filter_map(|c| c.ok()) {
        if !line.is_empty() {
            collect_line += &line;
        } else {
            combined_lines.push(collect_line.clone());
            collect_line = String::new();
        }
    }
    combined_lines.push(collect_line.clone());


    dbg!(combined_lines.iter().map(|line|
        line.chars().collect::<HashSet<char>>().iter().count()
    ).sum::<usize>());

    Ok(())
}
