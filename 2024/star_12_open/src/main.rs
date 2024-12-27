use aoc_parse::parser;
use aoc_parse::prelude::*;
use itertools::Itertools;
use polars::export::num::ToPrimitive;
use std::fs::read_to_string;
use std::time::Duration;
use std::{fmt, thread};

#[derive(Debug)]
enum Element {
    Nothing,
    Wall,
    GuardUp,
    GuardDown,
    GuardLeft,
    GuardRight,
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn get_field(input: &Vec2d<char>, x: i64, y: i64) -> char {
    let Some(x) = x.to_usize() else { return 'E' };
    let Some(y) = y.to_usize() else { return 'E' };
    input.index(x, y).to_owned()
}

fn set_field(input: &mut Vec2d<char>, x: i64, y: i64, char: char) {
    *input.index_mut(x as usize, y as usize) = char;
}

// fn get_field(input: &str, x: usize, y: usize) -> char {
//     let width = input.chars().position(|char| { char == '\n' }).unwrap();
//     input.chars().nth(y * width + x).unwrap()
// }
//
// fn set_field(input: &mut String, x: usize, y: usize, char: char) {
//     let width = input.chars().position(|char| { char == '\n' }).unwrap();
//     let index = y * width + x;
//     input.replace_range(index..=index, &char.to_string());
// }

// fn get_field(input: &Vec<char>, x: usize, y: usize) -> char {
//     dbg!(input);
//     let width = input.iter().position(|&char| char == '\n').unwrap();
//     let index = y * width + x;
//     *input.get(index).unwrap()
// }
//
// fn set_field(input: &mut Vec<char>, x: usize, y: usize, char: char) {
//     // input = char;
// }

fn main() {
    let contents =
        read_to_string("input.txt").expect("Should have been able to read the file");
    let parser = parser!(lines(any_char+));
    let input = parser.parse(&contents).unwrap();

    let starting_map = input;
    let width = starting_map[0].iter().len();
    let height = starting_map.iter().len();
    let mut score = 0;
    let mut map = Vec2d::new(
        starting_map.into_iter().flatten().collect_vec(),
        width,
        height,
    );

    for obstacle_x in 0..width as i64 {
        for obstacle_y in 0..height as i64 {
            let mut map = map.clone();
            let field = get_field(&map, obstacle_x, obstacle_y);
            if "#^".contains(field) {
                continue;
            }
            set_field(&mut map, obstacle_x, obstacle_y, 'O');

            'gameloop: loop {
                // map.iter().for_each(|line| {
                //     line.iter().for_each(|char| print!("{}", char));
                //     println!()
                // });
                // println!("{map}\n========================");
                // thread::sleep(Duration::from_millis(100));
                'movement: for x in 0..width as i64 {
                    for y in 0..height as i64 {
                        let element = get_field(&map, x, y);
                        match element {
                            '^' => {
                                let up = get_field(&map, x, y - 1);
                                match up {
                                    '.' | '|' | '-' => {
                                        set_field(&mut map, x, y, '|');
                                        set_field(&mut map, x, y - 1, '^');
                                        break 'movement;
                                    }
                                    '#' | 'O' => {
                                        set_field(&mut map, x, y, '+');
                                        set_field(&mut map, x + 1, y, '>');
                                    }
                                    '+' => {
                                        score += 1;
                                        break 'gameloop;
                                    }
                                    _ => break 'gameloop,
                                }
                            }
                            'v' => {
                                let down = get_field(&map, x, y + 1);
                                match down {
                                    '.' | '|' | '-' => {
                                        set_field(&mut map, x, y, '|');
                                        set_field(&mut map, x, y + 1, 'v');
                                        break 'movement;
                                    }
                                    '#' | 'O' => {
                                        set_field(&mut map, x, y, '+');
                                        set_field(&mut map, x - 1, y, '<');
                                    }
                                    '+' => {
                                        score += 1;
                                        break 'gameloop;
                                    }
                                    _ => break 'gameloop,
                                }
                            }
                            '>' => {
                                let right = get_field(&map, x + 1, y);
                                match right {
                                    '.' | '|' | '-' => {
                                        set_field(&mut map, x, y, '-');
                                        set_field(&mut map, x + 1, y, '>');
                                        break 'movement;
                                    }
                                    '#' | 'O' => {
                                        set_field(&mut map, x, y, '+');
                                        set_field(&mut map, x, y + 1, 'v');
                                    }
                                    '+' => {
                                        score += 1;
                                        break 'gameloop;
                                    }
                                    _ => break 'gameloop,
                                }
                            }
                            '<' => {
                                let right = get_field(&map, x - 1, y);
                                match right {
                                    '.' | '|' | '-' => {
                                        set_field(&mut map, x, y, '-');
                                        set_field(&mut map, x - 1, y, '<');
                                        break 'movement;
                                    }
                                    '#' | 'O' => {
                                        set_field(&mut map, x, y, '+');
                                        set_field(&mut map, x, y - 1, '^');
                                    }
                                    '+' => {
                                        score += 1;
                                        break 'gameloop;
                                    }
                                    _ => break 'gameloop,
                                }
                            }
                            _ => {}
                        }
                    }
                }
            }
            println!("{}", score)
        }
    }
}

#[derive(Clone)]
pub struct Vec2d<T> {
    vec: Vec<T>,
    height: usize,
    width: usize,
}

impl Vec2d<char> {
    pub fn new(vec: Vec<char>, width: usize, height: usize) -> Self {
        assert!(vec.len() == height * width);
        Self { vec, height, width }
    }

    pub fn row(&self, y: usize) -> &[char] {
        &self.vec[self.width * y..((self.width * y) + self.width)]
    }

    pub fn index(&self, x: usize, y: usize) -> &char {
        if x >= self.width || y >= self.height {
            return &'E';
        }
        &self.vec[(self.width * y) + x]
    }

    pub fn index_mut(&mut self, x: usize, y: usize) -> &mut char {
        &mut self.vec[self.width * y + x]
    }
}
impl fmt::Display for Vec2d<char> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut str = String::new();
        for i in 0..self.height {
            if i != 0 {
                str.push('\n');
            }
            str.push_str(&self.row(i).iter().join(" "));
        }
        write!(f, "{}", str)
    }
}
