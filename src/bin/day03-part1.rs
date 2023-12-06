use std::fs::read_to_string;
type Map = Vec<Vec<Tile>>;

#[derive(Debug, PartialEq, Eq)]
enum Tile {
    Digit(char),
    Symbol,
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
                    _ => Tile::Symbol,
                })
                .collect::<Vec<_>>()
        })
        .collect()
}
fn has_adjacent_symbol(map: &Map, x: usize, y: usize) -> bool {
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
    .any(|(dx, dy)| {
        let r = ((x as isize) + dx).clamp(0, map.len() as isize - 1);
        let c = ((y as isize) + dy).clamp(0, map[0].len() as isize - 1);
        matches!(map[r as usize][c as usize], Tile::Symbol)
    })
}
fn part1(map: &Map) -> u32 {
    let mut part_numbers: Vec<u32> = Vec::new();
    for r in 0..map.len() {
        let mut num = String::new();
        let mut is_part_number = false;
        for c in 0..map[0].len() {
            match map[r][c] {
                Tile::Digit(d) => {
                    num.push(d);
                    if has_adjacent_symbol(map, r, c) {
                        is_part_number = true;
                    }
                }
                _ => {
                    if is_part_number && !num.is_empty() {
                        part_numbers.push(num.parse().unwrap());
                    }
                    num.clear();
                    is_part_number = false;
                }
            }
        }
        if is_part_number && !num.is_empty() {
            part_numbers.push(num.parse().unwrap());
        }
    }
    part_numbers.into_iter().sum()
}

fn main() {
    let input = read_to_string("inputs/day03-input1.txt").unwrap();
    let map = parse(&input);
    let answer = part1(&map);
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
        assert_eq!(map[1][3], Tile::Symbol);
        assert_eq!(map[2][1], Tile::Blank);
    }
    #[test]
    fn part1_test() {
        let map = parse(INPUT.trim());
        assert_eq!(part1(&map), 4361);
    }
}
