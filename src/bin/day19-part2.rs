use std::collections::HashMap;
use std::fs::read_to_string;
use std::ops::Range;

use indextree::Arena;
use nom::character::complete::{alpha0, char, line_ending, one_of, u32};
use nom::multi::{separated_list0, separated_list1};
use nom::sequence::{delimited, preceded, separated_pair};
use nom::{Finish, Parser};

#[derive(Debug, Copy, Clone, PartialEq)]
enum Op {
    Le,
    Gt,
}
#[derive(Debug, Copy, Clone, PartialEq)]
enum Category {
    X(Op, u32),
    M(Op, u32),
    A(Op, u32),
    S(Op, u32),
}
impl Category {
    fn branch(&self, mut part: [Range<u32>; 4]) -> [Range<u32>; 4] {
        match self {
            Category::X(op, val) => match op {
                Op::Le => part[0] = part[0].start..*val,
                Op::Gt => part[0] = *val + 1..part[0].end,
            },
            Category::M(op, val) => match op {
                Op::Le => part[1] = part[1].start..*val,
                Op::Gt => part[1] = *val + 1..part[1].end,
            },
            Category::A(op, val) => match op {
                Op::Le => part[2] = part[2].start..*val,
                Op::Gt => part[2] = *val + 1..part[2].end,
            },
            Category::S(op, val) => match op {
                Op::Le => part[3] = part[3].start..*val,
                Op::Gt => part[3] = *val + 1..part[3].end,
            },
        }

        part
    }
}
#[derive(Debug, Clone, PartialEq)]
struct Workflow<'a> {
    name: &'a str,
    rules: Vec<Rule<'a>>,
    default: &'a str,
}
type Workflows<'a> = HashMap<&'a str, Workflow<'a>>;
#[derive(Debug, Clone, PartialEq)]
struct Rule<'a> {
    cat: Category,
    next: &'a str,
}

fn parse(input: &str) -> anyhow::Result<Workflows> {
    let label = alpha0::<&str, nom::error::VerboseError<&str>>;
    let cat = one_of("xmas")
        .and(one_of("<>"))
        .and(u32)
        .map(|((xmas, op), val)| {
            let op = match op {
                '>' => Op::Gt,
                '<' => Op::Le,
                _ => unimplemented!("no other operators"),
            };
            match xmas {
                'x' => Category::X(op, val),
                'm' => Category::M(op, val),
                'a' => Category::A(op, val),
                's' => Category::S(op, val),
                _ => unimplemented!("only xmas"),
            }
        });
    let rule = separated_pair(cat, char(':'), label).map(|(cat, next)| Rule { cat, next });
    let default = preceded(char(','), label);
    let rules = separated_list0(char(','), rule);
    let workflow = label
        .and(delimited(char('{'), rules.and(default), char('}')))
        .map(|(name, (rules, default))| Workflow {
            name,
            rules,
            default,
        });
    let workflows = separated_list1(line_ending, workflow);
    let mut parser = workflows;
    let (_input, workflows) = parser
        .parse(input)
        .finish()
        .map_err(|e: nom::error::VerboseError<&str>| anyhow::anyhow!("parser error: {:?}", e))?;
    Ok(workflows.into_iter().map(|wf| (wf.name, wf)).collect())
}

fn opposite_ranges(mut parent: [Range<u32>; 4], current: [Range<u32>; 4]) -> [Range<u32>; 4] {
    for i in 0..4 {
        parent[i] = if current[i].start != parent[i].start {
            parent[i].start..current[i].start
        } else if current[i].end != parent[i].end {
            current[i].end..parent[i].end
        } else {
            parent[i].clone()
        }
    }
    parent
}
fn part2(wfs: &Workflows) -> usize {
    let arena = &mut Arena::new();
    let parent_id = arena.new_node(("in", [1..4001, 1..4001, 1..4001, 1..4001]));
    let mut queue = vec![(parent_id, &wfs["in"])];
    while let Some((parent_id, wf)) = queue.pop() {
        let mut wf_ranges = arena
            .get(parent_id)
            .map(|node| node.get().1.clone())
            .unwrap();
        let mut parent_ranges = wf_ranges.clone();
        for rule in &wf.rules {
            parent_ranges = opposite_ranges(parent_ranges, wf_ranges.clone());
            wf_ranges = rule.cat.branch(parent_ranges.clone());
            let next_id = parent_id.append_value((rule.next, wf_ranges.clone()), arena);
            if rule.next != "R" && rule.next != "A" {
                queue.push((next_id, &wfs[rule.next]));
            }
        }
        let next_id = parent_id.append_value(
            (wf.default, opposite_ranges(parent_ranges, wf_ranges)),
            arena,
        );
        if wf.default != "R" && wf.default != "A" {
            queue.push((next_id, &wfs[wf.default]));
        }
    }
    arena
        .iter()
        .filter(|node| node.get().0 == "A")
        .map(|node| {
            node.get()
                .1
                .iter()
                .map(|range| range.len())
                .product::<usize>()
        })
        .sum()
}

fn main() {
    let input = read_to_string("inputs/day19-input1.txt").unwrap();
    let workflows = parse(&input).unwrap();

    let answer = part2(&workflows);
    println!("answer is: {answer}");
}
#[cfg(test)]
mod tests {

    use anyhow::Ok;

    use super::*;

    const INPUT: &str = r#"
px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}
"#;
    #[test]
    fn parsing() -> anyhow::Result<()> {
        use Category::*;
        use Op::*;
        let workflows = parse(INPUT.trim())?;
        let expected = Workflow {
            name: "in",
            rules: vec![Rule {
                cat: S(Le, 1351),
                next: "px",
            }],
            default: "qqz",
        };
        assert_eq!(workflows["in"], expected);
        Ok(())
    }
    #[test]
    fn part2_test() -> anyhow::Result<()> {
        let workflows = parse(INPUT.trim())?;
        assert_eq!(part2(&workflows), 167409079868000);
        Ok(())
    }
}
