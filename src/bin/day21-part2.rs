use std::collections::HashSet;
use std::fs::read_to_string;
use std::i64;
use std::iter::successors;

use glam::I64Vec2;
use itertools::Itertools;

fn parse(input: &str) -> (I64Vec2, HashSet<I64Vec2>, usize) {
    let size = input.lines().count();
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
    for y in -4 * size..5 * size {
        for x in -4 * size..5 * size {
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
fn plots(
    start_pos: I64Vec2,
    stones: &HashSet<I64Vec2>,
    size: usize,
    steps: usize,
) -> HashSet<I64Vec2> {
    successors(Some(HashSet::from([start_pos])), |acc| {
        Some(
            acc.iter()
                .flat_map(|s| {
                    [(0, -1), (0, 1), (-1, 0), (1, 0)]
                        .into_iter()
                        .map(move |dir| *s + I64Vec2::from(dir))
                        .filter(|next| {
                            !stones
                                .contains(&next.rem_euclid(I64Vec2::new(size as i64, size as i64)))
                        })
                })
                .collect(),
        )
    })
    .nth(steps)
    .unwrap()

    /* uncomment to see pattern
    for i in -((steps / size) as i64) .. (steps / size) as i64 {
        let counts = res
            .iter()
            .filter(|o| o.y < (i + 1) * size && o.y >= i * size)
            .count();
        println!("count in {i}*size..{}*size = {counts}", i + 1);
    }
    /*
     *
     */
    only middle, 1 above && 1 below fields (size x size) are different from previous mod size iteration
    other fields are just shifted up and down
    */
}

fn part2(start_pos: I64Vec2, stones: &HashSet<I64Vec2>, size: usize, steps: usize) -> usize {
    let ((i, sum_so_far, mut middle_fields), diffs) = (steps % size..)
        .step_by(size)
        .map(|i| {
            let plots = plots(start_pos, stones, size, i);
            let size = size as i64;
            (
                i,
                plots.len(),
                // getting 3 fields [middle.y-1, middle.y, middle.y+1]
                [
                    plots.iter().filter(|o| o.y < 0 && o.y >= -size).count(),
                    plots.iter().filter(|o| o.y < size && o.y >= 0).count(),
                    plots
                        .iter()
                        .filter(|o| o.y < 2 * size && o.y >= size)
                        .count(),
                ],
            )
        })
        .tuple_windows()
        // calculating difference from previous middle count
        .map(|(a, b)| (b, [b.2[0] - a.2[1], b.2[1] - a.2[1], b.2[2] - a.2[1]]))
        .tuple_windows()
        // find when calculated difference is stable across iterations
        .find_map(|(a, b)| (a.1 == b.1).then_some(b))
        .unwrap();
    let mut prev_middle = middle_fields[1];
    (i..steps).step_by(size).fold(sum_so_far, |mut acc, _| {
        middle_fields[0] = diffs[0] + prev_middle;
        middle_fields[1] = diffs[1] + prev_middle;
        middle_fields[2] = diffs[2] + prev_middle;
        acc += middle_fields[0] + middle_fields[1] + middle_fields[2] - prev_middle;
        prev_middle = middle_fields[1];
        acc
    })
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
        assert!(stones.contains(&I64Vec2::new(16 % size as i64, 1 % size as i64)));
        assert!(stones.contains(&I64Vec2::new(16 % size as i64, 12 % size as i64)));
    }
    #[test]
    fn part2_test() {
        let (start_pos, stones, size) = parse(INPUT.trim());
        assert_eq!(part2(start_pos, &stones, size, 100), 6536);
        assert_eq!(part2(start_pos, &stones, size, 500), 167004);
        assert_eq!(part2(start_pos, &stones, size, 1000), 668697);
        assert_eq!(part2(start_pos, &stones, size, 5000), 16733044);
    }
}
