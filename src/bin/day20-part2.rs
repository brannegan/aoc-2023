use itertools::Itertools;
use num::integer::lcm;
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
fn push_button_low_rx_reached<'a>(
    modules: &mut HashMap<&'a str, Module<'a>>,
    btn_pressed: usize,
    modules_to_check: &mut HashMap<&'a str, usize>,
) {
    let mut queue = VecDeque::from([Pulse {
        from: "button",
        to: "broadcaster",
        kind: PulseKind::Low,
    }]);
    while let Some(pulse) = queue.pop_front() {
        let Some(module) = modules.get_mut(pulse.to) else {
            continue;
        };
        let mut pulse_kind_to_send = PulseKind::Low;
        match module.kind {
            Kind::Broadcaster => {}
            Kind::Debug => println!("received {pulse:?}"),
            Kind::FlipFlop(on) => {
                if pulse.kind == PulseKind::Low {
                    if !on {
                        pulse_kind_to_send = PulseKind::High;
                    }
                    module.kind = Kind::FlipFlop(!on)
                } else {
                    continue;
                }
            }
            Kind::Conjunction(ref mut state) => {
                state.insert(pulse.from, pulse.kind);
                if state.iter().any(|(_, kind)| *kind == PulseKind::Low) {
                    pulse_kind_to_send = PulseKind::High;
                }
                if pulse.kind == PulseKind::High
                    && modules_to_check.iter().any(|(name, _)| *name == pulse.from)
                {
                    modules_to_check.insert(pulse.from, btn_pressed);
                }
            }
        };
        queue.extend(module.dst.iter().map(|to| Pulse {
            kind: pulse_kind_to_send,
            from: pulse.to,
            to,
        }));
    }
}
fn part2<'a>(mut modules: HashMap<&'a str, Module<'a>>) -> Option<usize> {
    init_conjunctions(&mut modules);
    let mut cycle_detection_modules: HashMap<&str, usize> =
        if let Kind::Conjunction(rx_src_modules) = &modules
            .iter()
            .find(|(_, module)| module.dst.iter().contains(&"rx"))
            .unwrap()
            .1
            .kind
        {
            rx_src_modules.iter().map(|(name, _)| (*name, 0)).collect()
        } else {
            return None;
        };
    let mut btn_pressed = 0;
    while cycle_detection_modules
        .iter()
        .any(|(_, btn_pressed)| btn_pressed == &0)
    {
        btn_pressed += 1;
        push_button_low_rx_reached(&mut modules, btn_pressed, &mut cycle_detection_modules);
    }
    cycle_detection_modules.into_values().reduce(lcm)
}
fn main() {
    let input = read_to_string("inputs/day20-input1.txt").unwrap();
    let modules = parse(&input);
    let answer = part2(modules).unwrap();
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
}
