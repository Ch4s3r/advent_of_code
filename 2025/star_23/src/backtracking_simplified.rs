use crate::parser;

use crate::parser::Shape;
use std::collections::HashSet;
use roaring::RoaringBitmap;
use memoize::memoize;

fn rotate_90(grid: &[String]) -> Vec<String> {
    let grid_size = grid.len();
    (0..grid_size).map(|column_index| (0..grid_size).rev().map(|row_index| grid[row_index].chars().nth(column_index).unwrap()).collect()).collect()
}

fn can_place_shape(grid: &RoaringBitmap, shape: &Shape, starting_row: usize, starting_col: usize, container_width: usize, container_height: usize) -> bool {
    for (shape_row_idx, shape_row) in shape.grid.iter().enumerate() {
        for (shape_col_idx, cell_char) in shape_row.chars().enumerate() {
            if cell_char == '#' {
                let grid_position = ((starting_row + shape_row_idx) * container_width + (starting_col + shape_col_idx)) as u32;
                if grid_position >= (container_width * container_height) as u32 || grid.contains(grid_position) {
                    return false;
                }
            }
        }
    }
    true
}

fn place_shape(grid: &mut RoaringBitmap, shape: &Shape, starting_row: usize, starting_col: usize, container_width: usize) {
    for (shape_row_idx, shape_row) in shape.grid.iter().enumerate() {
        for (shape_col_idx, cell_char) in shape_row.chars().enumerate() {
            if cell_char == '#' {
                grid.insert(((starting_row + shape_row_idx) * container_width + (starting_col + shape_col_idx)) as u32);
            }
        }
    }
}

fn remove_shape(grid: &mut RoaringBitmap, shape: &Shape, starting_row: usize, starting_col: usize, container_width: usize) {
    for (shape_row_idx, shape_row) in shape.grid.iter().enumerate() {
        for (shape_col_idx, cell_char) in shape_row.chars().enumerate() {
            if cell_char == '#' {
                grid.remove(((starting_row + shape_row_idx) * container_width + (starting_col + shape_col_idx)) as u32);
            }
        }
    }
}

#[memoize]
fn calculate_cells_in_shape(shape_grid: String) -> usize {
    shape_grid.chars().filter(|&c| c == '#').count()
}

fn calculate_total_cells_needed(shape_counts_remaining: &[u8], available_shapes: &[Shape]) -> usize {
    shape_counts_remaining.iter().enumerate()
        .map(|(shape_id, &count)| {
            let cells_in_this_shape = available_shapes.iter()
                .find(|s| s.id == shape_id)
                .map(|s| {
                    let grid_string = s.grid.iter().flat_map(|row| row.chars()).collect::<String>();
                    calculate_cells_in_shape(grid_string)
                })
                .unwrap_or(0);
            cells_in_this_shape * count as usize
        })
        .sum()
}

fn backtrack(
    occupied_cells: &mut RoaringBitmap,
    shape_counts_remaining: Vec<u8>,
    available_shapes: &[Shape],
    container_width: usize,
    container_height: usize,
) -> bool {
    if shape_counts_remaining.iter().all(|&count| count == 0) {
        return true;
    }
    
    let total_cells_needed = calculate_total_cells_needed(&shape_counts_remaining, available_shapes);
    let available_cells = (container_width * container_height) as u64 - occupied_cells.len();
    if total_cells_needed as u64 > available_cells {
        return false;
    }
    
    for grid_row in 0..container_height {
        for grid_col in 0..container_width {
            let grid_position = (grid_row * container_width + grid_col) as u32;
            if !occupied_cells.contains(grid_position) {
                for current_shape in available_shapes {
                    if shape_counts_remaining[current_shape.id] > 0 && can_place_shape(occupied_cells, current_shape, grid_row, grid_col, container_width, container_height) {
                        place_shape(occupied_cells, current_shape, grid_row, grid_col, container_width);
                        let mut remaining_after_placement = shape_counts_remaining.clone();
                        remaining_after_placement[current_shape.id] -= 1;
                        
                        if backtrack(occupied_cells, remaining_after_placement, available_shapes, container_width, container_height) {
                            return true;
                        }
                        
                        remove_shape(occupied_cells, current_shape, grid_row, grid_col, container_width);
                    }
                }
            }
        }
    }
    
    false
}


pub fn solve(parsed: &parser::Input) {
    let mut all_shapes_with_rotations = parsed.shapes.clone();
    all_shapes_with_rotations = add_rotations(all_shapes_with_rotations);
    

    let (fitting_count, non_fitting_count) = parsed.containers.iter().enumerate().fold((0, 0), |(fitting_count, non_fitting_count), (_container_idx, container)| {
        let mut occupied_grid_cells = RoaringBitmap::new();
        let required_shape_counts: Vec<u8> = container.shapes.iter().map(|&count| count as u8).collect();
        
        if backtrack(&mut occupied_grid_cells, required_shape_counts, &all_shapes_with_rotations, container.width, container.height) {
            // println!("Container {:3} ({}x{}): YES", container_idx, container.width, container.height);
            (fitting_count + 1, non_fitting_count)
        } else {
            // println!("Container {:3} ({}x{}): NO", container_idx, container.width, container.height);
            (fitting_count, non_fitting_count + 1)
        }
    });
    
    println!("Summary: {}/{} containers fit.", fitting_count, fitting_count + non_fitting_count);
}

fn add_rotations(original_shapes: Vec<parser::Shape>) -> Vec<parser::Shape> {
    let mut seen_rotations = HashSet::new();
    let mut shapes_with_all_rotations = Vec::new();
    
    for original_shape in original_shapes {
        let mut current_rotation = original_shape.grid.clone();
        for _rotation_step in 0..4 {
            if seen_rotations.insert(current_rotation.clone()) {
                shapes_with_all_rotations.push(parser::Shape { id: original_shape.id, grid: current_rotation.clone() });
            }
            current_rotation = rotate_90(&current_rotation);
        }
    }
    
    shapes_with_all_rotations
}
