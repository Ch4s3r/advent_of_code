mod parser;

use memoize::memoize;
use std::collections::HashMap;

const FFT_MASK: u8 = 1;
const DAC_MASK: u8 = 2;

#[memoize(Ignore: graph)]
fn count_paths(current: String, mask: u8, graph: &HashMap<String, Vec<String>>) -> usize {
    if current == "out" {
        return if mask == (FFT_MASK | DAC_MASK) { 1 } else { 0 };
    }
    
    let mut total = 0;
    if let Some(neighbors) = graph.get(&current) {
        for neighbor in neighbors {
            let mut next_mask = mask;
            if neighbor == "fft" { next_mask |= FFT_MASK; }
            if neighbor == "dac" { next_mask |= DAC_MASK; }
            
            total += count_paths(neighbor.clone(), next_mask, graph);
        }
    }
    total
}

fn main() {
    let input = include_str!("../input.txt");
    // Trim whitespace to avoid parsing issues with trailing newlines if any
    let input = input.trim();
    
    let graph = parser::parse_input(input);
    
    let mut start_mask = 0;
    if "svr" == "fft" { start_mask |= FFT_MASK; }
    if "svr" == "dac" { start_mask |= DAC_MASK; }

    let result = count_paths("svr".to_string(), start_mask, &graph);
        
    println!("Total paths from 'svr' to 'out' visiting 'fft' and 'dac': {}", result);
}
