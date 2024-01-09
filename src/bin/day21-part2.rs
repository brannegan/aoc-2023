use std::collections::HashSet;
use std::fs::read_to_string;
use std::iter::successors;

use glam::I64Vec2;

fn parse(input: &str) -> (I64Vec2, HashSet<I64Vec2>, i64) {
    let size = input.lines().count() as i64;
    let stones: HashSet<I64Vec2> = input
        .lines()
        .enumerate()
        .flat_map(|(r, line)| {
            line.chars()
                .enumerate()
                .filter(|(_, ch)| matches!(ch, '#'))
                .map(move |(c, _)| I64Vec2::new(c as i64, r as i64))
        })
        .collect();
    let start_pos = input
        .lines()
        .enumerate()
        .find_map(|(r, line)| {
            line.chars()
                .enumerate()
                .find(|(_, ch)| matches!(ch, 'S'))
                .map(move |(c, _)| I64Vec2::new(c as i64, r as i64))
        })
        .expect("find S(start) position");
    (start_pos, stones, size)
}
fn _print_field(stones: &HashSet<I64Vec2>, plots: &HashSet<I64Vec2>, size: i64) {
    for y in -2 * size..3 * size {
        for x in -2 * size..3 * size {
            if stones.contains(&I64Vec2::new(x, y).rem_euclid(I64Vec2::new(size, size))) {
                print!("#");
            } else if plots.contains(&I64Vec2::new(x, y)) {
                print!("O");
            } else {
                print!(".");
            }
        }
        println!();
    }
}
fn part2(start_pos: I64Vec2, stones: &HashSet<I64Vec2>, size: i64, steps: usize) -> usize {
    successors(Some(HashSet::from([start_pos])), |acc| {
        Some(
            acc.iter()
                .flat_map(|s| {
                    [(0, -1), (0, 1), (-1, 0), (1, 0)]
                        .into_iter()
                        .map(move |dir| *s + I64Vec2::from(dir))
                        .filter(|next| !stones.contains(&next.rem_euclid(I64Vec2::new(size, size))))
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
    let (start_pos, stones, size) = parse(&input);
    let answer = part2(start_pos, &stones, size, 26501365);
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
        assert_eq!(start_pos, I64Vec2::new(5, 5));
        assert_eq!(size, 11);
        assert!(stones.contains(&I64Vec2::new(1, 2)));
        assert!(stones.contains(&I64Vec2::new(16 % size, 1 % size)));
        assert!(stones.contains(&I64Vec2::new(16 % size, 12 % size)));
    }
    #[test]
    fn part2_test() {
        let (start_pos, stones, size) = parse(INPUT.trim());
        assert_eq!(part2(start_pos, &stones, size, 6), 16);
        assert_eq!(part2(start_pos, &stones, size, 50), 1594);
        assert_eq!(part2(start_pos, &stones, size, 100), 6536);
        assert_eq!(part2(start_pos, &stones, size, 500), 167004);
        //assert_eq!(part2(start_pos, &stones, size, 1000), 668697);
        //assert_eq!(part2(start_pos, &stones, size, 5000), 16733044);
    }
}
