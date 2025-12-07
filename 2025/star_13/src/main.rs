mod parser;

use parser::{Cell, parse_input};
use std::fs;

use crate::parser::Pos;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Failed to read input file");
    let mut grid = parse_input(&input).expect("Failed to parse input").1;

    // println!("{:#?}", grid);
    grid.set(grid.start_pos().x, grid.start_pos().y + 1, Cell::Beam);
    // println!("{:#?}", grid);
    let mut total_splitters_hit = 0;

    loop {
        let mut new_beams = vec![];
        let max_y = grid.beam_positions().map(|p| p.y).max().unwrap_or(0);
        for beam_position in grid.beam_positions().filter(|p| p.y == max_y) {
            // println!("Beam at position: {:?}", beam_position);
            let next_position = grid.get(beam_position.x, beam_position.y + 1);
            match next_position {
                Some(Cell::Splitter) => {
                    // println!("Beam hit a splitter at position: {:?}", beam_position);
                    new_beams.push(Pos::new(beam_position.x - 1, beam_position.y + 1));
                    new_beams.push(Pos::new(beam_position.x + 1, beam_position.y + 1));
                    total_splitters_hit += 1;
                },
                Some(Cell::Empty) => {
                    new_beams.push(Pos::new(beam_position.x, beam_position.y + 1));
                },
                _ => {},
            }
        }
        for new_beam in new_beams.iter() {
            grid.set(new_beam.x, new_beam.y, Cell::Beam);
        }
        // print!("{:#?}", grid);
        if new_beams.is_empty() {
            break;
        }
    }
    println!("Total splitters hit: {}", total_splitters_hit);
}
