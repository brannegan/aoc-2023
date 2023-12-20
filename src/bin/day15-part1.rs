use std::fs::read_to_string;

fn parse(input: &str) -> Vec<&str> {
    input.split(',').collect()
}
fn hash(input: &str) -> u32 {
    input
        .chars()
        .fold(0, |acc, ch| (acc + ch as u32) * 17 % 256)
}
fn part1(sequence: &[&str]) -> u32 {
    sequence.iter().copied().map(hash).sum()
}

fn main() {
    let input = read_to_string("inputs/day15-input1.txt").unwrap();
    let sequence = parse(input.trim());
    let answer = part1(&sequence);
    println!("answer is: {answer}");
}
#[cfg(test)]
mod tests {

    use crate::hash;

    use super::*;

    const INPUT: &str = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";
    #[test]
    fn parsing() {
        let sequence = parse(INPUT.trim());
        assert_eq!(sequence[0], "rn=1");
        assert_eq!(sequence[1], "cm-");
    }
    #[test]
    fn ascii() {
        let ch = 'H';
        assert_eq!(ch as u8, 72);
        let ch = 'A';
        assert_eq!(ch as u8, 65);
    }
    #[test]
    fn hash_test() {
        let input = "HASH";
        assert_eq!(hash(input), 52);
    }
    #[test]
    fn part1_test() {
        let sequence = parse(INPUT.trim());
        assert_eq!(part1(&sequence), 1320);
    }
}
