use std::collections::{HashMap, VecDeque};
use std::fs::read_to_string;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum PulseKind {
    High,
    Low,
}
#[derive(Debug, Clone, PartialEq, Eq)]
enum Kind<'a> {
    Broadcaster,
    Debug,
    FlipFlop(bool),
    Conjunction(HashMap<&'a str, PulseKind>),
}
#[derive(Debug, Clone, PartialEq, Eq)]
struct Module<'a> {
    kind: Kind<'a>,
    dst: Vec<&'a str>,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Pulse<'a> {
    from: &'a str,
    to: &'a str,
    kind: PulseKind,
}
fn parse(input: &str) -> HashMap<&str, Module> {
    input
        .lines()
        .filter_map(|line| line.split_once(" -> "))
        .map(|(module, dst)| {
            let dst = dst.split(", ").collect();
            match module {
                "broadcaster" => (
                    module,
                    Module {
                        kind: Kind::Broadcaster,
                        dst,
                    },
                ),
                "output" => (
                    module,
                    Module {
                        kind: Kind::Debug,
                        dst,
                    },
                ),
                name if name.starts_with('&') => (
                    &name[1..],
                    Module {
                        kind: Kind::Conjunction(HashMap::new()),
                        dst,
                    },
                ),
                name if name.starts_with('%') => (
                    &name[1..],
                    Module {
                        kind: Kind::FlipFlop(false),
                        dst,
                    },
                ),
                _ => unimplemented!("no other modules yet"),
            }
        })
        .collect()
}
fn init_conjunctions<'a>(modules: &mut HashMap<&'a str, Module<'a>>) {
    let conjunctions: Vec<_> = modules
        .iter()
        .filter(|(_, m)| matches!(m.kind, Kind::Conjunction(_)))
        .map(|(k, _)| *k)
        .collect();
    for c in conjunctions.iter() {
        let module_targets_conj: Vec<_> = modules
            .iter()
            .filter(|(_, module)| module.dst.iter().any(|dst| dst == c))
            .map(|(name, _)| *name)
            .collect();
        if let Some(module) = modules.get_mut(*c) {
            module.kind = Kind::Conjunction(HashMap::from_iter(
                module_targets_conj.into_iter().map(|t| (t, PulseKind::Low)),
            ));
        }
    }
}
fn push_button<'a>(modules: &mut HashMap<&'a str, Module<'a>>) -> (usize, usize) {
    let mut queue = VecDeque::from([Pulse {
        from: "button",
        to: "broadcaster",
        kind: PulseKind::Low,
    }]);
    let mut pulse_counter = (0, 0);
    while let Some(pulse) = queue.pop_front() {
        //println!("{pulse:?}");
        if pulse.kind == PulseKind::High {
            pulse_counter.0 += 1;
        } else {
            pulse_counter.1 += 1;
        }
        let Some(module) = modules.get_mut(pulse.to) else {
            continue;
        };
        let mut kind_to_send = PulseKind::Low;
        match module.kind {
            Kind::Broadcaster => {}
            Kind::Debug => println!("received {pulse:?}"),
            Kind::FlipFlop(on) => {
                if pulse.kind == PulseKind::Low {
                    if !on {
                        kind_to_send = PulseKind::High;
                    }
                    module.kind = Kind::FlipFlop(!on)
                } else {
                    continue;
                }
            }
            Kind::Conjunction(ref mut state) => {
                state.insert(pulse.from, pulse.kind);
                if state.iter().any(|(_, kind)| *kind == PulseKind::Low) {
                    kind_to_send = PulseKind::High;
                }
            }
        };
        queue.extend(module.dst.iter().map(|to| Pulse {
            kind: kind_to_send,
            from: pulse.to,
            to,
        }));
        //println!("{queue:?}");
    }
    pulse_counter
}
fn part1<'a>(mut modules: HashMap<&'a str, Module<'a>>) -> usize {
    init_conjunctions(&mut modules);
    let mut pulses = (0, 0);
    for _ in 0..1000 {
        let res = push_button(&mut modules);
        pulses.0 += res.0;
        pulses.1 += res.1;
    }
    pulses.0 * pulses.1
}
fn main() {
    let input = read_to_string("inputs/day20-input1.txt").unwrap();
    let modules = parse(&input);
    let answer = part1(modules);
    println!("answer is: {answer}");
}
#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = r#"
broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a
"#;
    const INPUT2: &str = r#"
broadcaster -> a
%a -> inv, con
&inv -> b
%b -> con
&con -> output
"#;
    #[test]
    fn parsing() {
        let modules = parse(INPUT.trim());
        assert_eq!(
            modules["broadcaster"],
            Module {
                kind: Kind::Broadcaster,
                dst: vec!["a", "b", "c"]
            }
        )
    }
    #[test]
    fn part1_test() {
        let parsed = parse(INPUT.trim());
        assert_eq!(part1(parsed), 32000000);
        let parsed = parse(INPUT2.trim());
        assert_eq!(part1(parsed), 11687500);
    }
}
