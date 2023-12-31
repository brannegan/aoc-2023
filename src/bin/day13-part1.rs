use std::fs::read_to_string;
use std::ops::Range;

use itertools::Itertools;

#[derive(Debug, PartialEq, Eq)]
struct Pattern {
    rows: Vec<String>,
    cols: Vec<String>,
}
impl Pattern {
    fn new(lines: Vec<&str>) -> Self {
        let rows: Vec<_> = lines.into_iter().map(String::from).collect();
        let mut cols = Vec::with_capacity(rows[0].len());
        for c in 0..rows[0].len() {
            let mut col = String::with_capacity(rows.len());
            for row in rows.iter() {
                col.push(row.as_bytes()[c] as char)
            }
            cols.push(col)
        }

        Self { rows, cols }
    }
}

fn parse(input: &str) -> Vec<Pattern> {
    input
        .split("\n\n")
        .map(|pat| pat.lines().collect())
        .map(Pattern::new)
        .collect()
}

fn mirror_pos(lines: &[String]) -> Option<usize> {
    lines
        .iter()
        .enumerate()
        .tuple_windows()
        .filter(|((_, a), (_, b))| a == b)
        .find_map(|((a_i, _), (b_i, _))| {
            let top = &lines[0..=a_i];
            let btm = &lines[b_i..];
            top.iter()
                .rev()
                .flat_map(|line| line.chars())
                .zip(btm.iter().flat_map(|line| line.chars()))
                .all(|(a, b)| a == b)
                .then_some(a_i + 1)
        })
}
fn part1(patterns: &[Pattern]) -> usize {
    patterns
        .iter()
        .map(|pat| {
            mirror_pos(&pat.rows)
                .map(|left| left * 100)
                .or_else(|| mirror_pos(&pat.cols))
                .unwrap_or(0)
        })
        .sum()
}

fn main() {
    let input = read_to_string("inputs/day13-input1.txt").unwrap();
    let patterns = parse(&input);
    let answer = part1(&patterns);
    println!("answer is: {answer}");
}
#[cfg(test)]
mod tests {

    use super::*;

    const INPUT: &str = r#"
#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#

"#;

    #[test]
    fn parsing() {
        let patterns = parse(INPUT.trim());
        assert_eq!(patterns.len(), 2);
        assert_eq!(patterns[0].rows[0], "#.##..##.");
        assert_eq!(patterns[0].cols[0], "#.##..#");
    }
    #[test]
    fn mirror_test() {
        let patterns = parse(INPUT.trim());

        assert_eq!(mirror_pos(&patterns[0].cols), Some(5));
        assert_eq!(mirror_pos(&patterns[1].rows), Some(4));
    }

    #[test]
    fn part1_test() {
        let patterns = parse(INPUT.trim());
        assert_eq!(part1(&patterns), 400);
    }
}
