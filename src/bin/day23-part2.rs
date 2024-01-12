use std::fmt::Display;
use std::fs::read_to_string;

use itertools::Itertools;
use petgraph::matrix_graph::NodeIndex;
use petgraph::visit::EdgeRef;
use petgraph::{Graph, Undirected};

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

fn construct_graph(map: &Map) -> Graph<(usize, usize), f32, Undirected> {
    let mut graph = Graph::new_undirected();
    let w = map[0].len();
    let h = map.len();
    (0..h).cartesian_product(0..w).for_each(|(r, c)| {
        graph.add_node((r, c));
    });
    (0..h).cartesian_product(0..w).for_each(|(r, c)| {
        if c + 1 < w && !matches!(map[r][c], Tile::Forest) && !matches!(map[r][c + 1], Tile::Forest)
        {
            graph.add_edge(NodeIndex::new(r * w + c), NodeIndex::new(r * w + c + 1), 1.);
        }
        if r + 1 < h && !matches!(map[r][c], Tile::Forest) && !matches!(map[r + 1][c], Tile::Forest)
        {
            graph.add_edge(
                NodeIndex::new(r * w + c),
                NodeIndex::new((r + 1) * w + c),
                1.,
            );
        }
    });
    graph.retain_nodes(|g, ni| map[g[ni].0][g[ni].1] != Tile::Forest);
    graph
}

fn parse(input: &str) -> Map {
    input
        .lines()
        .map(|line| line.chars().map(Tile::from).collect())
        .collect()
}

fn part2(map: Map) -> usize {
    let size = map.len();
    let graph = construct_graph(&map);

    let start = graph
        .node_indices()
        .find(|ni| graph[*ni] == (0, 1))
        .expect("start");
    let end = graph
        .node_indices()
        .find(|ni| graph[*ni] == (size - 1, size - 2))
        .expect("exit");
    let mut crossroads: Vec<_> = graph
        .node_indices()
        .filter(|a| graph.neighbors(*a).count() > 2)
        .collect();
    crossroads.push(start);
    crossroads.push(end);
    let mut graph_of_crossroads = Graph::new_undirected();
    for cr in &crossroads {
        graph_of_crossroads.add_node(graph[*cr]);
    }
    crossroads
        .iter()
        .combinations(2)
        .map(|comb| {
            let (cost, _) = petgraph::algo::astar(
                &graph,
                *comb[0],
                |ni| ni == *comb[1],
                |e| {
                    let source = e.source();
                    if crossroads.contains(&source) && source != *comb[0] && source != *comb[1] {
                        // penalize if we walk through another crossroad
                        1000
                    } else {
                        1
                    }
                },
                |_| 0,
            )
            .unwrap();
            (cost, comb)
        })
        .filter(|(cost, _)| cost < &1000) //filter penalized
        .for_each(|(cost, comb)| {
            let a = graph_of_crossroads
                .node_indices()
                .find(|ni| graph_of_crossroads[*ni] == graph[*comb[0]])
                .unwrap();
            let b = graph_of_crossroads
                .node_indices()
                .find(|ni| graph_of_crossroads[*ni] == graph[*comb[1]])
                .unwrap();
            graph_of_crossroads.add_edge(a, b, cost);
        });

    let start = graph_of_crossroads
        .node_indices()
        .find(|ni| graph_of_crossroads[*ni] == (0, 1))
        .expect("start");
    let end = graph_of_crossroads
        .node_indices()
        .find(|ni| graph_of_crossroads[*ni] == (size - 1, size - 2))
        .expect("start");
    petgraph::algo::all_simple_paths(&graph_of_crossroads, start, end, 1, None)
        .map(|path: Vec<_>| {
            path.iter()
                .tuple_windows()
                .map(|(a, b)| graph_of_crossroads[graph_of_crossroads.find_edge(*a, *b).unwrap()])
                .sum::<usize>()
        })
        .max()
        .expect("maximum weighted path")
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
