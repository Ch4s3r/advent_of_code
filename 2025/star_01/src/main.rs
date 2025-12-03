use crate::parser::{
    Direction::{Left, Right},
    parse_commands,
};

mod parser;

fn main() {
    let filename = std::env::args().nth(1).expect("Please provide a filename as argument");
    let input = std::fs::read_to_string(&filename).expect("Failed to read file");
    let commands = parse_commands(input.trim());
    let count = calculate_position_count(commands);
    println!("Count of times reached position 0: {}", count);
}

fn calculate_position_count(commands: Vec<parser::Command>) -> i32 {
    let mut count = 0;
    let mut position = 50u32;
    for command in commands {
        match command.direction {
            Left => {
                let distance = command.distance % 100;
                position = if position < distance {
                    100 - (distance - position)
                } else {
                    position - distance
                };
                println!("Moving left by {} to {}", command.distance, position);
            }
            Right => {
                let distance = command.distance % 100;
                position = if position + distance >= 100 {
                    (position + distance) - 100
                } else {
                    position + distance
                };
                println!("Moving right by {} to {}", command.distance, position);
            }
        }
        if position == 0 {
            count += 1;
            println!("Reached position 0");
        }
    }
    count
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_left_50_wraps_around() {
        let commands = vec![parser::Command {
            direction: Left,
            distance: 250,
        }];
        let count = calculate_position_count(commands);
        assert_eq!(count, 1); // 50 - 250 wraps to 0
    }
}