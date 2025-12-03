#[derive(Debug, Clone, PartialEq)]
pub struct Command {
    pub direction: Direction,
    pub distance: u32,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Direction {
    Left,
    Right,
}

pub fn parse_command_line(line: &str) -> Option<Command> {
    let line = line.trim();
    if line.is_empty() {
        return None;
    }

    let first_char = line.chars().next()?;
    let direction = match first_char {
        'L' => Direction::Left,
        'R' => Direction::Right,
        _ => return None,
    };

    let distance_str = &line[1..];
    let distance = distance_str.parse::<u32>().ok()?;

    Some(Command {
        direction,
        distance,
    })
}

pub fn parse_commands(input: &str) -> Vec<Command> {
    input.lines().filter_map(parse_command_line).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_input_with_nom() {
        let input =
            std::fs::read_to_string("input_test.txt").expect("Failed to read input_test.txt");
        let commands = parse_commands(input.trim());

        assert_eq!(commands.len(), 10);
        assert_eq!(
            commands[0],
            Command {
                direction: Direction::Left,
                distance: 68
            }
        );
        assert_eq!(
            commands[1],
            Command {
                direction: Direction::Left,
                distance: 30
            }
        );
        assert_eq!(
            commands[2],
            Command {
                direction: Direction::Right,
                distance: 48
            }
        );
    }
}
