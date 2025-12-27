mod parser;

use parser::{parse_all, Machine};
use std::collections::{HashSet, VecDeque};
use memoize::memoize;

/// Press a button schematic and toggle the specified indicator lights
fn press_button(lights: &[bool], button_indices: &[u32]) -> Vec<bool> {
    let mut new_lights = lights.to_vec();
    for &idx in button_indices {
        if (idx as usize) < new_lights.len() {
            new_lights[idx as usize] = !new_lights[idx as usize];
        }
    }
    new_lights
}

/// Convert Vec<bool> to String representation for memoization
fn bool_vec_to_string(lights: &[bool]) -> String {
    lights.iter().map(|b| if *b { '1' } else { '0' }).collect()
}

/// Memoized function to check if a state matches target
#[memoize]
fn check_target(current_state: String, target_state: String) -> bool {
    current_state == target_state
}

/// Find the minimum number of button presses to match the target indicator light diagram using BFS with memoization
fn find_min_presses(machine: &Machine) -> Option<u32> {
    let target = &machine.indicator_light_diagram;
    let button_schematics = &machine.button_schematics;
    
    // Start with all lights as false
    let initial_state: Vec<bool> = vec![false; target.len()];
    let target_state_str = bool_vec_to_string(target);
    
    // BFS to find shortest path
    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();
    
    let initial_state_str = bool_vec_to_string(&initial_state);
    queue.push_back((initial_state.clone(), initial_state_str.clone(), 0u32));
    visited.insert(initial_state);
    
    while let Some((current_lights, current_state_str, presses)) = queue.pop_front() {
        // Check if we've reached the target using memoized function
        if check_target(current_state_str, target_state_str.clone()) {
            return Some(presses);
        }
        
        // Try pressing each button
        for button in button_schematics {
            let next_lights = press_button(&current_lights, button);
            
            if !visited.contains(&next_lights) {
                visited.insert(next_lights.clone());
                let next_state_str = bool_vec_to_string(&next_lights);
                queue.push_back((next_lights, next_state_str, presses + 1));
            }
        }
    }
    
    None
}

fn main() {
    let machines = parse_all("input.txt");
    
    let mut total_presses = 0;
    for (i, machine) in machines.iter().enumerate() {
        match find_min_presses(machine) {
            Some(presses) => {
                println!("Machine {}: {} button presses", i + 1, presses);
                total_presses += presses;
            }
            None => println!("Machine {}: No solution found", i + 1),
        }
    }
    
    println!("\nTotal presses: {}", total_presses);
}
