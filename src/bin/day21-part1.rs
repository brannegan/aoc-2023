use std::collections::HashSet;
use std::fs::read_to_string;
use std::iter::successors;

use glam::IVec2;

fn parse(input: &str) -> (IVec2, HashSet<IVec2>, usize) {
    let size = input.lines().count();
    let stones: HashSet<IVec2> = input
        .lines()
        .enumerate()
        .flat_map(|(r, line)| {
            line.chars()
                .enumerate()
                .filter(|(_, ch)| matches!(ch, '#'))
                .map(move |(c, _)| IVec2::new(c as i32, r as i32))
        })
        .collect();
    let start_pos = input
        .lines()
        .enumerate()
        .find_map(|(r, line)| {
            line.chars()
                .enumerate()
                .find(|(_, ch)| matches!(ch, 'S'))
                .map(move |(c, _)| IVec2::new(c as i32, r as i32))
        })
        .expect("find S position");
    (start_pos, stones, size)
}
fn _print_field(stones: &HashSet<IVec2>, plots: Vec<IVec2>, size: usize) {
    for y in 0..size {
        for x in 0..size {
            if stones.contains(&IVec2::new(x as i32, y as i32)) {
                print!("#");
            } else if plots.contains(&IVec2::new(x as i32, y as i32)) {
                print!("O");
            } else {
                print!(".");
            }
        }
        println!();
    }
}
fn part1(start_pos: IVec2, stones: HashSet<IVec2>, steps: usize) -> usize {
    successors(Some(HashSet::from([start_pos])), |acc| {
        Some(
            acc.iter()
                .flat_map(|s| {
                    [(0, -1), (0, 1), (-1, 0), (1, 0)]
                        .into_iter()
                        .map(move |dir| *s + IVec2::from(dir))
                        .filter(|next| !stones.contains(next))
                })
                .collect(),
        )
    })
    .nth(steps)
    .unwrap()
    .len()
}
fn main() {
    let input = read_to_string("inputs/day21-input1.txt").unwrap();
    let (start_pos, stones, _size) = parse(&input);
    let answer = part1(start_pos, stones, 64);
    println!("answer is: {answer}");
}
#[cfg(test)]
mod tests {

    use super::*;

    const INPUT: &str = r#"
...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........
"#;
    #[test]
    fn parsing() {
        let (start_pos, stones, size) = parse(INPUT.trim());
        assert_eq!(start_pos, IVec2::new(5, 5));
        assert!(stones.contains(&IVec2::new(1, 2)));
        assert_eq!(size, 11);
    }
    #[test]
    fn part1_test() {
        let (start_pos, stones, _size) = parse(INPUT.trim());
        assert_eq!(part1(start_pos, stones, 6), 16);
    }
}
