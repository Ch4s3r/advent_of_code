use aoc_parse::parser;
use aoc_parse::prelude::*;
use itertools::{Itertools, Permutations, repeat_n};
use std::collections::{HashMap, HashSet};
use std::fmt::{Debug, Display, Formatter};
use std::fs::read_to_string;
use std::hash::Hash;
use std::{fmt, fs};

fn main() {
    let contents =
        read_to_string("input.txt").expect("Should have been able to read the file");
    let parser = parser!(lines(any_char+));
    let input = parser.parse(&contents).unwrap();
    let width = input[0].len();
    let height = input.len();
    let flattened_input = input.into_iter().flatten().collect_vec();
    let mut map = Vec2d::new(flattened_input, width, height);
    let mut antinode_locations: HashSet<(char, i64, i64)> = HashSet::new();

    for current_x in 0..width {
        for current_y in 0..height {
            let current = *map.index(current_y, current_x);
            if current == '.' {
                continue;
            }
            for other_x in 0..width {
                for other_y in 0..height {
                    let other = *map.index(other_y, other_x);
                    if current == other {
                        if current_x == other_x && current_y == other_y {
                            continue;
                        }
                        let diff_x = current_x as i64 - other_x as i64;
                        let diff_y = current_y as i64 - other_y as i64;
                        let antinode_x = current_x as i64 + diff_x;
                        let antinode_y = current_y as i64 + diff_y;

                        if antinode_x < width as i64
                            && antinode_x >= 0
                            && antinode_y < height as i64
                            && antinode_y >= 0
                        {
                            antinode_locations.insert(('x', antinode_x, antinode_y));
                        }
                    }
                }
            }
        }
    }

    println!("{:?}", antinode_locations.len());
    for (char, x, y) in antinode_locations.iter() {
        if map.index(*y as usize, *x as usize) == &'.' {
            *map.index_mut(*y as usize, *x as usize) = '#';
        }
        // println!("{char}: ({x},{y}):\n{map}\n==========");
    }
}

pub struct Vec2d<T> {
    vec: Vec<T>,
    row: usize,
    col: usize,
}

impl<T> Vec2d<T> {
    pub fn new(vec: Vec<T>, row: usize, col: usize) -> Self {
        assert!(vec.len() == row * col);
        Self { vec, row, col }
    }

    pub fn row(&self, row: usize) -> &[T] {
        let i = self.col * row;
        &self.vec[i..(i + self.col)]
    }

    pub fn index(&self, row: usize, col: usize) -> &T {
        let i = self.col * row;
        &self.vec[i + col]
    }

    pub fn index_mut(&mut self, row: usize, col: usize) -> &mut T {
        let i = self.col * row;
        &mut self.vec[i + col]
    }
}
impl fmt::Display for Vec2d<char> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut str = String::new();
        for i in 0..self.row {
            if i != 0 {
                str.push('\n');
            }
            str.push_str(&self.row(i).iter().join(" "));
        }
        write!(f, "{}", str)
    }
}

// (4,3) - (5,5) = (-1,-2)
// . . . . . . . . . .
// . . . # . . . . . .
// . . . . . . . . . .
// . . . . a . . . . .
// . . . . . . . . . .
// . . . . . a . . . .
// . . . . . . . . . .
// . . . . . . # . . .
// . . . . . . . . . .
// . . . . . . . . . .
// (5,3) - (4,5) = (1,-2)
// . . . . . . . . . .
// . . . . . . # . . .
// . . . . . . . . . .
// . . . . . a . . . .
// . . . . . . . . . .
// . . . . a . . . . .
// . . . . . . . . . .
// . . . # . . . . . .
// . . . . . . . . . .
// . . . . . . . . . .
// (3,4) - (6,4) = (-3,0)
// . . . . . . . . . .
// . . . . . . . . . .
// . . . . . . . . . .
// . . . . . . . . . .
// # . . a . . a . . #
// . . . . . . . . . .
// . . . . . . . . . .
// . . . . . . . . . .
// . . . . . . . . . .
// . . . . . . . . . .
// (6,4) - (3,4) = (3,0)
// . . . . . . . . . .
// . . . . . . . . . .
// . . . . . . . . . .
// . . . . . . . . . .
// # . . a . . a . . #
// . . . . . . . . . .
// . . . . . . . . . .
// . . . . . . . . . .
// . . . . . . . . . .
// . . . . . . . . . .
