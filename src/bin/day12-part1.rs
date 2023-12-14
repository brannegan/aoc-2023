use std::fmt::Display;
use std::fs::read_to_string;
use std::ops::{Not, Range};

#[derive(Debug, PartialEq, Eq)]
enum Tile {
    Damaged,
    Operational,
    Unknown,
}
#[derive(Debug, PartialEq, Eq)]
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
    fn is_valid(&self, mut range: Range<usize>) -> bool {
        let start = range.start;
        let end = range.end;
        let range_is_valid =
            range.all(|tile_i| matches!(self.field[tile_i], Tile::Damaged | Tile::Unknown));
        let left_is_valid = if start == 0 {
            true
        } else {
            matches!(self.field[start - 1], Tile::Operational | Tile::Unknown)
        };
        let right_is_valid = if end == self.field.len() {
            true
        } else {
            matches!(self.field[end], Tile::Operational | Tile::Unknown)
        };
        left_is_valid && range_is_valid && right_is_valid
    }
    fn recursive(&self, gid: usize, offset: usize) -> usize {
        let size = self.groups[gid];
        if offset + size > self.field.len() {
            return 0;
        }
        let len = self.field[offset..]
            .iter()
            .position(|tile| *tile == Tile::Damaged)
            .unwrap_or(self.field[offset..].len() - size)
            .max(size);
        if gid == self.groups.len() - 1 {
            return (offset..=self.field.len() - size)
                .filter(|&start| self.is_valid(start..start + size))
                .filter(|&start| self.field[offset..start].contains(&Tile::Damaged).not())
                .filter(|&start| self.field[start + size..].contains(&Tile::Damaged).not())
                .count();
        }
        (offset..=offset + len)
            .filter(|&start| start + size < self.field.len())
            .filter(|&start| self.is_valid(start..start + size))
            .filter(|&start| self.field[offset..start].contains(&Tile::Damaged).not())
            .fold(0, |acc, start| {
                acc + self.recursive(gid + 1, start + size + 1)
            })
    }
    fn arrangements(&self) -> usize {
        self.recursive(0, 0)
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
        write!(f, " : ")?;
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
fn part1(springs: &[Spring]) -> usize {
    springs.iter().map(Spring::arrangements).sum()
}

fn main() {
    let input = read_to_string("inputs/day12-input1.txt").unwrap();
    let springs = parse(&input);
    let answer = part1(&springs);
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
    fn parsing() {
        let springs = parse(INPUT.trim());
        assert_eq!(format!("{}", springs[0]), "???.###");
    }
    #[test]
    fn arrangements() {
        let springs = parse(INPUT.trim());
        assert_eq!(springs[0].arrangements(), 1);
        assert_eq!(springs[1].arrangements(), 4);
        assert_eq!(springs[2].arrangements(), 1);
        assert_eq!(springs[3].arrangements(), 1);
        assert_eq!(springs[4].arrangements(), 4);
    }
    #[test]
    fn part1_test() {
        let springs = parse(INPUT.trim());
        assert_eq!(part1(&springs), 21);
    }
}
// 7857
