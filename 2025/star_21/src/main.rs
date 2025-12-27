mod parser;

use memoize::memoize;
use std::collections::HashMap;
use std::sync::OnceLock;

static GRAPH: OnceLock<HashMap<String, Vec<String>>> = OnceLock::new();

#[memoize]
fn count_paths(current: String) -> usize {
    if current == "out" {
        return 1;
    }

    let graph = GRAPH.get().expect("Graph not initialized");
    
    if let Some(neighbors) = graph.get(&current) {
        let mut sum = 0;
        for neighbor in neighbors {
            sum += count_paths(neighbor.clone());
        }
        sum
    } else {
        0
    }
}

fn main() {
    let input = include_str!("../input.txt");
    // Trim whitespace to avoid parsing issues with trailing newlines if any
    let input = input.trim();
    
    let graph = parser::parse_input(input);
    
    // Initialize the global graph
    GRAPH.set(graph).expect("Failed to set graph");

    let result = count_paths("you".to_string());
    
    println!("Total unique paths from 'you' to 'out': {}", result);
}
