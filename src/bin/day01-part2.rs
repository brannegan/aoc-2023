use std::collections::BTreeMap;
use std::fs::read_to_string;

const WORDS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];
const DIGITS: [&str; 9] = ["1", "2", "3", "4", "5", "6", "7", "8", "9"];

fn main() {
    let input = read_to_string("inputs/day01-input2.txt").unwrap();
    let answer = trebuchet_calibration(&input);
    println!("answer is: {answer}");
}
fn restored_value(s: &str) -> usize {
    let digit_to_index: BTreeMap<_, _> = WORDS
        .iter()
        .enumerate()
        .chain(DIGITS.iter().enumerate())
        .flat_map(|(i, &word)| {
            s.match_indices(word)
                .map(move |(offset, _)| (offset, i + 1))
        })
        .collect();
    match (digit_to_index.iter().next(), digit_to_index.iter().last()) {
        (Some((_, fst)), Some((_, lst))) => fst * 10 + lst,
        (Some((_, fst)), None) => fst * 10 + fst,
        (_, _) => unreachable!(),
    }
}

fn trebuchet_calibration(input: &str) -> usize {
    input.trim().lines().map(restored_value).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_line() {
        const INPUT: &str = "eightwothree";
        assert_eq!(restored_value(INPUT), 83);
    }

    #[test]
    fn same_digits() {
        const INPUT: &str = "8jkfncbeight7seven8";
        assert_eq!(restored_value(INPUT), 88);
    }
    #[test]
    fn same_words() {
        const INPUT: &str = "onetwoone";
        assert_eq!(restored_value(INPUT), 11);
    }

    const INPUT: &str = r#"
two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen
"#;
    #[test]
    fn part2() {
        let input = INPUT.trim();
        eprintln!("input = {:#?}", input);
        assert_eq!(trebuchet_calibration(input), 281);
    }
}
