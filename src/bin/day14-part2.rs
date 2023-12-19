use std::collections::HashSet;
use std::fmt::Display;
use std::fs::read_to_string;

use glam::IVec2;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Dir(IVec2);
const NORTH: Dir = Dir(IVec2::Y);
const SOUTH: Dir = Dir(IVec2::NEG_Y);
const WEST: Dir = Dir(IVec2::X);
const EAST: Dir = Dir(IVec2::NEG_X);

fn part2(platform: Platform) -> i32 {
    platform.spin_1_000_000_000().total_load()
}
#[derive(Debug, Clone, Default, PartialEq, Eq)]
struct Platform {
    round_rocks: HashSet<IVec2>,
    cube_rocks: HashSet<IVec2>,
    size: i32,
}

impl Platform {
    fn total_load(&self) -> i32 {
        self.round_rocks
            .iter()
            .map(|coord| self.size - coord.y)
            .sum()
    }
    fn tilt(mut self, dir: Dir) -> Self {
        let mut rolled_rocks = HashSet::new();
        let mut roll = |x, y, next_pos: &mut IVec2| {
            if self.cube_rocks.contains(&IVec2::new(x, y)) {
                *next_pos = IVec2::new(x, y) + dir.0;
            }
            if self.round_rocks.contains(&IVec2::new(x, y)) {
                rolled_rocks.insert(*next_pos);
                *next_pos += dir.0
            }
        };

        for x in 0..self.size {
            match dir {
                NORTH => {
                    let mut next_pos = IVec2::new(x, 0);
                    for y in 0..self.size {
                        roll(x, y, &mut next_pos)
                    }
                }
                SOUTH => {
                    let mut next_pos = IVec2::new(x, self.size - 1);
                    for y in (0..self.size).rev() {
                        roll(x, y, &mut next_pos)
                    }
                }

                WEST => {
                    let mut next_pos = IVec2::new(0, x);
                    for y in 0..self.size {
                        roll(y, x, &mut next_pos)
                    }
                }
                EAST => {
                    let mut next_pos = IVec2::new(self.size - 1, x);
                    for y in (0..self.size).rev() {
                        roll(y, x, &mut next_pos)
                    }
                }
                _ => {
                    unimplemented!("no other direction yet")
                }
            };
        }
        self.round_rocks = rolled_rocks;
        self
    }
    fn cycle(self) -> Self {
        self.tilt(NORTH).tilt(WEST).tilt(SOUTH).tilt(EAST)
    }
    fn spin_1_000_000_000(mut self) -> Self {
        let mut vec_of_round_rocks = vec![self.round_rocks.clone()];
        let mut cycles = 0;
        let repeat_cycle = loop {
            self = self.cycle();
            cycles += 1;
            let next = self.round_rocks.clone();
            let cycle_pos = vec_of_round_rocks.iter().position(|old| old == &next);
            if let Some(pos) = cycle_pos {
                break pos;
            }
            vec_of_round_rocks.push(next);
        };
        for _ in 0..(1_000_000_000 - cycles) % (cycles - repeat_cycle) {
            self = self.cycle()
        }
        self
    }
}

impl Display for Platform {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.size {
            for x in 0..self.size {
                if self.round_rocks.contains(&IVec2::new(x, y)) {
                    write!(f, "O")?;
                } else if self.cube_rocks.contains(&IVec2::new(x, y)) {
                    write!(f, "#")?;
                } else {
                    write!(f, ".")?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

fn parse(input: &str) -> Platform {
    let lines = input.lines();
    let mut platform = Platform::default();
    platform.size = lines
        .enumerate()
        .map(|(line_i, line)| {
            line.chars()
                .enumerate()
                .for_each(|(char_i, char)| match char {
                    '#' => {
                        platform
                            .cube_rocks
                            .insert(IVec2::new(char_i as i32, line_i as i32));
                    }
                    'O' => {
                        platform
                            .round_rocks
                            .insert(IVec2::new(char_i as i32, line_i as i32));
                    }
                    _ => {}
                })
        })
        .count() as i32;
    platform
}
fn main() {
    let input = read_to_string("inputs/day14-input1.txt").unwrap();
    let platform = parse(&input);
    let answer = part2(platform);
    println!("answer is: {answer}");
}
#[cfg(test)]
mod tests {

    use super::*;

    const INPUT: &str = r#"
O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....
"#;
    const TILTED: &str = r#"
OOOO.#.O..
OO..#....#
OO..O##..O
O..#.OO...
........#.
..#....#.#
..O..#.O.O
..O.......
#....###..
#....#....
"#;
    const CYCLE_1: &str = r#"
.....#....
....#...O#
...OO##...
.OO#......
.....OOO#.
.O#...O#.#
....O#....
......OOOO
#...O###..
#..OO#....
"#;
    #[test]
    fn parsing() {
        let platform = parse(INPUT.trim());
        println!("{}", platform);
        assert_eq!(format!("{}", platform).trim(), INPUT.trim());
    }
    #[test]
    fn tilt_test() {
        let platform = parse(INPUT.trim());
        let tilted = platform.tilt(NORTH);
        assert_eq!(format!("{}", tilted).trim(), TILTED.trim());
    }
    #[test]
    fn cycle_1() {
        let platform = parse(INPUT.trim());
        let cycle_1 = platform.cycle();

        assert_eq!(format!("{}", cycle_1).trim(), CYCLE_1.trim());
    }
    #[test]
    fn part2_test() {
        let platform = parse(INPUT.trim());
        assert_eq!(part2(platform), 64);
    }
    #[test]
    fn total_load() {
        let platform = parse(INPUT.trim());
        let tilted = platform.tilt(NORTH);
        assert_eq!(tilted.total_load(), 136);
    }
}
