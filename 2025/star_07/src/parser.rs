use nom::{
    branch::alt,
    character::complete::{char, newline},
    combinator::map,
    multi::{many1, separated_list1},
    IResult,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Cell {
    PaperRoll,  // @
    Empty,      // .
}

#[derive(Debug, Clone, Copy)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

#[derive(Debug, Clone)]
pub struct Room {
    pub width: usize,
    pub height: usize,
    pub paper_rolls: Vec<Position>,
    pub grid: Vec<Vec<Cell>>,
}

impl Room {
    pub fn new(grid: Vec<Vec<Cell>>) -> Self {
        let height = grid.len();
        let width = grid.first().map(|row| row.len()).unwrap_or(0);
        
        let mut paper_rolls = Vec::new();
        
        // Find all paper roll positions
        for y in 0..height {
            for x in 0..width {
                if grid[y][x] == Cell::PaperRoll {
                    paper_rolls.push(Position { x, y });
                }
            }
        }
        
        Room {
            width,
            height,
            paper_rolls,
            grid,
        }
    }
    
    pub fn count_paper_rolls(&self) -> usize {
        self.paper_rolls.len()
    }
}

fn parse_cell(input: &str) -> IResult<&str, Cell> {
    alt((
        map(char('@'), |_| Cell::PaperRoll),
        map(char('.'), |_| Cell::Empty),
    ))(input)
}

fn parse_row(input: &str) -> IResult<&str, Vec<Cell>> {
    many1(parse_cell)(input)
}

pub fn parse_grid(input: &str) -> IResult<&str, Room> {
    let (input, rows) = separated_list1(newline, parse_row)(input)?;
    let room = Room::new(rows);
    Ok((input, room))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_cell_tree() {
        let (remaining, cell) = parse_cell("@.").unwrap();
        assert_eq!(cell, Cell::PaperRoll);
        assert_eq!(remaining, ".");
    }

    #[test]
    fn test_parse_cell_empty() {
        let (remaining, cell) = parse_cell(".@").unwrap();
        assert_eq!(cell, Cell::Empty);
        assert_eq!(remaining, "@");
    }

    #[test]
    fn test_parse_row() {
        let (remaining, row) = parse_row("@.@\n").unwrap();
        assert_eq!(row.len(), 3);
        assert_eq!(row[0], Cell::PaperRoll);
        assert_eq!(row[1], Cell::Empty);
        assert_eq!(row[2], Cell::PaperRoll);
        assert_eq!(remaining, "\n");
    }

    #[test]
    fn test_parse_grid() {
        let input = "@.@\n.@.\n@.@";
        let (remaining, room) = parse_grid(input).unwrap();
        assert_eq!(room.width, 3);
        assert_eq!(room.height, 3);
        assert_eq!(room.count_paper_rolls(), 5);
        assert_eq!(room.grid[0][0], Cell::PaperRoll);
        assert_eq!(room.grid[0][1], Cell::Empty);
        assert_eq!(remaining, "");
    }
}
