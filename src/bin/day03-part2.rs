use std::fs::read_to_string;

use itertools::Itertools;
type Map = Vec<Vec<Tile>>;

#[derive(Debug, PartialEq, Eq)]
enum Tile {
    Digit(char),
    Symbol,
    Gear,
    Blank,
}

fn parse(input: &str) -> Map {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '0'..='9' => Tile::Digit(c),
                    '.' => Tile::Blank,
                    '*' => Tile::Gear,
                    _ => Tile::Symbol,
                })
                .collect::<Vec<_>>()
        })
        .collect()
}
fn gear_position(map: &Map, x: usize, y: usize) -> Option<(usize, usize)> {
    [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ]
    .iter()
    .find_map(|(dx, dy)| {
        let r = ((x as isize) + dx).clamp(0, map.len() as isize - 1) as usize;
        let c = ((y as isize) + dy).clamp(0, map[0].len() as isize - 1) as usize;
        matches!(map[r][c], Tile::Gear).then_some((r, c))
    })
}
fn part2(map: &Map) -> u32 {
    let mut possible_part_numbers = Vec::new();
    for r in 0..map.len() {
        let mut num = String::new();
        let mut gear_pos = None;
        for c in 0..map[0].len() {
            if let Tile::Digit(d) = map[r][c] {
                num.push(d);
                if gear_pos.is_none() {
                    gear_pos = gear_position(map, r, c);
                }
            } else {
                if let Some(gear_pos) = gear_pos {
                    possible_part_numbers.push((gear_pos, num.parse::<u32>().unwrap()));
                }
                gear_pos = None;
                num.clear();
            }
        }
        if let Some(gear_pos) = gear_pos {
            possible_part_numbers.push((gear_pos, num.parse::<u32>().unwrap()));
        }
    }
    possible_part_numbers
        .into_iter()
        .into_group_map()
        .iter()
        .filter(|(_, gear_numbers)| (gear_numbers.len() > 1))
        .map(|(_, gear_numbers)| gear_numbers[0] * gear_numbers[1])
        .sum()
}

fn main() {
    let input = read_to_string("inputs/day03-input1.txt").unwrap();
    let map = parse(&input);
    let answer = part2(&map);
    println!("answer is: {answer}");
}
#[cfg(test)]
mod tests {

    use super::*;

    const INPUT: &str = r#"
467..114..
...*......
..35...633
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
"#;
    #[test]
    fn parsing() {
        let map = parse(INPUT.trim());
        assert_eq!(map[0][1], Tile::Digit('6'));
        assert_eq!(map[1][3], Tile::Gear);
        assert_eq!(map[8][3], Tile::Symbol);
        assert_eq!(map[2][1], Tile::Blank);
    }
    #[test]
    fn part2_test() {
        let map = parse(INPUT.trim());
        part2(&map);
        assert_eq!(part2(&map), 467835);
    }
}
