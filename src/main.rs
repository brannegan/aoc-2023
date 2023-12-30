use std::fs::read_to_string;

fn parse(input: &str) -> &str {
    let mut lines = input.lines();
    input
}
fn part1(parsed: &str) -> usize {
    0
}

fn main() {
    let input = read_to_string("inputs/day16-input1.txt").unwrap();
    let parsed = parse(&input);
    let answer = part1(&parsed);
    println!("answer is: {answer}");
}
#[cfg(test)]
mod tests {

    use super::*;

    const INPUT: &str = r#"

"#;
    #[test]
    fn parsing() {
        let parsed = parse(INPUT.trim());
        assert_eq!(parsed, "");
    }
    #[test]
    fn part1_test() {
        let parsed = parse(INPUT.trim());
        assert_eq!(part1(&parsed), 0);
    }
}
