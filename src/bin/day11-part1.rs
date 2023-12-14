use std::fmt::Display;
use std::fs::read_to_string;

use glam::IVec2;
use itertools::Itertools;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Empty,
    Galaxy,
}
impl From<char> for Tile {
    fn from(value: char) -> Self {
        match value {
            '#' => Tile::Galaxy,
            '.' => Tile::Empty,
            _ => unimplemented!(),
        }
    }
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Tile::Empty => '.',
                Tile::Galaxy => '#',
            }
        )
    }
}

struct GalaxyMap {
    galaxies: Vec<IVec2>,
    width: usize,
    height: usize,
}
impl GalaxyMap {
    fn new(tiles: Vec<Vec<Tile>>) -> Self {
        let width = tiles[0].len();
        let height = tiles.len();
        Self {
            galaxies: tiles
                .iter()
                .enumerate()
                .flat_map(|(row_i, row)| {
                    row.iter()
                        .enumerate()
                        .filter(|(_, tile)| **tile == Tile::Galaxy)
                        .map(move |(col_i, _)| (col_i as i32, row_i as i32).into())
                })
                .collect(),
            width,
            height,
        }
    }
    fn get(&self, x: usize, y: usize) -> Tile {
        let coord = IVec2::from((x as i32, y as i32));
        if self.galaxies.contains(&coord) {
            Tile::Galaxy
        } else {
            Tile::Empty
        }
    }
    fn expand(&mut self) {
        let empty_rows: Vec<i32> = (0..self.height as i32)
            .filter(|y| {
                self.galaxies
                    .iter()
                    .map(|coord| coord.y)
                    .all(|coord_y| coord_y != *y)
            })
            .collect();
        let empty_cols: Vec<i32> = (0..self.width as i32)
            .filter(|x| {
                self.galaxies
                    .iter()
                    .map(|coord| coord.x)
                    .all(|coord_x| coord_x != *x)
            })
            .collect();
        for y in empty_rows.into_iter().rev() {
            self.galaxies
                .iter_mut()
                .filter(|coord| coord.y > y)
                .for_each(|coord| coord.y += 1);
            self.height += 1;
        }
        for x in empty_cols.into_iter().rev() {
            self.galaxies
                .iter_mut()
                .filter(|coord| coord.x > x)
                .for_each(|coord| coord.x += 1);
            self.width += 1;
        }
    }
    fn length(&self, galaxy_id1: usize, galaxy_id2: usize) -> i32 {
        let coord1 = self.galaxies[galaxy_id1];
        let coord2 = self.galaxies[galaxy_id2];
        (coord1.x - coord2.x).abs() + (coord1.y - coord2.y).abs()
    }
}
impl Display for GalaxyMap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                write!(f, "{}", self.get(x, y))?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

fn parse(input: &str) -> GalaxyMap {
    GalaxyMap::new(
        input
            .lines()
            .map(|line| line.chars().map(Tile::from).collect())
            .collect(),
    )
}
fn part1(mut galaxy_map: GalaxyMap) -> i32 {
    galaxy_map.expand();
    (0..galaxy_map.galaxies.len())
        .combinations(2)
        .map(|pair| galaxy_map.length(pair[0], pair[1]))
        .sum()
}

fn main() {
    let input = read_to_string("inputs/day11-input1.txt").unwrap();
    let map = parse(&input);
    let answer = part1(map);
    println!("answer is: {answer}");
}
#[cfg(test)]
mod tests {

    use super::*;

    const INPUT: &str = r#"
...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....
"#;
    const INPUT_EXPANDED: &str = r#"
....#........
.........#...
#............
.............
.............
........#....
.#...........
............#
.............
.............
.........#...
#....#.......
"#;
    #[test]
    fn parsing() {
        let galaxy_map = parse(INPUT.trim());
        assert_eq!(galaxy_map.get(3, 0), Tile::Galaxy);
    }
    #[test]
    fn expansion() {
        let mut galaxy_map = parse(INPUT.trim());
        galaxy_map.expand();
        assert_eq!(format!("{}", galaxy_map), INPUT_EXPANDED.trim_start());
    }
    #[test]
    fn length() {
        let mut galaxy_map = parse(INPUT.trim());
        galaxy_map.expand();
        assert_eq!(galaxy_map.length(5 - 1, 9 - 1), 9);
        assert_eq!(galaxy_map.length(1 - 1, 7 - 1), 15);
        assert_eq!(galaxy_map.length(3 - 1, 6 - 1), 17);
        assert_eq!(galaxy_map.length(8 - 1, 9 - 1), 5);
    }
    #[test]
    fn part1_test() {
        let map = parse(INPUT.trim());
        assert_eq!(part1(map), 374);
    }
}
