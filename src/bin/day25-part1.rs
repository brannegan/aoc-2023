use std::collections::{HashMap, HashSet};
use std::convert::Infallible;
use std::fs::read_to_string;

//use petgraph::dot::{Config, Dot};
use petgraph::graph::UnGraph;
use rustworkx_core::connectivity::stoer_wagner_min_cut;

#[derive(Debug, Clone)]
struct Components<'a> {
    graph: UnGraph<&'a str, ()>,
}

impl<'a> Components<'a> {
    fn new(data: HashMap<&'a str, HashSet<&'a str>>) -> Self {
        let mut graph = UnGraph::new_undirected();
        data.into_iter()
            .flat_map(|(node, connected)| connected.into_iter().map(move |other| (node, other)))
            .for_each(|(a, b)| {
                let a = graph
                    .node_indices()
                    .find(|ni| graph[*ni] == a)
                    .unwrap_or_else(|| graph.add_node(a));
                let b = graph
                    .node_indices()
                    .find(|ni| graph[*ni] == b)
                    .unwrap_or_else(|| graph.add_node(b));
                graph.add_edge(a, b, ());
            });
        Self { graph }
    }
}
fn parse(input: &str) -> Components {
    let data = input
        .lines()
        .filter_map(|line| {
            line.split_once(": ")
                .map(|(name, connected)| (name, connected.split(' ').collect::<HashSet<&str>>()))
        })
        .collect();
    Components::new(data)
}
fn part1(components: &Components) -> anyhow::Result<usize> {
    let (min_cut, partition) =
        stoer_wagner_min_cut(&components.graph, |_| Ok::<u32, Infallible>(1))?
            .ok_or(anyhow::anyhow!("less than 2 edges"))?;
    assert_eq!(min_cut, 3);
    Ok(partition.len() * (components.graph.node_count() - partition.len()))
    //println!(
    //    "{:?}",
    //    Dot::with_config(&components.graph, &[Config::EdgeNoLabel])
    //);
}

fn main() {
    let input = read_to_string("inputs/day25-input1.txt").unwrap();
    let parsed = parse(&input);
    let answer = part1(&parsed).expect("found min cut");
    println!("answer is: {answer}");
}
#[cfg(test)]
mod tests {

    use anyhow::Ok;

    use super::*;

    const INPUT: &str = r#"
jqt: rhn xhk nvd
rsh: frs pzl lsr
xhk: hfx
cmg: qnr nvd lhk bvb
rhn: xhk bvb hfx
bvb: xhk hfx
pzl: lsr hfx nvd
qnr: nvd
ntq: jqt hfx bvb xhk
nvd: lhk
lsr: lhk
rzs: qnr cmg lsr rsh
frs: qnr lhk lsr
"#;
    #[test]
    fn parsing() {
        let components = parse(INPUT.trim());
        assert_eq!(components.graph.node_count(), 15);
    }
    #[test]
    fn part1_test() -> anyhow::Result<()> {
        let parsed = parse(INPUT.trim());
        assert_eq!(part1(&parsed)?, 54);
        Ok(())
    }
}
