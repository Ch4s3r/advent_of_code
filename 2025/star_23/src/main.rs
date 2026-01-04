mod parser;
mod solver;

use parser::parse_input;
use solver::solve;
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let content = fs::read_to_string("input.txt")?;
    let parsed = parse_input(&content)?;
    
    solve(&parsed);
    
    Ok(())
}
