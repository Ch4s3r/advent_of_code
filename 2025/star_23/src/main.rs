mod parser;
mod backtracking;
mod backtracking_simplified;
mod approximation;

use parser::parse_input;
use std::fs;
use std::time::Instant;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let content = fs::read_to_string("input.txt")?;
    let parsed = parse_input(&content)?;
    
    println!("=== Example 1: Backtracking Simplified (RoaringBitmap) ===");
    let start = Instant::now();
    backtracking_simplified::solve(&parsed);
    let elapsed = start.elapsed();
    println!("Time: {:.3}ms\n", elapsed.as_secs_f64() * 1000.0);
    
    println!("=== Example 2: Full Backtracking Solution ===");
    let start = Instant::now();
    backtracking::solve(&parsed);
    let elapsed = start.elapsed();
    println!("Time: {:.3}ms\n", elapsed.as_secs_f64() * 1000.0);
    
    println!("=== Example 3: Approximation (Cell Count Only) ===");
    let start = Instant::now();
    approximation::solve(&parsed);
    let elapsed = start.elapsed();
    println!("Time: {:.3}ms\n", elapsed.as_secs_f64() * 1000.0);
    
    Ok(())
}
