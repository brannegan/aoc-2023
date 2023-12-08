use std::fs::read_to_string;

#[derive(Debug, PartialEq, Eq)]
struct Race {
    time: u64,
    distance: u64,
}

impl Race {
    fn new(time: u64, distance: u64) -> Self {
        Self { time, distance }
    }
}

fn parse(input: &str) -> Option<Race> {
    let mut lines = input.lines();
    let time = lines
        .next()?
        .strip_prefix("Time:")?
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect::<String>()
        .parse()
        .ok()?;
    let dist = lines
        .next()?
        .strip_prefix("Distance:")?
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect::<String>()
        .parse()
        .ok()?;
    Some(Race::new(time, dist))
}
fn part1(race: &Race) -> usize {
    (14..=race.time - 14)
        .map(|v| v * (race.time - v))
        .filter(|s| s > &race.distance)
        .count()
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
        assert_eq!(races, Race::new(71530, 940200));
    }
    #[test]
    fn part1_test() {
        let races = parse(INPUT.trim()).unwrap();
        assert_eq!(part1(&races), 71503);
    }
}
