use crate::parser::{
    Direction::{Left, Right},
    parse_commands,
};

mod parser;

fn main() {
    let filename = std::env::args()
        .nth(1)
        .expect("Please provide a filename as argument");
    let input = std::fs::read_to_string(&filename).expect("Failed to read file");
    let commands = parse_commands(input.trim());
    let count = calculate_position_count(commands);
    println!("Count of times reached position 0: {}", count);
}

fn calculate_position_count(commands: Vec<parser::Command>) -> u32 {
    let mut count = 0;
    let mut position = 50u32;
    for command in commands {
        match command.direction {
            Left => {
                for _ in 0..command.distance {
                    match position {
                        0 => {
                            position = 99;
                        }
                        1 => {
                            position = 0;
                            count += 1;
                        }
                        _ => {
                            position -= 1;
                        }
                    }
                }
            }
            Right => {
                for _ in 0..command.distance {
                    match position {
                        99 => {
                            position = 0;
                            count += 1;
                        }
                        _ => {
                            position += 1;
                        }
                    }
                }
            }
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_left_3_times_over_zero() {
        let commands = vec![parser::Command {
            direction: Left,
            distance: 260,
        }];
        let count = calculate_position_count(commands);
        assert_eq!(count, 3);
    }

    #[test]
    fn test_left_250_onto_0() {
        let commands = vec![parser::Command {
            direction: Left,
            distance: 250,
        }];
        let count = calculate_position_count(commands);
        assert_eq!(count, 3);
    }

    #[test]
    fn test_left_240() {
        let commands = vec![parser::Command {
            direction: Left,
            distance: 240,
        }];
        let count = calculate_position_count(commands);
        assert_eq!(count, 2);
    }

    #[test]
    fn test_right() {
        let commands = vec![parser::Command {
            direction: Right,
            distance: 1060,
        }];
        let count = calculate_position_count(commands);
        assert_eq!(count, 11);
    }

    #[test]
    fn test_right_big() {
        let commands = vec![
            parser::Command {
                direction: Left,
                distance: 42,
            },
            parser::Command {
                direction: Right,
                distance: 919,
            },
        ];
        let count = calculate_position_count(commands);
        assert_eq!(count, 9);
    }

    #[test]
    fn test_right_31_by_44_to_75() {
        let commands = vec![
            parser::Command {
                direction: Left,
                distance: 19,
            },
            parser::Command {
                direction: Right,
                distance: 44,
            },
        ];
        let count = calculate_position_count(commands);
        assert_eq!(count, 0);
    }
}
