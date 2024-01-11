use std::fmt::Display;
use std::fs::read_to_string;

use itertools::Itertools;
use petgraph::matrix_graph::NodeIndex;
use petgraph::{Directed, Graph};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Forest,
    Path,
    LSlope,
    RSlope,
    USlope,
    DSlope,
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

fn construct_graph(map: &Map) -> Graph<(Tile, (usize, usize)), f32, Directed> {
    let mut graph = Graph::new();
    let w = map[0].len();
    let h = map.len();
    (0..h).cartesian_product(0..w).for_each(|(r, c)| {
        graph.add_node((map[r][c], (r, c)));
    });
    (0..h).cartesian_product(0..w).for_each(|(r, c)| {
        if c + 1 < w && !matches!(map[r][c], Tile::Forest) && !matches!(map[r][c + 1], Tile::Forest)
        {
            graph.add_edge(NodeIndex::new(r * w + c), NodeIndex::new(r * w + c + 1), 0.);
            graph.add_edge(NodeIndex::new(r * w + c + 1), NodeIndex::new(r * w + c), 0.);
        }
        if r + 1 < h && !matches!(map[r][c], Tile::Forest) && !matches!(map[r + 1][c], Tile::Forest)
        {
            graph.add_edge(
                NodeIndex::new(r * w + c),
                NodeIndex::new((r + 1) * w + c),
                0.,
            );
            graph.add_edge(
                NodeIndex::new((r + 1) * w + c),
                NodeIndex::new(r * w + c),
                0.,
            );
        }
    });
    graph.retain_nodes(|g, ni| g[ni].0 != Tile::Forest);
    graph
}

fn parse(input: &str) -> Map {
    input
        .lines()
        .map(|line| line.chars().map(Tile::from).collect())
        .collect()
}

fn path_to_exit(
    g: &Graph<(Tile, (usize, usize)), f32>,
    from: NodeIndex<u32>,
    to: NodeIndex<u32>,
) -> Vec<NodeIndex<u32>> {
    petgraph::algo::astar(g, from, |node_i| node_i == to, |e| *e.weight(), |_| 0.)
        .expect("reached exit point")
        .1
}
fn find_node_with_simple_cycle(
    g: &Graph<(Tile, (usize, usize)), f32>,
    last: NodeIndex<u32>,
) -> Option<NodeIndex<u32>> {
    g.edge_indices().find_map(|ei| {
        let (a, b) = g.edge_endpoints(ei).unwrap();
        g.find_edge(b, a).map(|_| if last != a { a } else { b })
    })
}
fn convert_to_dag(
    mut graph: Graph<(Tile, (usize, usize)), f32>,
    to: NodeIndex<u32>,
) -> Graph<(Tile, (usize, usize)), f32> {
    let mut prev_node = NodeIndex::new(0);
    while let Some(from) = find_node_with_simple_cycle(&graph, prev_node) {
        prev_node = from;
        let path = path_to_exit(&graph, from, to);
        for (a, b) in path.iter().tuple_windows() {
            if let Some(edge_to_remove) = graph.find_edge(*b, *a) {
                graph.remove_edge(edge_to_remove);
            }
        }
    }
    graph
}

fn part2(map: Map) -> usize {
    let size = map.len();
    let graph = construct_graph(&map);

    let from = graph
        .node_indices()
        .find(|ni| graph[*ni].1 == (0, 1))
        .expect("start");
    let to = graph
        .node_indices()
        .find(|ni| graph[*ni].1 == (size - 1, size - 2))
        .expect("exit");
    let mut dag = convert_to_dag(graph, to);
    dag.edge_weights_mut().for_each(|w| *w = -1.);
    let paths = petgraph::algo::bellman_ford(&dag, from).expect("no negative cycles");
    let max_weight = paths
        .distances
        .iter()
        .min_by(|a, b| a.total_cmp(b))
        .unwrap()
        .abs();
    max_weight.abs() as usize
}

fn main() {
    let input = read_to_string("inputs/day23-input1.txt").unwrap();
    let parsed = parse(&input);
    let answer = part2(parsed);
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
    fn part2_test() {
        let parsed = parse(INPUT.trim());
        assert_eq!(part2(parsed), 154);
    }
}
