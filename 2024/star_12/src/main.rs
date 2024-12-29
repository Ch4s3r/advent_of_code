use aoc_parse::parser;
use aoc_parse::prelude::*;
use itertools::{Itertools, Positions};
use polars::export::num::ToPrimitive;
use std::collections::HashSet;
use std::fs::read_to_string;
use std::time::Duration;
use std::{fmt, thread};

fn main() {
    let contents =
        read_to_string("input.txt").expect("Should have been able to read the file");
    let parser = parser!(lines(any_char+));
    let input = parser.parse(&contents).unwrap();
    let mut map = Vec2d::new(input);

    let mut starting_position = (0, 0);
    for x in 0..map.width {
        for y in 0..map.height {
            if *map.index(x, y).unwrap() == '^' {
                starting_position = (x, y);
            }
        }
    }
    *map.index_mut(starting_position.0, starting_position.1)
        .unwrap() = '.';
    let mut score = 0;
    for obstacle_x in 0..map.width {
        for obstacle_y in 0..map.height {
            if obstacle_x == starting_position.0 && obstacle_y == starting_position.1 {
                continue
            }
            let mut visited_corners: HashSet<(i64, i64, char)> = HashSet::new();
            let mut map = map.clone();
            *map.index_mut(obstacle_x, obstacle_y).unwrap() = 'O';
            let mut position = starting_position;
            let mut direction = '^';
            loop {
                let (next_x, next_y, next_field) =
                    get_next_field(&map, direction, position.0, position.1);

                // dbg!(next_x, next_y, next_field);
                // let mut map_dbg = map.clone();
                // *map_dbg.index_mut(position.0,position.1).unwrap() = direction;
                // dbg!(&map_dbg);
                // thread::sleep(Duration::from_millis(100));

                if next_field == '.' {
                    position = (next_x, next_y);
                }
                if next_field == '#' || next_field == 'O' {
                    if visited_corners.contains(&(position.0, position.1, direction)) {
                        score += 1;
                        break;
                    }
                    visited_corners.insert((position.0, position.1, direction));
                    direction = turn_right(direction);
                }
                if next_field == 'e' {
                    break;
                }
            }
            // dbg!(visited_corners.len());
        }
    }
    dbg!(score);
}

fn turn_right(direction: char) -> char {
    match direction {
        '^' => '>',
        'v' => '<',
        '<' => '^',
        '>' => 'v',
        _ => panic!("direction not found"),
    }
}

fn get_next_field(map: &Vec2d, direction: char, x: i64, y: i64) -> (i64, i64, char) {
    match direction {
        '^' => (x, y - 1, *map.index(x, y - 1).unwrap_or(&'e')),
        'v' => (x, y + 1, *map.index(x, y + 1).unwrap_or(&'e')),
        '<' => (x - 1, y, *map.index(x - 1, y).unwrap_or(&'e')),
        '>' => (x + 1, y, *map.index(x + 1, y).unwrap_or(&'e')),
        _ => panic!("direction not found"),
    }
}

#[derive(Clone)]
pub struct Vec2d {
    map: Vec<char>,
    width: i64,
    height: i64,
}

impl Vec2d {
    pub fn new(map: Vec<Vec<char>>) -> Self {
        let width = map[0].len() as i64;
        let height = map.len() as i64;
        let flattened_input = map.into_iter().flatten().collect_vec();
        assert_eq!(flattened_input.len() as i64, height * width);
        Self {
            map: flattened_input,
            height,
            width,
        }
    }

    pub fn index(&self, x: i64, y: i64) -> Option<&char> {
        if x >= self.width as i64 || y >= self.height as i64 || x < 0 || y < 0 {
            return None;
        }
        Some(&self.map[(self.width as usize * y as usize) + x as usize])
    }

    pub fn index_mut(&mut self, x: i64, y: i64) -> Option<&mut char> {
        if x >= self.width as i64 || y >= self.height as i64 || x < 0 || y < 0 {
            return None;
        }
        Some(&mut self.map[self.width as usize * y as usize + x as usize])
    }

    pub fn row(&self, y: usize) -> &[char] {
        &self.map[self.width as usize * y..((self.width as usize * y) + self.width as usize)]
    }
}
impl fmt::Debug for Vec2d {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut str = String::new();
        for i in 0..self.height {
            if i != 0 {
                str.push('\n');
            }
            str.push_str(&self.row(i as usize).iter().join(" "));
        }
        write!(f, "\n{}", str)
    }
}
