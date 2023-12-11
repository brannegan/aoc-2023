use std::collections::HashMap;
use std::fs::read_to_string;

use nom::bytes::complete::{tag, take};
use nom::character::complete::{line_ending, one_of};
use nom::multi::{many1, separated_list1};
use nom::sequence::{delimited, separated_pair, terminated};
use nom::{Finish, Parser};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
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

fn part1(dirs: &[Dir], map: &Map) -> usize {
    let mut pos = "AAA";
    dirs.iter()
        .cycle()
        .take_while(|dir| {
            pos = match dir {
                Dir::L => map[&pos].0,
                Dir::R => map[&pos].1,
            };
            pos != "ZZZ"
        })
        .count()
        + 1
}

fn main() {
    let input = read_to_string("inputs/day08-input1.txt").unwrap();
    let (dirs, map) = parse(&input).unwrap();
    let answer = part1(&dirs, &map);
    println!("answer is: {answer}");
}
#[cfg(test)]
mod tests {

    use super::*;

    const INPUT1: &str = r#"
RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)
"#;
    const INPUT2: &str = r#"
LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)
"#;
    #[test]
    fn parsing() {
        use Dir::*;
        let (dirs, map) = parse(INPUT1.trim()).unwrap();
        assert_eq!(dirs, vec![R, L]);
        assert_eq!(map[&"AAA"], ("BBB", "CCC"));

        let (dirs, map) = parse(INPUT2.trim()).unwrap();
        assert_eq!(dirs, vec![L, L, R]);
        assert_eq!(map[&"BBB"], ("AAA", "ZZZ"));
    }
    #[test]
    fn part1_test() {
        let (dirs, map) = parse(INPUT1.trim()).unwrap();
        assert_eq!(part1(&dirs, &map), 2);

        let (dirs, map) = parse(INPUT2.trim()).unwrap();
        assert_eq!(part1(&dirs, &map), 6);
    }
}
