mod parser;

use parser::parse_grid;
use std::fs;

fn count_adjacent_paper_rolls(room: &parser::Room, pos: parser::Position) -> usize {
    let mut count = 0;
    
    let directions = [
        (-1, -1), // top-left
        (0, -1),  // top
        (1, -1),  // top-right
        (-1, 0),  // left
        (1, 0),   // right
        (-1, 1),  // bottom-left
        (0, 1),   // bottom
        (1, 1),   // bottom-right
    ];
    
    for (dx, dy) in directions {
        let nx = (pos.x as i32 + dx) as usize;
        let ny = (pos.y as i32 + dy) as usize;
        
        if nx < room.width && ny < room.height {
            if room.grid[ny][nx] == parser::Cell::PaperRoll {
                count += 1;
            }
        }
    }
    
    count
}

fn calculate_isolation_sum(room: &parser::Room) -> usize {
    let mut sum = 0;
    
    for position in &room.paper_rolls {
        let adjacent_count = count_adjacent_paper_rolls(room, *position);
        if adjacent_count < 4 {
            sum += 1;
        }
    }
    
    sum
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Failed to read input file");
    
    match parse_grid(&input) {
        Ok((_, room)) => {
            println!("Parsed room successfully!");
            println!("Room dimensions: {} x {}", room.width, room.height);
            println!("Paper rolls found: {}\n", room.count_paper_rolls());
            
            // Calculate isolation sum
            let isolation_sum = calculate_isolation_sum(&room);
            println!("Paper rolls with < 4 adjacent neighbors: {}", isolation_sum);
        }
        Err(e) => {
            eprintln!("Failed to parse room: {:?}", e);
        }
    }
}
