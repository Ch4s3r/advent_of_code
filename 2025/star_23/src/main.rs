mod parser;
mod solver;
mod approximation;

use parser::parse_input;
use solver::solve;
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let content = fs::read_to_string("input.txt")?;
    let parsed = parse_input(&content)?;
    
    println!("=== Example 1: Full Backtracking Solution ===");
    solve(&parsed);
    
    println!("=== Example 2: Approximation (Cell Count Only) ===");
    approximation::solve(&parsed);
    
    Ok(())
}
