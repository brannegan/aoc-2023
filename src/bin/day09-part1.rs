use std::fs::read_to_string;

use itertools::{unfold, Itertools};

fn parse(input: &str) -> Vec<Vec<i32>> {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(str::parse)
                .map(Result::unwrap)
                .collect()
        })
        .collect()
}

fn extrapolate(line: &Vec<i32>) -> i32 {
    let init = line.to_owned();
    let extrapolated = unfold(init, |v| {
        *v = v
            .iter()
            .tuple_windows()
            .map(|(l, r)| r - l)
            .collect::<Vec<_>>();
        if v.iter().all(|e| *e == 0) {
            None
        } else {
            Some(v.clone())
        }
    })
    .fold(0, |acc, v| acc + v.last().unwrap());
    let last = *line.last().unwrap();
    last + extrapolated
}
fn part1(values: &[Vec<i32>]) -> i32 {
    values.iter().map(extrapolate).sum()
}

fn main() {
    let input = read_to_string("inputs/day09-input1.txt").unwrap();
    let values = parse(&input);
    let answer = part1(&values);
    println!("answer is: {answer}");
}
#[cfg(test)]
mod tests {

    use super::*;

    const INPUT: &str = r#"
0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45
"#;
    #[test]
    fn parsing() {
        let values = parse(INPUT.trim());
        assert_eq!(values[1], vec![1, 3, 6, 10, 15, 21]);
        assert_eq!(values[2], vec![10, 13, 16, 21, 30, 45]);
    }
    #[test]
    fn part1_test() {
        let values = parse(INPUT.trim());
        assert_eq!(part1(&values), 114);
    }
}
