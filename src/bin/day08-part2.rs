use std::collections::HashMap;
use std::fs::read_to_string;

use nom::bytes::complete::{tag, take};
use nom::character::complete::{line_ending, one_of};
use nom::multi::{many1, separated_list1};
use nom::sequence::{delimited, separated_pair, terminated};
use nom::{Finish, Parser};
use num::integer::lcm;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
enum Dir {
    L,
    R,
}
type Map<'a> = HashMap<&'a str, (&'a str, &'a str)>;

fn parse(input: &str) -> anyhow::Result<(Vec<Dir>, Map)> {
    let lr = one_of("LR").map(|c| match c {
        'L' => Dir::L,
        'R' => Dir::R,
        _ => unimplemented!(),
    });
    let label = |i| take(3u8)(i);
    let dirs = terminated(many1(lr), line_ending);
    let dst = delimited(tag("("), separated_pair(label, tag(", "), label), tag(")"));
    let line = separated_pair(label, tag(" = "), dst);
    let mut parser = separated_pair(dirs, line_ending, separated_list1(line_ending, line));
    let (_rest, (dirs, map)) = parser
        .parse(input)
        .finish()
        .map_err(|e: nom::error::VerboseError<&str>| anyhow::anyhow!("parser error: {:?}", e))?;
    Ok((dirs, map.into_iter().collect()))
}

fn part2(dirs: &[Dir], map: &Map) -> usize {
    map.keys()
        .filter(|key| key.ends_with('A'))
        .copied()
        .map(|mut pos| {
            let cu = &mut pos;
            dirs.iter()
                .cycle()
                .map(|dir| {
                    *cu = match dir {
                        Dir::L => map[cu].0,
                        Dir::R => map[cu].1,
                    };
                    *cu
                })
                .take_while(|pos| !pos.ends_with('Z'))
                .count()
                + 1
        })
        .reduce(lcm)
        .unwrap()
}

fn main() {
    let input = read_to_string("inputs/day08-input1.txt").unwrap();
    let (dirs, map) = parse(&input).unwrap();
    let answer = part2(&dirs, &map);
    println!("answer is: {answer}");
}
#[cfg(test)]
mod tests {

    use super::*;

    const INPUT1: &str = r#"
LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)
"#;
    #[test]
    fn parsing() {
        use Dir::*;
        let (dirs, map) = parse(INPUT1.trim()).unwrap();
        assert_eq!(dirs, vec![L, R]);
        assert_eq!(map[&"11A"], ("11B", "XXX"));
    }
    #[test]
    fn part2_test() {
        let (dirs, map) = parse(INPUT1.trim()).unwrap();
        assert_eq!(part2(&dirs, &map), 6);
    }
}
