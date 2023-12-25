use std::collections::{BinaryHeap, HashMap};
use std::fmt::Display;
use std::fs::read_to_string;

#[derive(Debug)]
struct HeatMap {
    data: Vec<Vec<u32>>,
}

impl HeatMap {
    fn size(&self) -> usize {
        self.data.len()
    }
    fn neighbors(&self, cur: Crucible) -> impl Iterator<Item = Pos> {
        let left = cur.pos.x.checked_sub(1).map(|x| Pos::new(x, cur.pos.y));
        let right = (cur.pos.x + 1 < self.size()).then_some(Pos::new(cur.pos.x + 1, cur.pos.y));
        let up = cur.pos.y.checked_sub(1).map(|y| Pos::new(cur.pos.x, y));
        let down = (cur.pos.y + 1 < self.size()).then_some(Pos::new(cur.pos.x, cur.pos.y + 1));
        [left, right, up, down]
            .into_iter()
            .flatten()
            .filter(move |next| !cur.trail.iter().any(|pos| pos == next))
            .filter(move |next| {
                !((next.x.abs_diff(cur.trail.last().unwrap().x) == 4
                    && next.y.abs_diff(cur.trail.last().unwrap().y) == 0)
                    || (next.y.abs_diff(cur.trail.last().unwrap().y) == 4
                        && next.x.abs_diff(cur.trail.last().unwrap().x) == 0))
            })
    }

    fn weight(&self, pos: Pos) -> u32 {
        self.data[pos.y][pos.x]
    }
}
#[derive(Debug, Default, PartialEq, Eq, Clone, Copy, Hash)]
struct Pos {
    x: usize,
    y: usize,
}

impl Display for Pos {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}

impl Pos {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, Default)]
struct MinPos(Crucible, u32);

impl PartialOrd for MinPos {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for MinPos {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.1.cmp(&self.1)
    }
}

#[derive(Debug, Default, PartialEq, Eq, Clone, Copy, Hash)]
struct Crucible {
    pos: Pos,
    trail: [Pos; 3],
}

impl Crucible {
    fn new(pos: Pos) -> Self {
        Self {
            pos,
            trail: Default::default(),
        }
    }
}

impl HeatMap {
    // stealed from https://en.wikipedia.org/wiki/A*_search_algorithm
    fn astar(&self, start: Pos, goal: Pos) -> (u32, Vec<Pos>) {
        let mut came_from: HashMap<Crucible, Crucible> = HashMap::new();
        let mut g_score: HashMap<Crucible, u32> = HashMap::new();
        let mut f_score: BinaryHeap<MinPos> = BinaryHeap::new();
        let init = Crucible::new(start);
        f_score.push(MinPos(init, 0));
        g_score.insert(init, 0);
        while let Some(MinPos(current, lowest)) = f_score.pop() {
            if current.pos == goal {
                return (lowest, path(&came_from, current));
            }
            for next_pos in self.neighbors(current) {
                let mut neighbor = current;
                neighbor.trail.rotate_right(1);
                neighbor.trail[0] = current.pos;
                neighbor.pos = next_pos;

                let neighbor_g_score = g_score[&current] + self.weight(next_pos);
                if neighbor_g_score < *g_score.get(&neighbor).unwrap_or(&u32::MAX) {
                    came_from.insert(neighbor, current);
                    g_score.insert(neighbor, neighbor_g_score);
                    f_score.push(MinPos(neighbor, neighbor_g_score));
                }
            }
        }
        (0, vec![])
    }
}

fn path(came_from: &HashMap<Crucible, Crucible>, mut cur: Crucible) -> Vec<Pos> {
    let mut path = vec![cur.pos];
    while let Some(prev) = came_from.get(&cur) {
        path.push(prev.pos);
        cur = *prev;
    }
    path.into_iter().rev().collect()
}

fn parse(input: &str) -> HeatMap {
    let data: Vec<Vec<_>> = input
        .lines()
        .map(|line| line.chars().map(|ch| ch.to_digit(10).unwrap()).collect())
        .collect();

    HeatMap { data }
}
fn part1(heat_map: &HeatMap) -> u32 {
    let (len, path) = heat_map.astar(
        Pos::new(0, 0),
        Pos::new(heat_map.size() - 1, heat_map.size() - 1),
    );
    for y in 0..heat_map.data.len() {
        for x in 0..heat_map.data[0].len() {
            if path.contains(&Pos::new(x, y)) {
                print!(".");
            } else {
                print!("{}", heat_map.data[y][x]);
            }
        }
        println!()
    }

    len
}

fn main() {
    let input = read_to_string("inputs/day17-input1.txt").unwrap();
    let heat_map = parse(&input);
    let answer = part1(&heat_map);
    println!("answer is: {answer}");
}
#[cfg(test)]
mod tests {

    use super::*;

    const INPUT: &str = r#"
2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533
"#;
    const SIMPLE: &str = r#"
2413
3315
3215
4546
"#;
    #[test]
    fn parsing() {
        let heat_map = parse(INPUT.trim());
        assert_eq!(heat_map.data[0][1], 4);
        assert_eq!(heat_map.data[1][0], 3);
    }
    #[test]
    fn weight() {
        let heat_map = parse(INPUT.trim());
        assert_eq!(heat_map.weight(Pos::new(1, 0)), 4);
        assert_eq!(heat_map.weight(Pos::new(0, 1)), 3);
    }
    #[test]
    fn astar_test() {
        let heat_map = parse(SIMPLE.trim());
        let (len, path) = heat_map.astar(Pos::new(0, 0), Pos::new(3, 3));
        assert_eq!(
            path,
            vec![
                Pos::new(0, 0),
                Pos::new(1, 0),
                Pos::new(2, 0),
                Pos::new(2, 1),
                Pos::new(2, 2),
                Pos::new(3, 2),
                Pos::new(3, 3)
            ]
        );
        assert_eq!(len, 18);
    }
    #[test]
    fn part1_test() {
        let heat_map = parse(INPUT.trim());
        assert_eq!(part1(&heat_map), 102);
    }
}
