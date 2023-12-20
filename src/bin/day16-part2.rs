use std::collections::HashSet;
use std::fmt::Display;
use std::fs::read_to_string;

use indicatif::ProgressIterator;

#[derive(Debug, Clone, Default)]
struct BeamMap {
    layout: Vec<Vec<char>>,
    energy_map: Vec<Vec<bool>>,
    size: usize,
    known_directions: HashSet<((usize, usize), (i32, i32))>,
}

impl Display for BeamMap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.energy_map.iter() {
            for c in row.iter() {
                let ch = if *c { '#' } else { '.' };
                write!(f, "{ch}")?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl BeamMap {
    fn launch_beam(&mut self, mut pos: (usize, usize), mut dir: (i32, i32)) {
        self.known_directions.insert((pos, dir));
        loop {
            self.energy_map[pos.0][pos.1] = true;
            match self.layout[pos.0][pos.1] {
                '|' => {
                    if dir.0 == 0 {
                        if !self.known_directions.contains(&(pos, (-1, 0))) {
                            self.launch_beam(pos, (-1, 0));
                        }
                        dir = (1, 0);
                    }
                }
                '-' => {
                    if dir.1 == 0 {
                        if !self.known_directions.contains(&(pos, (0, -1))) {
                            self.launch_beam(pos, (0, -1));
                        }
                        dir = (0, 1);
                    }
                }
                '\\' => {
                    if self.known_directions.contains(&(pos, dir)) {
                        break;
                    }
                    self.known_directions.insert((pos, dir));
                    dir = (dir.1, dir.0);
                }
                '/' => {
                    if self.known_directions.contains(&(pos, dir)) {
                        break;
                    }
                    self.known_directions.insert((pos, dir));
                    dir = (-dir.1, -dir.0);
                }
                '.' => {}
                _ => unimplemented!("no other objects on map"),
            };
            let new_pos = (pos.0 as i32 + dir.0, pos.1 as i32 + dir.1);
            if new_pos.0 < 0
                || new_pos.1 < 0
                || new_pos.0 >= self.size as i32
                || new_pos.1 >= self.size as i32
            {
                break;
            }
            pos = (new_pos.0 as usize, new_pos.1 as usize);
        }
    }
    fn power(&self) -> usize {
        self.energy_map
            .iter()
            .flat_map(|v| v.iter())
            .filter(|e| **e)
            .count()
    }
    fn reset(&mut self) {
        self.energy_map = vec![vec![false; self.size]; self.size];
        self.known_directions.clear();
    }
}

fn parse(input: &str) -> BeamMap {
    let layout: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let size = layout.len();
    let energy_map = vec![vec![false; size]; size];
    BeamMap {
        layout,
        energy_map,
        size,
        ..Default::default()
    }
}
fn part2(beammap: &mut BeamMap) -> usize {
    let mut powers = Vec::with_capacity(beammap.size * 4);
    for i in (0..beammap.size).progress() {
        beammap.launch_beam((i, 0), (0, 1));
        powers.push(beammap.power());
        beammap.reset();

        beammap.launch_beam((0, i), (1, 0));
        powers.push(beammap.power());
        beammap.reset();

        beammap.launch_beam((i, beammap.size - 1), (0, -1));
        powers.push(beammap.power());
        beammap.reset();

        beammap.launch_beam((beammap.size - 1, i), (-1, 0));
        powers.push(beammap.power());
        beammap.reset();
    }
    powers.into_iter().max().unwrap()
}

fn main() {
    let input = read_to_string("inputs/day16-input1.txt").unwrap();
    let mut beammap = parse(&input);
    let answer = part2(&mut beammap);
    println!("answer is: {answer}");
}
#[cfg(test)]
mod tests {

    use super::*;

    const INPUT: &str = r#"
.|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|....
"#;
    const ENERGIZED: &str = r#"
.#####....
.#.#.#....
.#.#.#####
.#.#.##...
.#.#.##...
.#.#.##...
.#.#####..
########..
.#######..
.#...#.#..
"#;

    #[test]
    fn parsing() {
        let beammap = parse(INPUT.trim());
        assert_eq!(beammap.layout[0][1], '|');
        assert_eq!(beammap.layout[0][5], '\\');
    }
    #[test]
    fn launch_beam() {
        let mut beammap = parse(INPUT.trim());
        beammap.launch_beam((0, 3), (1, 0));
        println!("{beammap}");
        assert_eq!(format!("{beammap}").trim(), ENERGIZED.trim());
    }
    #[test]
    fn part2_test() {
        let mut beammap = parse(INPUT.trim());
        assert_eq!(part2(&mut beammap), 51);
    }
}
