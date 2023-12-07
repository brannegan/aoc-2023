use nom::bytes::complete::tag;
use nom::character::complete::{char, line_ending, space1, u32};
use nom::multi::separated_list1;
use nom::sequence::{preceded, separated_pair, tuple};
use nom::{Finish, Parser};
use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::fs::read_to_string;

#[derive(Debug)]
struct Card {
    winning: HashSet<u32>,
    my: HashSet<u32>,
}

impl Card {
    fn new(winning: Vec<u32>, yours: Vec<u32>) -> Self {
        Self {
            winning: winning.into_iter().collect(),
            my: yours.into_iter().collect(),
        }
    }
}

fn parse(input: &str) -> anyhow::Result<Vec<Card>> {
    let card_number = tuple((tag("Card"), space1, u32, char(':'), space1));
    let numbers = |i| separated_list1(space1, u32)(i);
    let sep = tuple((space1, char('|'), space1));
    let line = separated_pair(numbers, sep, numbers).map(|(w, y)| Card::new(w, y));

    let mut parser = separated_list1(line_ending, preceded(card_number, line));
    parser
        .parse(input)
        .finish()
        .map(|(_, parsed)| parsed)
        .map_err(|e: nom::error::VerboseError<&str>| anyhow::anyhow!("parser error: {:?}", e))
}
fn part2(cards: &[Card]) -> usize {
    let cache = RefCell::new(HashMap::new());
    fn count_copies(
        cards_copies: &[usize],
        id: usize,
        cache: &RefCell<HashMap<usize, usize>>,
    ) -> usize {
        if cache.borrow().contains_key(&id) {
            return cache.borrow()[&id];
        }
        let count = (1..=cards_copies[id])
            .fold(1, |acc, i| acc + count_copies(cards_copies, id + i, cache));
        cache.borrow_mut().insert(id, count);
        count
    }
    let cards_copies: Vec<_> = cards
        .iter()
        .map(|c| c.my.intersection(&c.winning).count())
        .collect();
    (0..cards_copies.len()).fold(0, |acc, id| acc + count_copies(&cards_copies, id, &cache))
}

fn main() {
    let input = read_to_string("inputs/day04-input1.txt").unwrap();
    let cards = parse(&input).unwrap();
    let answer = part2(&cards);
    println!("answer is: {answer}");
}
#[cfg(test)]
mod tests {
    use anyhow::Ok;

    use super::*;

    const INPUT: &str = r#"
Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
"#;
    #[test]
    fn parsing() -> anyhow::Result<()> {
        let cards = parse(INPUT.trim())?;
        assert_eq!(
            cards[0].winning,
            vec![41, 48, 83, 86, 17].into_iter().collect()
        );
        assert_eq!(
            cards[0].my,
            vec![83, 86, 6, 31, 17, 9, 48, 53].into_iter().collect()
        );
        Ok(())
    }
    #[test]
    fn part2_test() -> anyhow::Result<()> {
        let cards = parse(INPUT.trim())?;
        assert_eq!(part2(&cards), 30);
        Ok(())
    }
}
