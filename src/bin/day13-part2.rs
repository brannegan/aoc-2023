use std::fs::read_to_string;

use itertools::Itertools;

#[derive(Debug, Clone, PartialEq, Eq)]
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
        .filter(|((_, a), (_, b))| {
            a == b || a.chars().zip(b.chars()).filter(|(a, b)| a != b).count() <= 1
        })
        .find_map(|((a_i, _), (b_i, _))| {
            let top = &lines[0..=a_i];
            let btm = &lines[b_i..];
            (top.iter()
                .rev()
                .flat_map(|line| line.chars())
                .zip(btm.iter().flat_map(|line| line.chars()))
                .filter(|(a, b)| a != b)
                .count()
                == 1)
                .then_some(a_i + 1)
        })
}
fn part2(patterns: &[Pattern]) -> usize {
    patterns
        .iter()
        .map(|pattern| {
            mirror_pos(&pattern.rows)
                .map(|left| left * 100)
                .or_else(|| mirror_pos(&pattern.cols))
                .unwrap_or(0)
        })
        .sum()
}

fn main() {
    let input = read_to_string("inputs/day13-input1.txt").unwrap();
    let patterns = parse(&input);
    let answer = part2(&patterns);
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

        assert_eq!(mirror_pos(&patterns[0].rows), Some(3));
        assert_eq!(mirror_pos(&patterns[1].rows), Some(1));
    }

    #[test]
    fn part2_test() {
        let patterns = parse(INPUT.trim());
        assert_eq!(part2(&patterns), 402);
    }
}
