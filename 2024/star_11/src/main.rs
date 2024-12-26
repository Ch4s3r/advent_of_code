use aoc_parse::parser;
use aoc_parse::prelude::*;
use itertools::Itertools;
use std::fs::read_to_string;

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

fn get_field(input: &Vec<Vec<char>>, x: usize, y: usize) -> char {
    let Some(y) = input.get(y) else { return 'E' };
    let Some(x) = y.get(x) else { return 'E' };
    x.to_owned()
}

fn set_field(input: &mut Vec<Vec<char>>, x: usize, y: usize, char: char) {
    input[y][x] = char;
}

fn main() {
    let contents = read_to_string("test_input.txt").expect("Should have been able to read the file");
    let parser = parser!(lines(any_char+));
    let input = parser.parse(&contents).unwrap();

    let mut map = input;
    let mut score = 1;
    'gameloop: loop {
        // map.iter().for_each(|line| {
        //     line.iter().for_each(|char| print!("{}", char));
        //     println!()
        // });
        // println!("========================");
        'movement: for x in 0..map[0].iter().len() {
            for y in 0..map.iter().len() {
                let element = get_field(&map, x, y);
                match element {
                    '^' => {
                        let up = get_field(&map, x, y - 1);
                        match up {
                            '.' | 'X' => {
                                set_field(&mut map, x, y, 'X');
                                set_field(&mut map, x, y - 1, '^');
                                if up != 'X' {
                                    score += 1;
                                }
                                break 'movement;
                            }
                            '#' => {
                                set_field(&mut map, x, y, '>');
                            }
                            _ => break 'gameloop,
                        }
                    }
                    'v' => {
                        let down = get_field(&map, x, y + 1);
                        match down {
                            '.' | 'X' => {
                                set_field(&mut map, x, y, 'X');
                                set_field(&mut map, x, y + 1, 'v');
                                if down != 'X' {
                                    score += 1;
                                }
                                break 'movement;
                            }
                            '#' => {
                                set_field(&mut map, x, y, '<');
                            }
                            _ => break 'gameloop,
                        }
                    }
                    '<' => {
                        let left = get_field(&map, x - 1, y);
                        match left {
                            '.' | 'X' => {
                                set_field(&mut map, x, y, 'X');
                                set_field(&mut map, x - 1, y, '<');
                                if left  != 'X' {
                                    score += 1;
                                }
                                break 'movement;
                            }
                            '#' => {
                                set_field(&mut map, x, y, '^');
                            }
                            _ => break 'gameloop,
                        }
                    }
                    '>' => {
                        let right = get_field(&map, x + 1, y);
                        match right {
                            '.' | 'X' => {
                                set_field(&mut map, x, y, 'X');
                                set_field(&mut map, x + 1, y, '>');
                                if right != 'X' {
                                    score += 1;
                                }
                                break 'movement;
                            }
                            '#' => {
                                set_field(&mut map, x, y, 'v');
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
