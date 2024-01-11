use std::fmt::Display;
use std::fs::read_to_string;

use itertools::Itertools;
use petgraph::matrix_graph::NodeIndex;
use petgraph::{graph, Graph, Undirected};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Forest,
    Path,
    LSlope,
    RSlope,
    USlope,
    DSlope,
    Walk,
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Tile::Forest => '#',
                Tile::Path => '.',
                Tile::LSlope => '<',
                Tile::RSlope => '>',
                Tile::USlope => '^',
                Tile::DSlope => 'v',
                Tile::Walk => 'O',
            }
        )
    }
}
impl From<char> for Tile {
    fn from(value: char) -> Self {
        match value {
            '#' => Tile::Forest,
            '.' => Tile::Path,
            '<' => Tile::LSlope,
            '>' => Tile::RSlope,
            '^' => Tile::USlope,
            'v' => Tile::DSlope,
            _ => unimplemented!(),
        }
    }
}
type Map = Vec<Vec<Tile>>;
fn _print_map(map: &Map) {
    for row in map {
        for c in row {
            print!("{c}");
        }
        println!();
    }
}

fn construct_graph(map: &Map) -> Graph<(Tile, (usize, usize)), (), Undirected> {
    let mut graph = Graph::new_undirected();
    let w = map[0].len();
    let h = map.len();

    (0..h).cartesian_product(0..w).for_each(|(r, c)| {
        graph.add_node((map[r][c], (r, c)));
    });

    (0..h).cartesian_product(0..w).for_each(|(r, c)| {
        if c + 1 < w && !matches!(map[r][c], Tile::Forest) && !matches!(map[r][c + 1], Tile::Forest)
        {
            graph.add_edge(NodeIndex::new(r * w + c), NodeIndex::new(r * w + c + 1), ());
        }
        if r + 1 < h && !matches!(map[r][c], Tile::Forest) && !matches!(map[r + 1][c], Tile::Forest)
        {
            graph.add_edge(
                NodeIndex::new(r * w + c),
                NodeIndex::new((r + 1) * w + c),
                (),
            );
        }
    });
    graph
}
fn parse(input: &str) -> Map {
    input
        .lines()
        .map(|line| line.chars().map(Tile::from).collect())
        .collect()
}

fn part1(map: Map) -> usize {
    let size = map.len();
    let mut graph = construct_graph(&map);
    graph.retain_nodes(|g, ni| g[ni].0 != Tile::Forest);
    let from = graph
        .node_indices()
        .find(|ni| graph[*ni].1 == (0, 1))
        .expect("entrance");
    let to = graph
        .node_indices()
        .find(|ni| graph[*ni].1 == (size - 1, size - 2))
        .expect("exit");
    dbg!(from, to);
    let paths: Vec<Vec<_>> = petgraph::algo::all_simple_paths(&graph, from, to, 1, None).collect();
    dbg!(paths.len());
    let longest_path = paths
        .iter()
        .filter(|path| {
            path.iter()
                .map(|ni| graph[*ni].1)
                .tuple_windows()
                .all(|(a, b)| {
                    a.0 > b.0 && !matches!(map[b.0][b.1], Tile::DSlope)
                        || a.0 < b.0 && !matches!(map[b.0][b.1], Tile::USlope)
                        || a.1 > b.1 && !matches!(map[b.0][b.1], Tile::RSlope)
                        || a.1 < b.1 && !matches!(map[b.0][b.1], Tile::LSlope)
                })
        })
        .max_by_key(|path| path.len())
        .expect("valid path");
    /*
    for ni in longest_path {
        let (r, c) = graph[*ni].1;
        map[r][c] = Tile::Walk;
    }
    print_map(&map);
    */

    longest_path.len() - 1
}

fn main() {
    let input = read_to_string("inputs/day23-input1.txt").unwrap();
    let parsed = parse(&input);
    let answer = part1(parsed);
    println!("answer is: {answer}");
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = r#"
#.#####################
#.......#########...###
#######.#########.#.###
###.....#.>.>.###.#.###
###v#####.#v#.###.#.###
###.>...#.#.#.....#...#
###v###.#.#.#########.#
###...#.#.#.......#...#
#####.#.#.#######.#.###
#.....#.#.#.......#...#
#.#####.#.#.#########v#
#.#...#...#...###...>.#
#.#.#v#######v###.###v#
#...#.>.#...>.>.#.###.#
#####v#.#.###v#.#.###.#
#.....#...#...#.#.#...#
#.#########.###.#.#.###
#...###...#...#...#.###
###.###.#.###v#####v###
#...#...#.#.>.>.#.>.###
#.###.###.#.###.#.#v###
#.....###...###...#...#
#####################.#
"#;
    #[test]
    fn parsing() {
        let parsed = parse(INPUT.trim());
        assert_eq!(parsed[0][1], Tile::Path);
    }
    #[test]
    fn part1_test() {
        let parsed = parse(INPUT.trim());
        assert_eq!(part1(parsed), 94);
    }
}
