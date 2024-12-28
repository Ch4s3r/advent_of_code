#![feature(hash_set_entry)]

use aoc_parse::macros::lines;
use aoc_parse::prelude::{any_char, digit, line};
use aoc_parse::{Parser, parser};
use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::fmt;
use std::fs::read_to_string;
use std::ops::Sub;

fn main() {
    let contents =
        read_to_string("input.txt").expect("Should have been able to read the file");
    let parser = parser!(lines(any_char+));
    let input = parser.parse(&contents).unwrap();
    let mut map = Vec2d::new(input);

    let mut trailhead: HashMap<(i64, i64), i64> = HashMap::new();

    for x in 0..map.width as i64 {
        for y in 0..map.height as i64 {
            if *map.index(x, y).unwrap_or(&'.') == '0' {
                get_trails(&map, x, y, x, y, 1, &mut trailhead);
            }
        }
    }
    dbg!(&trailhead);
    dbg!(trailhead.iter().map(|x| {x.1}).sum::<i64>());
}

fn get_trails(
    map: &Vec2d,
    start_x: i64,
    start_y: i64,
    x: i64,
    y: i64,
    height: i64,
    trailhead: &mut HashMap<(i64, i64), i64>,
) {
    let height_char = height.to_string().chars().nth(0).unwrap();
    if *map.index(x, y).unwrap_or(&'.') == '9' {
        *trailhead.entry((start_x, start_y)).or_insert(0) += 1;
        return;
    }
    if *map.index(x - 1, y).unwrap_or(&'.') == height_char {
        get_trails(map, start_x, start_y, x - 1, y, height + 1, trailhead);
    }
    if *map.index(x + 1, y).unwrap_or(&'.') == height_char {
        get_trails(map, start_x, start_y, x + 1, y, height + 1, trailhead);
    }
    if *map.index(x, y - 1).unwrap_or(&'.') == height_char {
        get_trails(map, start_x, start_y, x, y - 1, height + 1, trailhead);
    }
    if *map.index(x, y + 1).unwrap_or(&'.') == height_char {
        get_trails(map, start_x, start_y, x, y + 1, height + 1, trailhead);
    }
}

#[derive(Clone)]
pub struct Vec2d {
    map: Vec<char>,
    width: usize,
    height: usize,
}

impl Vec2d {
    pub fn new(map: Vec<Vec<char>>) -> Self {
        let width = map[0].len();
        let height = map.len();
        let flattened_input = map.into_iter().flatten().collect_vec();
        assert_eq!(flattened_input.len(), height * width);
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
        Some(&self.map[(self.width * y as usize) + x as usize])
    }

    pub fn index_mut(&mut self, x: i64, y: i64) -> Option<&mut char> {
        if x >= self.width as i64 || y >= self.height as i64 || x < 0 || y < 0 {
            return None;
        }
        Some(&mut self.map[self.width * y as usize + x as usize])
    }

    pub fn row(&self, y: usize) -> &[char] {
        &self.map[self.width * y..((self.width * y) + self.width)]
    }
}
impl fmt::Debug for Vec2d {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut str = String::new();
        for i in 0..self.height {
            if i != 0 {
                str.push('\n');
            }
            str.push_str(&self.row(i).iter().join(" "));
        }
        write!(f, "\n{}", str)
    }
}
