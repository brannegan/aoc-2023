use std::cmp::max;
use std::fs::read_to_string;

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{line_ending, space1, u32};
use nom::multi::separated_list1;
use nom::sequence::{preceded, separated_pair, tuple};
use nom::{Finish, Parser};

#[derive(Debug)]
struct Game {
    sets: Vec<CubeSet>,
}
impl Game {
    fn new(sets: Vec<CubeSet>) -> Self {
        Self { sets }
    }
    fn fewest_set(&self) -> CubeSet {
        self.sets
            .iter()
            .fold(Default::default(), |min, cur| CubeSet {
                red: max(cur.red, min.red),
                green: max(cur.green, min.green),
                blue: max(cur.blue, min.blue),
            })
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq)]
struct CubeSet {
    red: u32,
    blue: u32,
    green: u32,
}

#[derive(Debug)]
enum Cube {
    Red(u32),
    Green(u32),
    Blue(u32),
}

fn parse(input: &str) -> anyhow::Result<Vec<Game>> {
    let game_number = tuple((tag("Game "), u32, tag(": ")));
    let red = separated_pair(u32, space1, tag("red")).map(|(x, _)| Cube::Red(x));
    let green = separated_pair(u32, space1, tag("green")).map(|(x, _)| Cube::Green(x));
    let blue = separated_pair(u32, space1, tag("blue")).map(|(x, _)| Cube::Blue(x));
    let subset = separated_list1(tag(", "), alt((red, green, blue))).map(|cubes| {
        cubes.iter().fold(CubeSet::default(), |mut acc, cube| {
            match cube {
                Cube::Red(r) => acc.red += r,
                Cube::Green(g) => acc.green += g,
                Cube::Blue(b) => acc.blue += b,
            };
            acc
        })
    });
    let line = separated_list1(tag("; "), subset).map(Game::new);

    let mut parser = separated_list1(line_ending, preceded(game_number, line));
    parser
        .parse(input)
        .finish()
        .map(|(_, parsed)| parsed)
        .map_err(|e: nom::error::VerboseError<&str>| anyhow::anyhow!("parser error: {:?}", e))
}
fn part2(games: &[Game]) -> u32 {
    games
        .iter()
        .map(Game::fewest_set)
        .map(|set| set.red * set.green * set.blue)
        .sum()
}

fn main() {
    let input = read_to_string("inputs/day02-input1.txt").unwrap();
    let games = parse(&input).unwrap();
    let answer = part2(&games);
    println!("answer is: {answer}");
}
#[cfg(test)]
mod tests {
    use anyhow::Ok;

    use super::*;

    const INPUT: &str = r#"
Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
"#;
    #[test]
    fn part2_test() -> anyhow::Result<()> {
        let games = parse(INPUT.trim())?;
        assert_eq!(part2(&games), 2286);
        Ok(())
    }
}
