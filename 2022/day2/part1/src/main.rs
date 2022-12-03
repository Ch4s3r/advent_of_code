use std::str::FromStr;

use anyhow::Result;
use itertools::Itertools;
use nom::bytes::complete::tag;
use nom::character::complete::{alpha1, newline};
use nom::multi::separated_list1;
use nom::sequence::tuple;
use nom::IResult;

use crate::HandType::{Paper, Rock, Scissor};

#[derive(Debug, PartialEq)]
struct Game {
    opponent: HandType,
    player: HandType,
}

#[derive(Debug, PartialEq)]
enum HandType {
    Rock,
    Paper,
    Scissor,
}

impl FromStr for HandType {
    type Err = ();

    fn from_str(input: &str) -> std::result::Result<Self, Self::Err> {
        match input {
            "A" | "X" => Ok(Rock),
            "B" | "Y" => Ok(Paper),
            "C" | "Z" => Ok(Scissor),
            _ => Err(()),
        }
    }
}

fn main() -> Result<()> {
    let input = include_str!("../data/input_test.txt");
    app(input)?;
    Ok(())
}

fn app(input: &str) -> Result<i32> {
    let games = parse_input(input).unwrap().1;
    let points = dbg!(games
        .iter()
        .map(|game| game.calculate_points())
        .collect_vec());
    let total_points: i32 = points.iter().sum();
    dbg!(total_points);
    Ok(total_points)
}

fn parse_input(input: &str) -> IResult<&str, Vec<Game>> {
    separated_list1(newline, parse_game)(input)
}

fn parse_game(input: &str) -> IResult<&str, Game> {
    let (input, (opponent, _, player)) = tuple((alpha1, tag(" "), alpha1))(input)?;
    Ok((
        input,
        Game {
            opponent: HandType::from_str(opponent).unwrap(),
            player: HandType::from_str(player).unwrap(),
        },
    ))
}

impl From<&HandType> for i32 {
    fn from(handtype: &HandType) -> Self {
        match handtype {
            Rock => 1,
            Paper => 2,
            Scissor => 3,
        }
    }
}

impl Game {
    fn calculate_points(&self) -> i32 {
        let opponent_points = i32::from(&self.opponent);
        let player_points = i32::from(&self.player);

        if player_points == 3 && opponent_points == 1 {
            return player_points;
        }
        if player_points > opponent_points || player_points == 1 && opponent_points == 3 {
            return player_points + 6;
        }
        if player_points == opponent_points {
            return player_points + 3;
        }
        player_points
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() -> Result<()> {
        let input = "A Y
B X
C Z";
        let actual = app(input)?;
        assert_eq!(15, actual);
        Ok(())
    }
}
