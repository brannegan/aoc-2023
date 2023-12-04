use std::fs::read_to_string;

fn main() {
    let input = read_to_string("inputs/day01-input1.txt").unwrap();
    let answer = part1(&input);
    println!("answer is: {answer}");
}

fn part1(input: &str) -> u32 {
    input
        .trim()
        .lines()
        .map(|line| line.chars().filter(char::is_ascii_digit))
        .fold(0, |mut acc, it| {
            let fst = it.clone().peekable().peek().unwrap().to_digit(10).unwrap();
            let lst = it.last().unwrap().to_digit(10).unwrap();
            acc = acc + fst * 10 + lst;
            acc
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"
1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet
"#;
    #[test]
    fn part1_test() {
        assert_eq!(part1(INPUT), 142);
    }
}
