mod parser;

use memoize::memoize;
use parser::{Cell, parse_input};
use std::fs;

use crate::parser::{Grid, Pos};

fn main() {
    let input = fs::read_to_string("input.txt").expect("Failed to read input file");
    let grid = parse_input(&input).expect("Failed to parse input").1;

    print!("Grid:\n{:?}", grid);

    let total_timelines =
        create_beam_path(Pos::new(grid.start_pos().x, grid.start_pos().y), &grid) + 1;
    println!("Total timelines created: {}", total_timelines);
}

#[memoize(Ignore: grid)]
fn create_beam_path(pos: Pos, grid: &Grid) -> usize {
    let next_position = grid.get(pos.x, pos.y + 1);
    match next_position {
        Some(Cell::Empty) => create_beam_path(Pos::new(pos.x, pos.y + 1), grid),
        Some(Cell::Splitter) => {
            1 + create_beam_path(Pos::new(pos.x + 1, pos.y + 1), grid)
                + create_beam_path(Pos::new(pos.x - 1, pos.y + 1), grid)
        }
        _ => 0,
    }
}
