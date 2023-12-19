use std::collections::HashSet;
use std::fs::read_to_string;

fn parse(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}
fn tilt(platform: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut tilted = platform.clone();
    let w = platform[0].len();
    let h = platform.len();
    let mut row_d = platform[h - 1].clone();
    let mut round_rocks_queue: HashSet<(usize, usize)> = HashSet::new();

    for r in (1..h).rev() {
        let mut row_u = platform[r - 1].clone();
        for c in 0..w {
            match (row_d[c], row_u[c]) {
                ('O', '.') => {
                    let to_remove = round_rocks_queue
                        .iter()
                        .filter(|(_, col)| c == *col)
                        .copied()
                        .max_by_key(|(r, _)| *r);
                    if let Some(key) = to_remove {
                        round_rocks_queue.remove(&key);
                        tilted[key.0][key.1] = '.';
                        row_d[c] = 'O';
                        row_u[c] = 'O';
                        round_rocks_queue.insert((r, c));
                    } else {
                        row_d[c] = '.';
                        row_u[c] = 'O'
                    }
                }
                ('O', 'O') => {
                    round_rocks_queue.insert((r, c));
                }
                ('O', '#') => {
                    round_rocks_queue.retain(|key| key.1 != c);
                }

                _ => (),
            }
        }
        tilted[r] = row_d.clone();
        tilted[r - 1] = row_u.clone();
        row_d = row_u;
    }
    tilted
}
fn part1(platform: &[Vec<char>]) -> usize {
    let h = platform.len();
    platform
        .iter()
        .enumerate()
        .map(|(i, row)| row.iter().filter(|ch| **ch == 'O').count() * (h - i))
        .sum()
}
fn _print(platform: &[Vec<char>]) {
    for row in platform.iter() {
        for t in row.iter() {
            print!("{t}");
        }
        println!();
    }
    println!();
}
fn main() {
    let input = read_to_string("inputs/day14-input1.txt").unwrap();
    let rocks = parse(&input);
    let tilted = tilt(rocks);
    let answer = part1(&tilted);
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
    #[test]
    fn parsing() {
        let rocks = parse(INPUT.trim());
        assert_eq!(rocks[0][0], 'O');
        assert_eq!(rocks[0][5], '#');
        assert_eq!(rocks[2][0], '.');
    }
    #[test]
    fn tilt_test() {
        let rocks = parse(INPUT.trim());
        let tilted = tilt(rocks);
        assert_eq!(tilted, parse(TILTED.trim()));
    }
    #[test]
    fn part1_test() {
        let rocks = parse(INPUT.trim());
        let tilted = tilt(rocks);
        assert_eq!(part1(&tilted), 136);
    }
}
