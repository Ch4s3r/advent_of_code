use nom::{
    IResult,
    branch::alt,
    bytes::complete::tag,
    character::complete::line_ending,
    combinator::map,
    multi::{many1, many0},
    sequence::terminated,
    Parser,
};
use std::fmt;
use roaring::RoaringBitmap;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Pos {
    pub x: usize,
    pub y: usize,
}

impl Pos {
    pub fn new(x: usize, y: usize) -> Self {
        Pos { x, y }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Cell {
    Start,
    Splitter,
    Beam,
    Empty,
}

impl fmt::Debug for Cell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Cell::Start => write!(f, "S"),
            Cell::Splitter => write!(f, "^"),
            Cell::Beam => write!(f, "|"),
            Cell::Empty => write!(f, "."),
        }
    }
}

#[derive(Clone)]
pub struct Grid {
    width: usize,
    height: usize,
    start_pos: Pos,
    start_bitmap: RoaringBitmap,
    splitter_bitmap: RoaringBitmap,
    beam_bitmap: RoaringBitmap,
    empty_bitmap: RoaringBitmap,
}

impl Grid {
    fn index(&self, x: usize, y: usize) -> Option<u32> {
        if x < self.width && y < self.height {
            Some((y * self.width + x) as u32)
        } else {
            None
        }
    }

    pub fn get(&self, x: usize, y: usize) -> Option<Cell> {
        let idx = self.index(x, y)?;
        
        if self.start_bitmap.contains(idx) {
            Some(Cell::Start)
        } else if self.splitter_bitmap.contains(idx) {
            Some(Cell::Splitter)
        } else if self.beam_bitmap.contains(idx) {
            Some(Cell::Beam)
        } else if self.empty_bitmap.contains(idx) {
            Some(Cell::Empty)
        } else {
            None
        }
    }

    pub fn set(&mut self, x: usize, y: usize, cell: Cell) -> bool {
        let Some(idx) = self.index(x, y) else {
            return false;
        };

        // Remove from all bitmaps first
        self.start_bitmap.remove(idx);
        self.splitter_bitmap.remove(idx);
        self.beam_bitmap.remove(idx);
        self.empty_bitmap.remove(idx);

        // Insert into the correct bitmap
        match cell {
            Cell::Start => self.start_bitmap.insert(idx),
            Cell::Splitter => self.splitter_bitmap.insert(idx),
            Cell::Beam => self.beam_bitmap.insert(idx),
            Cell::Empty => self.empty_bitmap.insert(idx),
        };

        true
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn start_pos(&self) -> Pos {
        self.start_pos
    }

    pub fn beam_positions(&self) -> impl Iterator<Item = Pos> + '_ {
        self.beam_bitmap.iter().map(|idx| {
            let idx = idx as usize;
            let x = idx % self.width;
            let y = idx / self.width;
            Pos::new(x, y)
        })
    }
}

impl fmt::Debug for Grid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Grid {{")?;
        writeln!(f, "  dimensions: {} x {}", self.height, self.width)?;
        writeln!(f, "  start_pos: {:?}", self.start_pos)?;
        writeln!(f, "  grid:")?;
        for y in 0..self.height {
            write!(f, "    ")?;
            for x in 0..self.width {
                if let Some(cell) = self.get(x, y) {
                    write!(f, "{:?}", cell)?;
                } else {
                    write!(f, "?")?;
                }
            }
            writeln!(f)?;
        }
        writeln!(f, "}}")
    }
}

fn parse_cell(input: &str) -> IResult<&str, Cell> {
    alt((
        map(tag("S"), |_| Cell::Start),
        map(tag("^"), |_| Cell::Splitter),
        map(tag("|"), |_| Cell::Beam),
        map(tag("."), |_| Cell::Empty),
    )).parse(input)
}

fn parse_row(input: &str) -> IResult<&str, Vec<Cell>> {
    many1(parse_cell).parse(input)
}

fn parse_line(input: &str) -> IResult<&str, Vec<Cell>> {
    terminated(parse_row, line_ending).parse(input)
}

pub fn parse_input(input: &str) -> IResult<&str, Grid> {
    let (remaining, cells) = many0(parse_line).parse(input)?;

    if cells.is_empty() {
        return Err(nom::Err::Error(nom::error::Error::new(
            "",
            nom::error::ErrorKind::Eof,
        )));
    }

    let height = cells.len();
    let width = cells[0].len();

    let mut start_bitmap = RoaringBitmap::new();
    let mut splitter_bitmap = RoaringBitmap::new();
    let mut beam_bitmap = RoaringBitmap::new();
    let mut empty_bitmap = RoaringBitmap::new();
    let mut start_pos = Pos::new(0, 0);

    // Populate the bitmaps
    for (y, row) in cells.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            let idx = (y * width + x) as u32;
            match cell {
                Cell::Start => {
                    start_bitmap.insert(idx);
                    start_pos = Pos::new(x, y);
                }
                Cell::Splitter => {
                    splitter_bitmap.insert(idx);
                }
                Cell::Beam => {
                    beam_bitmap.insert(idx);
                }
                Cell::Empty => {
                    empty_bitmap.insert(idx);
                }
            }
        }
    }

    Ok((
        remaining,
        Grid {
            width,
            height,
            start_pos,
            start_bitmap,
            splitter_bitmap,
            beam_bitmap,
            empty_bitmap,
        },
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_cell() {
        assert_eq!(parse_cell("S"), Ok(("", Cell::Start)));
        assert_eq!(parse_cell("^"), Ok(("", Cell::Splitter)));
        assert_eq!(parse_cell("."), Ok(("", Cell::Empty)));
    }

    #[test]
    fn test_parse_row() {
        let input = "S.^.";
        let (_, row) = parse_row(input).unwrap();
        assert_eq!(
            row,
            vec![Cell::Start, Cell::Empty, Cell::Splitter, Cell::Empty]
        );
    }

    #[test]
    fn test_parse_grid() {
        let input = ".......S.......\n...............";
        let (_, grid) = parse_input(input).unwrap();
        assert_eq!(grid.start_pos(), Pos::new(7, 0));
        assert_eq!(grid.height(), 2);
        assert_eq!(grid.width(), 15);
    }

    #[test]
    fn test_grid_get_set() {
        let input = ".......S.......\n...............";
        let (_, mut grid) = parse_input(input).unwrap();
        
        // Test get
        assert_eq!(grid.get(7, 0), Some(Cell::Start));
        assert_eq!(grid.get(0, 0), Some(Cell::Empty));
        
        // Test set
        assert!(grid.set(0, 0, Cell::Beam));
        assert_eq!(grid.get(0, 0), Some(Cell::Beam));
        
        // Test out of bounds
        assert_eq!(grid.get(100, 100), None);
        assert!(!grid.set(100, 100, Cell::Empty));
    }
}
