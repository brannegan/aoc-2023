use std::collections::HashMap;
use std::fs::read_to_string;

use nom::character::complete::{alpha0, char, line_ending, one_of, u32};
use nom::multi::{separated_list0, separated_list1};
use nom::sequence::{delimited, separated_pair};
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
    fn fits(&self, part: [u32; 4]) -> bool {
        match &self {
            Category::X(op, val) => match op {
                Op::Le => part[0] < *val,
                Op::Gt => part[0] > *val,
            },
            Category::M(op, val) => match op {
                Op::Le => part[1] < *val,
                Op::Gt => part[1] > *val,
            },

            Category::A(op, val) => match op {
                Op::Le => part[2] < *val,
                Op::Gt => part[2] > *val,
            },

            Category::S(op, val) => match op {
                Op::Le => part[3] < *val,
                Op::Gt => part[3] > *val,
            },
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
enum Workflow<'a> {
    Rules(Vec<Rule<'a>>),
    Accept,
    Reject,
}
type Workflows<'a> = HashMap<&'a str, Workflow<'a>>;
#[derive(Debug, Clone, PartialEq)]
struct Rule<'a> {
    cat: Option<Category>,
    next: &'a str,
}
#[derive(Debug, Clone, Default)]
struct Aplenty<'a> {
    workflows: Workflows<'a>,
    parts: Vec<[u32; 4]>,
}

impl<'a> Aplenty<'a> {
    fn accepted_by_workflow(&self, workflow: &str, part: [u32; 4]) -> bool {
        match &self.workflows[workflow] {
            Workflow::Rules(rules) => {
                let rule = rules
                    .iter()
                    .find(|rule| rule.cat.is_some_and(|cat| cat.fits(part)))
                    .or(rules.last())
                    .unwrap();
                self.accepted_by_workflow(rule.next, part)
            }
            Workflow::Accept => true,
            Workflow::Reject => false,
        }
    }
}

fn parse(input: &str) -> anyhow::Result<Aplenty> {
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
    let common_rule = separated_pair(cat, char(':'), label).map(|(cat, next)| Rule {
        cat: Some(cat),
        next,
    });
    let default_rule = label.map(|next| Rule { cat: None, next });
    let rule = common_rule.or(default_rule);
    let rules = separated_list0(char(','), rule);
    let workflow = label
        .and(delimited(char('{'), rules, char('}')))
        .map(|(label, rules)| (label, Workflow::Rules(rules)));
    let workflows = separated_list1(line_ending, workflow);
    let gap = line_ending.and(line_ending);
    let xmas = separated_list0(char(','), separated_pair(one_of("xmas"), char('='), u32));
    let part = delimited(char('{'), xmas, char('}'))
        .map(|xmas| [xmas[0].1, xmas[1].1, xmas[2].1, xmas[3].1]);
    let parts = separated_list1(line_ending, part);
    let mut parser = workflows
        .and(gap)
        .and(parts)
        .map(|((workflows, _), parts)| Aplenty {
            workflows: workflows.into_iter().collect(),
            parts,
        });
    let (_input, mut aplenty) = parser
        .parse(input)
        .finish()
        .map_err(|e: nom::error::VerboseError<&str>| anyhow::anyhow!("parser error: {:?}", e))?;
    aplenty.workflows.insert("A", Workflow::Accept);
    aplenty.workflows.insert("R", Workflow::Reject);
    Ok(aplenty)
}
fn part1(aplenty: &Aplenty) -> u32 {
    aplenty
        .parts
        .iter()
        .filter(|part| aplenty.accepted_by_workflow("in", **part))
        .map(|part| part.iter().sum::<u32>())
        .sum::<u32>()
}

fn main() {
    let input = read_to_string("inputs/day19-input1.txt").unwrap();
    let parsed = parse(&input).unwrap();

    let answer = part1(&parsed);
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
        use Workflow::*;
        let parsed = parse(INPUT.trim())?;
        assert_eq!(parsed.parts[0], [787, 2655, 1222, 2876]);

        assert_eq!(
            parsed.workflows["in"],
            Rules(vec![
                Rule {
                    cat: Some(S(Le, 1351)),
                    next: "px"
                },
                Rule {
                    cat: None,
                    next: "qqz"
                }
            ])
        );
        Ok(())
    }
    #[test]
    fn accepted() -> anyhow::Result<()> {
        let aplenty = parse(INPUT.trim())?;
        assert!(aplenty.accepted_by_workflow("in", aplenty.parts[0]));
        assert!(!aplenty.accepted_by_workflow("in", aplenty.parts[1]));
        assert!(aplenty.accepted_by_workflow("in", aplenty.parts[2]));
        assert!(!aplenty.accepted_by_workflow("in", aplenty.parts[3]));
        assert!(aplenty.accepted_by_workflow("in", aplenty.parts[4]));
        Ok(())
    }
    #[test]
    fn part1_test() -> anyhow::Result<()> {
        let parsed = parse(INPUT.trim())?;
        assert_eq!(part1(&parsed), 19114);
        Ok(())
    }
}
