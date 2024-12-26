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
    let contents = read_to_string("test_input.txt").expect("Should have been able to read the file");
    let parser = parser!(lines(any_char+));
    let input = parser.parse(&contents).unwrap();

    let starting_map = input;
    let width = starting_map[0].iter().len();
    let height = starting_map.iter().len();
    let mut score = 0;
    let mut map = starting_map;

    for obstacle_x in 0..width {
        for obstacle_y in 0..height {
            let mut map = map.clone();
            let field = get_field(&map, obstacle_x, obstacle_y);
            if field == '#' {
                continue;
            }
            if !"^v<>".contains(field) {
                set_field(&mut map, obstacle_x, obstacle_y, 'O');
            }

            'gameloop: loop {
                // map.iter().for_each(|line| {
                //     line.iter().for_each(|char| print!("{}", char));
                //     println!()
                // });
                // println!("========================");
                // thread::sleep(Duration::from_millis(500));
                'movement: for x in 0..width {
                    for y in 0..height {
                        let element = get_field(&map, x, y);
                        match element {
                            '^' => {
                                if (y as i64 -1) <= 0 {
                                    continue
                                }
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
                                if (x as i64 -1) <= 0 {
                                    continue
                                }
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
