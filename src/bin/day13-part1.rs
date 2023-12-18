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
    let group_ids = lines
        .iter()
        .enumerate()
        .into_group_map_by(|(_, line)| line.to_owned())
        .into_values()
        .filter(|group| group.len() > 1)
        .flat_map(|v| {
            v.into_iter()
                .map(|(pos, _)| pos)
                .combinations(2)
                .map(|v| v[0]..v[1])
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    if group_ids.is_empty() {
        return None;
    }
    let top: Vec<_> = group_ids
        .iter()
        .filter(|group| group.start == 0)
        .cloned()
        .collect();
    let btm: Vec<_> = group_ids
        .iter()
        .filter(|group| group.end == lines.len() - 1)
        .cloned()
        .collect();

    let valid_mirror = |mirror_ranges: Vec<Range<usize>>| {
        mirror_ranges
            .iter()
            .find(|&range| {
                (range.start..=(range.start + (range.end - range.start).div_ceil(2)))
                    .zip((range.start + (range.end - range.start).div_ceil(2)..=range.end).rev())
                    .all(|(start, end)| group_ids.iter().any(|r| r.start == start && r.end == end))
            })
            .map(|range| range.start + range.len().div_ceil(2))
    };
    valid_mirror(top).or(valid_mirror(btm))
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
