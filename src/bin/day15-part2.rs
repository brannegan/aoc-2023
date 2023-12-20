use std::fs::read_to_string;

fn parse(input: &str) -> Vec<&str> {
    input.split(',').collect()
}
fn hash(input: &str) -> usize {
    input
        .chars()
        .fold(0, |acc, ch| (acc + ch as usize) * 17 % 256)
}
const EMPTY: Vec<(&str, usize)> = Vec::new();

fn focusing_power(boxes: &[Vec<(&str, usize)>]) -> usize {
    boxes
        .iter()
        .enumerate()
        .flat_map(|(box_i, boxx)| {
            boxx.iter()
                .enumerate()
                .map(move |(slot_i, (_, focal_len))| (slot_i + 1) * (box_i + 1) * focal_len)
        })
        .sum()
}

fn hashmap<'a>(sequence: &'a [&'a str]) -> [Vec<(&'a str, usize)>; 256] {
    sequence.iter().fold([EMPTY; 256], |mut acc, &step| {
        let (label, focal_len) = step.split_once(['-', '=']).expect("step parsed");
        let box_i = hash(label);
        let slot_i = acc[box_i].iter().position(|(l, _)| *l == label);

        if !focal_len.is_empty() {
            let focal_len = focal_len.parse().expect("focal lenght parsed");
            if let Some(slot_i) = slot_i {
                acc[box_i].push((label, focal_len));
                acc[box_i].swap_remove(slot_i);
            } else {
                acc[box_i].push((label, focal_len));
            }
        } else if let Some(slot_i) = slot_i {
            acc[box_i].remove(slot_i);
        }
        acc
    })
}
fn part2(sequence: &[&str]) -> usize {
    focusing_power(&hashmap(sequence))
}

fn main() {
    let input = read_to_string("inputs/day15-input1.txt").unwrap();
    let sequence = parse(input.trim());
    let answer = part2(&sequence);

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
    fn focusing_power_test() {
        let boxes = [
            vec![("rn", 1), ("cm", 2)],
            EMPTY,
            EMPTY,
            vec![("ot", 7), ("ab", 5), ("pc", 6)],
        ];
        assert_eq!(focusing_power(&boxes), 145);
    }
    #[test]
    fn hashmap_test() {
        let sequence = parse(INPUT.trim());
        let mut boxes = [EMPTY; 256];
        boxes[0] = vec![("rn", 1), ("cm", 2)];
        boxes[3] = vec![("ot", 7), ("ab", 5), ("pc", 6)];
        assert_eq!(hashmap(&sequence)[..4], boxes[..4]);
    }
    #[test]
    fn label() {
        let str = "ab-";
        assert_eq!(str.split_once(['-', '=']), Some(("ab", "")));
        assert!("".is_empty());

        let str = "ab=9";
        assert_eq!(str.split_once(['-', '=']), Some(("ab", "9")));
    }
    #[test]
    fn part2_test() {
        let sequence = parse(INPUT.trim());
        assert_eq!(part2(&sequence), 145);
    }
}
