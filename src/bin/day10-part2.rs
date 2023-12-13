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
    Path,
    Outside,
    Inside,
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
                Tile::Path => '#',
                Tile::Ground => '.',
                Tile::Outside => 'O',
                Tile::Inside => 'I',
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

fn replace_start_tile(begin: (usize, usize), end: (usize, usize)) -> Tile {
    let diff_y = begin.0 as isize - end.0 as isize;
    let diff_x = begin.1 as isize - end.1 as isize;
    match (diff_x, diff_y) {
        (0, 2) | (0, -2) => Tile::Dir(['W', 'E']),
        (2, 0) | (-2, 0) => Tile::Dir(['N', 'S']),
        (1, 1) => Tile::Dir(['S', 'W']),
        (-1, -1) => Tile::Dir(['N', 'E']),
        (-1, 1) => Tile::Dir(['S', 'E']),
        (1, -1) => Tile::Dir(['N', 'W']),
        _ => todo!(),
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

fn part2(mut pipes: Vec<Vec<Tile>>) -> usize {
    let [start, begin, end] = start_pos(&pipes);
    let h = pipes.len();
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
    let start_tile = replace_start_tile(begin, end);
    pipes[start.0][start.1] = start_tile;
    print_pipes(&pipes);
    loop_path.push(NodeIndex::new(start.0 * w + start.1));
    let loop_coords: Vec<_> = loop_path
        .iter()
        .map(|node| (node.index() / w, node.index() % w))
        .collect();
    let crossings = (0..h)
        .cartesian_product(0..w)
        .filter(|coord| !loop_coords.contains(coord))
        .map(|(r, c)| {
            let crossing = loop_coords
                .iter()
                .filter(move |(y, _)| *y == r)
                .filter(|(y, x)| {
                    !matches!(
                        pipes[*y][*x],
                        Tile::Dir(['N', 'E']) | Tile::Dir(['N', 'W']) | Tile::Dir(['W', 'E'])
                    )
                })
                .filter(|(_, x)| *x < c)
                .count();
            ((r, c), crossing)
        })
        .collect::<Vec<_>>();

    for (r, c) in loop_coords {
        pipes[r][c] = Tile::Path;
    }
    for ((r, c), crossing) in crossings.iter() {
        if crossing % 2 == 0 {
            pipes[*r][*c] = Tile::Outside;
        } else {
            pipes[*r][*c] = Tile::Inside;
        }
    }
    print_pipes(&pipes);
    crossings
        .iter()
        .filter(|(_, crossing)| crossing % 2 > 0)
        .count()
}

fn main() {
    let input = read_to_string("inputs/day10-input1.txt").unwrap();
    let pipes = parse(&input);
    let answer = part2(pipes);
    println!("answer is: {answer}");
}
#[cfg(test)]
mod tests {

    use super::*;

    const INPUT1: &str = r#"
...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........
"#;
    const INPUT2: &str = r#"
.F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...
"#;
    const INPUT3: &str = r#"
FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L
"#;
    // ...F7.
    // F--JL7
    // L--7.|F-7
    // ...|.LJ.|
    // ...L7F--J
    // ....LJ...
    #[test]
    fn tile_display() {
        let tile = Tile::Dir(['N', 'S']);
        println!("{tile}");
        assert_eq!(format!("{tile}"), "|");
    }
    #[test]
    fn part2_test1() {
        let pipes = parse(INPUT1.trim());
        assert_eq!(part2(pipes), 4);
    }
    #[test]
    fn part2_test2() {
        let pipes = parse(INPUT2.trim());
        assert_eq!(part2(pipes), 8);
    }
    #[test]
    fn part2_test3() {
        let pipes = parse(INPUT3.trim());
        assert_eq!(part2(pipes), 10);
    }
}
