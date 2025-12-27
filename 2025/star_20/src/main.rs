mod parser;
use parser::{parse_all, Machine};
use indicatif::ProgressBar;

fn solve_machine(machine: &Machine) -> u32 {
    let num_equations = machine.joltage_requirements.len();
    let num_vars = machine.button_schematics.len();
    
    // Build augmented matrix [A | b]
    let mut matrix = vec![vec![0.0; num_vars + 1]; num_equations];
    
    for (j, button) in machine.button_schematics.iter().enumerate() {
        for &counter_idx in button {
            matrix[counter_idx as usize][j] = 1.0;
        }
    }
    
    for (i, &target) in machine.joltage_requirements.iter().enumerate() {
        matrix[i][num_vars] = target as f64;
    }
    
    // Gaussian elimination
    let mut pivot_row = 0;
    let mut pivot_cols = vec![];
    let mut free_vars = vec![];
    let mut pivot_map = vec![None; num_vars]; // Maps column to row index
    
    for col in 0..num_vars {
        if pivot_row >= num_equations {
            free_vars.push(col);
            continue;
        }
        
        // Find pivot
        let mut max_row = pivot_row;
        for row in pivot_row + 1..num_equations {
            if matrix[row][col].abs() > matrix[max_row][col].abs() {
                max_row = row;
            }
        }
        
        if matrix[max_row][col].abs() < 1e-9 {
            free_vars.push(col);
            continue;
        }
        
        // Swap rows
        matrix.swap(pivot_row, max_row);
        pivot_cols.push(col);
        pivot_map[col] = Some(pivot_row);
        
        // Normalize pivot row
        let pivot_val = matrix[pivot_row][col];
        for j in col..=num_vars {
            matrix[pivot_row][j] /= pivot_val;
        }
        
        // Eliminate other rows
        for row in 0..num_equations {
            if row != pivot_row {
                let factor = matrix[row][col];
                for j in col..=num_vars {
                    matrix[row][j] -= factor * matrix[pivot_row][j];
                }
            }
        }
        
        pivot_row += 1;
    }
    
    // Check for inconsistency (0 = non-zero)
    for row in pivot_row..num_equations {
        if matrix[row][num_vars].abs() > 1e-9 {
            return u32::MAX; // Impossible
        }
    }
    
    // Calculate upper bound for search based on max target
    let max_target = *machine.joltage_requirements.iter().max().unwrap_or(&0);
    
    solve_with_free_vars(&matrix, &pivot_map, &free_vars, vec![0.0; num_vars], 0, max_target)
}

fn solve_with_free_vars(
    matrix: &Vec<Vec<f64>>,
    pivot_map: &Vec<Option<usize>>,
    free_vars: &Vec<usize>,
    mut current_solution: Vec<f64>,
    free_var_idx: usize,
    max_target: u32,
) -> u32 {
    if free_var_idx == free_vars.len() {
        // Calculate pivot variables
        let num_vars = current_solution.len();
        let mut sum_presses = 0;
        
        // Sum free vars
        for &fv in free_vars {
            sum_presses += current_solution[fv] as u32;
        }
        
        // Calculate and check pivot vars
        for (col, row_opt) in pivot_map.iter().enumerate() {
            if let Some(row) = row_opt {
                let mut val = matrix[*row][num_vars]; // constant
                for &fv in free_vars {
                    val -= matrix[*row][fv] * current_solution[fv];
                }
                
                if val < -1e-9 { return u32::MAX; }
                let rounded = val.round();
                if (val - rounded).abs() > 1e-9 { return u32::MAX; }
                
                current_solution[col] = rounded;
                sum_presses += rounded as u32;
            }
        }
        return sum_presses;
    }
    
    let col = free_vars[free_var_idx];
    let mut min_total = u32::MAX;
    
    // Try values for this free variable
    // Since A >= 0 and b >= 0 in original system, x_i <= max(b)
    for val in 0..=max_target {
        current_solution[col] = val as f64;
        let res = solve_with_free_vars(matrix, pivot_map, free_vars, current_solution.clone(), free_var_idx + 1, max_target);
        min_total = min_total.min(res);
    }
    min_total
}

fn main() {
    let input_file = std::env::args().nth(1).unwrap_or("input.txt".to_string());
    let machines = parse_all(&input_file);
    let total_machines = machines.len() as u64;
    
    let pb = ProgressBar::new(total_machines);
    pb.set_style(
        indicatif::ProgressStyle::default_bar()
            .template("{spinner:.green} [{bar:40.cyan/blue}] {pos}/{len} machines")
            .unwrap()
            .progress_chars("#>-")
    );
    
    let mut total = 0u32;
    
    for machine in machines.iter() {
        let presses = solve_machine(machine);
        if presses != u32::MAX {
            total += presses;
        }
        pb.inc(1);
    }
    
    pb.finish_with_message("All machines solved!");
    println!("\nTotal button presses: {}", total);
}