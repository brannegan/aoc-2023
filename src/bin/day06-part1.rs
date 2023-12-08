use std::fs::read_to_string;

#[derive(Debug, PartialEq, Eq)]
struct Race {
    time: u32,
    distance: u32,
}

impl Race {
    fn new(time: u32, distance: u32) -> Self {
        Self { time, distance }
    }
}

fn parse(input: &str) -> Option<Vec<Race>> {
    let mut lines = input.lines();
    let time_it = lines
        .next()?
        .strip_prefix("Time:")?
        .split_whitespace()
        .map(str::parse)
        .filter_map(Result::ok);
    let dist_it = lines
        .next()?
        .strip_prefix("Distance:")?
        .split_whitespace()
        .map(str::parse)
        .filter_map(Result::ok);
    Some(
        time_it
            .zip(dist_it)
            .map(|(time, distance)| Race::new(time, distance))
            .collect(),
    )
}
fn part1(races: &[Race]) -> usize {
    races
        .iter()
        .map(|race| {
            (0..race.time)
                .map(|v| v * (race.time - v))
                .filter(|s| s > &race.distance)
                .count()
        })
        .product()
}

fn main() {
    let input = read_to_string("inputs/day06-input1.txt").unwrap();
    let races = parse(&input).unwrap();
    let answer = part1(&races);
    println!("answer is: {answer}");
}
#[cfg(test)]
mod tests {

    use super::*;

    const INPUT: &str = r#"
Time:      7  15   30
Distance:  9  40  200
"#;
    #[test]
    fn parsing() {
        let races = parse(INPUT.trim()).unwrap();
        assert_eq!(
            races,
            vec![Race::new(7, 9), Race::new(15, 40), Race::new(30, 200)]
        );
    }
    #[test]
    fn part1_test() {
        let races = parse(INPUT.trim()).unwrap();
        assert_eq!(part1(&races), 288);
    }
}
