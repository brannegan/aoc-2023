#![allow(dead_code)]
use std::cmp::Ordering;
use std::fs::read_to_string;
use std::str::FromStr;

use anyhow::{anyhow, Ok};
use itertools::Itertools;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
#[repr(u8)]
enum Card {
    A,
    K,
    Q,
    T,
    Nine,
    Eight,
    Seven,
    Six,
    Five,
    Four,
    Three,
    Two,
    J,
}
impl Card {
    fn strength(&self) -> u8 {
        255 - *self as u8
    }
    fn from_char(c: char) -> Self {
        match c {
            'A' => Self::A,
            'K' => Self::K,
            'Q' => Self::Q,
            'J' => Self::J,
            'T' => Self::T,
            '9' => Self::Nine,
            '8' => Self::Eight,
            '7' => Self::Seven,
            '6' => Self::Six,
            '5' => Self::Five,
            '4' => Self::Four,
            '3' => Self::Three,
            '2' => Self::Two,
            _ => unimplemented!("no such card"),
        }
    }
}
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
#[repr(u8)]
enum HandType {
    Five,
    Four,
    FullHouse,
    Three,
    TwoPair,
    OnePair,
    High,
}
impl HandType {
    fn strength(&self) -> u8 {
        255 - *self as u8
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Hand([Card; 5]);
impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.handtype().strength().cmp(&other.handtype().strength()) {
            Ordering::Less => Ordering::Less,
            Ordering::Equal => self
                .0
                .iter()
                .map(Card::strength)
                .cmp(other.0.iter().map(Card::strength)),
            Ordering::Greater => Ordering::Greater,
        }
    }
}
impl From<Vec<Card>> for Hand {
    fn from(v: Vec<Card>) -> Self {
        Self([v[0], v[1], v[2], v[3], v[4]])
    }
}
impl Hand {
    fn handtype(&self) -> HandType {
        let dedup_it = self
            .0
            .iter()
            .sorted()
            .dedup_with_count()
            .sorted_by_key(|(count, _card)| -(*count as isize));
        let jokers = dedup_it
            .clone()
            .find(|(_, card)| **card == Card::J)
            .map(|(count, _)| count)
            .unwrap_or(0);
        let mut dedup_it = dedup_it.map(|(count, _)| count);
        match dedup_it.next() {
            Some(5) => HandType::Five,
            Some(4) => {
                if jokers != 0 {
                    HandType::Five
                } else {
                    HandType::Four
                }
            }
            Some(3) => {
                if let Some(2) = dedup_it.next() {
                    if jokers != 0 {
                        HandType::Five
                    } else {
                        HandType::FullHouse
                    }
                } else if jokers != 0 {
                    HandType::Four
                } else {
                    HandType::Three
                }
            }
            Some(2) => {
                if let Some(2) = dedup_it.next() {
                    if jokers == 2 {
                        HandType::Four
                    } else if jokers == 1 {
                        HandType::FullHouse
                    } else {
                        HandType::TwoPair
                    }
                } else if jokers != 0 {
                    HandType::Three
                } else {
                    HandType::OnePair
                }
            }
            Some(1) => {
                if jokers != 0 {
                    HandType::OnePair
                } else {
                    HandType::High
                }
            }
            _ => unreachable!(),
        }
    }
}
#[derive(Debug)]
struct Game {
    hand: Hand,
    bid: u32,
}
impl FromStr for Game {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut it = s.split_whitespace();
        let (hand, bid) = (
            it.next().ok_or(anyhow!("failed to parse hand"))?,
            it.next().ok_or(anyhow!("failed to parse bid"))?.parse()?,
        );
        let hand = hand
            .chars()
            .map(Card::from_char)
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        Ok(Game { hand, bid })
    }
}

fn parse(input: &str) -> Vec<Game> {
    input
        .lines()
        .map(|line| line.parse::<Game>().expect("Game parsed"))
        .collect()
}
fn part2(games: &[Game]) -> u32 {
    games
        .iter()
        .sorted_by(|a, b| a.hand.cmp(&b.hand))
        .enumerate()
        .fold(0, |acc, (i, game)| acc + ((i as u32 + 1) * game.bid))
}

fn main() {
    let input = read_to_string("inputs/day07-input1.txt").unwrap();
    let games = parse(&input);
    let answer = part2(&games);
    println!("answer is: {answer}");
}
#[cfg(test)]
mod tests {

    use super::*;

    const INPUT: &str = r#"
32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483
"#;
    #[test]
    fn parsing() {
        let games = parse(INPUT.trim());
        use Card::*;
        assert_eq!(games[0].hand, Hand([Three, Two, T, Three, K]));
        assert_eq!(games[0].bid, 765);
    }
    #[test]
    fn strength_card_test() {
        assert!(Card::A.strength() > Card::K.strength());
        assert!(Card::Q.strength() > Card::Three.strength());
    }
    #[test]
    fn strength_hand_test() {
        assert!(HandType::Four.strength() > HandType::High.strength());
    }
    #[test]
    fn handtype_test() {
        use Card::*;
        let hand = Hand([A, A, A, A, J]);
        assert_eq!(hand.handtype(), HandType::Five);
        let hand = Hand([A, A, A, J, J]);
        assert_eq!(hand.handtype(), HandType::Five);
        let hand = Hand([A, A, J, J, J]);
        assert_eq!(hand.handtype(), HandType::Five);
        let hand = Hand([A, J, J, J, J]);
        assert_eq!(hand.handtype(), HandType::Five);

        let hand = Hand([A, A, K, J, A]);
        assert_eq!(hand.handtype(), HandType::Four);
        let hand = Hand([A, J, K, J, J]);
        assert_eq!(hand.handtype(), HandType::Four);

        let hand = Hand([A, J, K, K, A]);
        assert_eq!(hand.handtype(), HandType::FullHouse);

        let hand = Hand([A, J, K, Q, A]);
        assert_eq!(hand.handtype(), HandType::Three);
        let hand = Hand([A, J, K, Q, J]);
        assert_eq!(hand.handtype(), HandType::Three);

        let hand = Hand([A, K, K, Q, A]);
        assert_eq!(hand.handtype(), HandType::TwoPair);

        let hand = Hand([A, Nine, K, Q, J]);
        assert_eq!(hand.handtype(), HandType::OnePair);

        let hand = Hand([T, Nine, K, Q, A]);
        assert_eq!(hand.handtype(), HandType::High);
    }
    #[test]
    fn handrank() {}
    #[test]
    fn part2_test() {
        let games = parse(INPUT.trim());
        assert_eq!(part2(&games), 5905);
    }
}
