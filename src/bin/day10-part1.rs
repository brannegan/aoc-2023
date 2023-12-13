use std::fmt::Display;
use std::fs::read_to_string;

use itertools::Itertools;
use petgraph::algo::astar;
use petgraph::stable_graph::NodeIndex;
use petgraph::{Graph, Undirected};

//#[derive(Debug, PartialEq, Eq, Clone, Copy)]
//enum CardDir {
//    N,
//    S,
//    E,
//    W,
//}
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Tile {
    Dir([char; 2]),
    Ground,
    Start,
}
impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Tile::Dir(dir) => match dir {
                    ['N', 'S'] => '|',
                    ['W', 'E'] => '-',
                    ['N', 'E'] => 'L',
                    ['N', 'W'] => 'J',
                    ['S', 'W'] => '7',
                    ['S', 'E'] => 'F',
                    _ => unimplemented!(),
                },
                Tile::Ground => '.',
                Tile::Start => 'S',
            }
        )
    }
}
impl From<char> for Tile {
    fn from(value: char) -> Self {
        use Tile::*;
        match value {
            '|' => Dir(['N', 'S']),
            '-' => Dir(['W', 'E']),
            'L' => Dir(['N', 'E']),
            'J' => Dir(['N', 'W']),
            '7' => Dir(['S', 'W']),
            'F' => Dir(['S', 'E']),
            '.' => Ground,
            'S' => Start,
            _ => unimplemented!("{value}"),
        }
    }
}
impl Tile {
    fn contains(&self, ch: char) -> bool {
        if let Tile::Dir(dir) = self {
            dir.contains(&ch)
        } else {
            false
        }
    }
}

fn parse(input: &str) -> Vec<Vec<Tile>> {
    input
        .lines()
        .map(|line| line.chars().map(Tile::from).collect())
        .collect()
}
fn construct_graph(map: &Vec<Vec<Tile>>) -> Graph<(Tile, (usize, usize)), (), Undirected> {
    let mut graph = Graph::new_undirected();
    let w = map[0].len();
    let h = map.len();

    (0..h).cartesian_product(0..w).for_each(|(r, c)| {
        graph.add_node((map[r][c], (r, c)));
    });

    (0..h).cartesian_product(0..w).for_each(|(r, c)| {
        if c + 1 < w && map[r][c].contains('E') && map[r][c + 1].contains('W') {
            graph.add_edge(NodeIndex::new(r * w + c), NodeIndex::new(r * w + c + 1), ());
        }
        if r + 1 < h && map[r][c].contains('S') && map[r + 1][c].contains('N') {
            graph.add_edge(
                NodeIndex::new(r * w + c),
                NodeIndex::new((r + 1) * w + c),
                (),
            );
        }
    });
    graph
}
fn start_pos(pipes: &Vec<Vec<Tile>>) -> [(usize, usize); 3] {
    let (start_r, start_c) = (0..pipes.len())
        .cartesian_product(0..pipes[0].len())
        .filter(|(r, c)| pipes[*r][*c] == Tile::Start)
        .last()
        .unwrap();
    let mut v = vec![(start_r, start_c)];
    if start_r > 0 && pipes[start_r - 1][start_c].contains('S') {
        v.push((start_r - 1, start_c))
    }
    if pipes[start_r + 1][start_c].contains('N') {
        v.push((start_r + 1, start_c))
    }
    if start_c > 0 && pipes[start_r][start_c - 1].contains('E') {
        v.push((start_r, start_c - 1))
    }
    if pipes[start_r][start_c + 1].contains('W') {
        v.push((start_r, start_c + 1))
    }
    v.try_into().expect("Starting point and 2 connected points")
}
fn print_pipes(pipes: &Vec<Vec<Tile>>) {
    for row in pipes {
        for t in row {
            print!("{t}");
        }
        println!();
    }
}

fn part1(pipes: Vec<Vec<Tile>>) -> usize {
    let [start, begin, end] = start_pos(&pipes);
    let w = pipes[0].len();
    let graph = construct_graph(&pipes);
    let (_cost, mut loop_path) = astar(
        &graph,
        NodeIndex::new(begin.0 * w + begin.1),
        |finish| finish == NodeIndex::new(end.0 * w + end.1),
        |_| 0,
        |_| 0,
    )
    .unwrap();
    loop_path.push(NodeIndex::new(start.0 * w + start.1));
    loop_path.len().div_ceil(2)
}

fn main() {
    let input = read_to_string("inputs/day10-input1.txt").unwrap();
    let pipes = parse(&input);
    let answer = part1(pipes);
    println!("answer is: {answer}");
}
#[cfg(test)]
mod tests {

    use super::*;

    const INPUT1: &str = r#"
-L|F7
7S-7|
L|7||
-L-J|
L|-JF
"#;
    const INPUT2: &str = r#"
..F7.
.FJ|.
SJ.L7
|F--J
LJ...
"#;
    #[test]
    fn tile_display() {
        let tile = Tile::Dir(['N', 'S']);
        println!("{tile}");
        assert_eq!(format!("{tile}"), "|");
    }
    #[test]
    fn part1_test1() {
        let pipes = parse(INPUT1.trim());
        assert_eq!(part1(pipes), 4);
    }
    #[test]
    fn part1_test2() {
        let pipes = parse(INPUT2.trim());
        assert_eq!(part1(pipes), 8);
    }
}
