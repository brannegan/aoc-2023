use indicatif::ProgressIterator;
use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt::Display;
use std::fs::read_to_string;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Tile {
    Damaged,
    Operational,
    Unknown,
}
#[derive(Debug, Clone, PartialEq, Eq)]
struct Spring {
    field: Vec<Tile>,
    groups: Vec<usize>,
}

impl Spring {
    fn new((field, groups): (&str, &str)) -> Self {
        let field = field.chars().map(Tile::from).collect();
        let groups = groups.split(',').map(|num| num.parse().unwrap()).collect();
        Self { field, groups }
    }
    fn recursive(
        &self,
        field_i: usize,
        group_i: usize,
        cur_len: usize,
        cache: &RefCell<HashMap<(usize, usize, usize), usize>>,
    ) -> usize {
        if cache.borrow().contains_key(&(field_i, group_i, cur_len)) {
            return cache.borrow()[&(field_i, group_i, cur_len)];
        }
        if field_i == self.field.len() {
            if (group_i == self.groups.len() && cur_len == 0)
                || (group_i == self.groups.len() - 1 && self.groups[group_i] == cur_len)
            {
                return 1;
            } else {
                return 0;
            }
        }
        let mut acc = 0;

        let mut process_tile = |tile: Tile| {
            if tile == Tile::Operational && cur_len == 0 {
                acc += self.recursive(field_i + 1, group_i, cur_len, cache);
            } else if tile == Tile::Operational
                && cur_len > 0
                && group_i < self.groups.len()
                && cur_len == self.groups[group_i]
            {
                acc += self.recursive(field_i + 1, group_i + 1, 0, cache);
            } else if tile == Tile::Damaged {
                acc += self.recursive(field_i + 1, group_i, cur_len + 1, cache);
            }
        };

        if self.field[field_i] == Tile::Unknown {
            process_tile(Tile::Damaged);
            process_tile(Tile::Operational);
        } else {
            process_tile(self.field[field_i]);
        }

        cache.borrow_mut().insert((field_i, group_i, cur_len), acc);
        acc
    }
    fn arrangements(&self) -> usize {
        let cache = RefCell::new(HashMap::new());
        self.recursive(0, 0, 0, &cache)
    }
    fn unfold(&mut self, count: usize) {
        let field_len = self.field.len();
        let group_len = self.groups.len();
        for _ in 0..count - 1 {
            self.field.push(Tile::Unknown);
            self.field.extend_from_within(0..field_len);
            self.groups.extend_from_within(0..group_len);
        }
    }
}
impl From<char> for Tile {
    fn from(value: char) -> Self {
        match value {
            '#' => Self::Damaged,
            '.' => Self::Operational,
            '?' => Self::Unknown,
            _ => unimplemented!(),
        }
    }
}
impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Tile::Damaged => '#',
                Tile::Operational => '.',
                Tile::Unknown => '?',
            }
        )
    }
}
impl Display for Spring {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for tile in self.field.iter() {
            write!(f, "{}", tile)?;
        }
        write!(f, " ")?;
        for tile in self.groups.iter() {
            write!(f, "{},", tile)?;
        }
        Ok(())
    }
}

fn parse(input: &str) -> Vec<Spring> {
    input
        .lines()
        .filter_map(|line| line.split_once(' '))
        .map(Spring::new)
        .collect()
}
fn part2(mut springs: Vec<Spring>) -> usize {
    springs.iter_mut().for_each(|spring| spring.unfold(5));
    springs.iter().progress().map(Spring::arrangements).sum()
}

fn main() {
    let input = read_to_string("inputs/day12-input1.txt").unwrap();
    let springs = parse(&input);
    let answer = part2(springs);
    println!("answer is: {answer}");
}
#[cfg(test)]
mod tests {

    use super::*;

    const INPUT: &str = r#"
???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1
"#;
    #[test]
    fn arrangements() {
        let mut springs = parse(INPUT.trim());
        springs.iter_mut().for_each(|spring| spring.unfold(5));
        let results: Vec<_> = springs.iter().map(Spring::arrangements).collect();

        assert_eq!(results, vec![1, 16384, 1, 16, 2500, 506250]);
    }
    #[test]
    fn part2_test() {
        let springs = parse(INPUT.trim());
        assert_eq!(part2(springs), 525152);
    }
}
