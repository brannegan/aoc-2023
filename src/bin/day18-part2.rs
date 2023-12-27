use std::fs::read_to_string;

use glam::I64Vec2;
use itertools::Itertools;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Dir {
    Up,
    Down,
    Right,
    Left,
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct DigInstruction {
    dir: Dir,
    len: i64,
    color: u32,
}

fn parse(input: &str) -> Vec<DigInstruction> {
    input
        .lines()
        .map(|line| line.splitn(3, ' '))
        .map(|mut it| {
            let dir = it
                .next()
                .map(|ch| match ch {
                    "U" => Dir::Up,
                    "D" => Dir::Down,
                    "R" => Dir::Right,
                    "L" => Dir::Left,
                    _ => unimplemented!(),
                })
                .unwrap();
            let len = it.next().map(|len| len.parse().unwrap()).unwrap();
            let color = it
                .next()
                .map(|color| color.strip_prefix("(#").unwrap().strip_suffix(')').unwrap())
                .map(|color| u32::from_str_radix(color, 16).unwrap())
                .unwrap();
            DigInstruction { dir, len, color }
        })
        .collect()
}
fn part2(input: &str) -> i64 {
    let parsed = parse(input);
    let decoded = decode(parsed);
    let vertices = vertices(decoded);
    let area = area(&vertices);
    assert_ne!(area, 952408144115);
    area
}

fn decode(encoded: Vec<DigInstruction>) -> Vec<DigInstruction> {
    encoded
        .into_iter()
        .map(|instr| DigInstruction {
            dir: match instr.color & 0x00000F {
                0 => Dir::Right,
                1 => Dir::Down,
                2 => Dir::Left,
                3 => Dir::Up,
                _ => unimplemented!(),
            },
            len: instr.color as i64 >> 4,
            color: instr.color,
        })
        .collect()
}
fn area(polygon: &[I64Vec2]) -> i64 {
    let perimeter: i64 = polygon
        .iter()
        .tuple_windows()
        .map(|(a, b)| (b.x - a.x + b.y - a.y).abs())
        .sum::<i64>()
        + (polygon[0].x + polygon[0].y);
    let inner_area = polygon
        .iter()
        .tuple_windows()
        .map(|(a, b)| (a.x * b.y - a.y * b.x))
        .sum::<i64>()
        .abs()
        / 2;
    inner_area + perimeter / 2 + 1 // 1 is (0,0)
}

fn vertices(dig_plan: Vec<DigInstruction>) -> Vec<I64Vec2> {
    dig_plan
        .into_iter()
        .scan(I64Vec2::ZERO, |state, instr| {
            match instr.dir {
                Dir::Up => {
                    *state += I64Vec2::new(0, instr.len);
                }
                Dir::Down => {
                    *state -= I64Vec2::new(0, instr.len);
                }
                Dir::Right => {
                    *state += I64Vec2::new(instr.len, 0);
                }
                Dir::Left => {
                    *state -= I64Vec2::new(instr.len, 0);
                }
            };
            Some(*state)
        })
        .collect()
}
fn main() {
    let input = read_to_string("inputs/day18-input1.txt").unwrap();
    let answer = part2(&input);
    println!("answer is: {answer}");
}
#[cfg(test)]
mod tests {

    use super::*;

    const INPUT: &str = r#"
R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)
"#;

    const FORMULA: &str = r#"
R 3 (#70c710)
D 3 (#0dc571)
L 3 (#5713f0)
U 3 (#d2c081)
"#;
    //   3
    // ###*
    // ###* 3
    // ###*
    // 0***
    //

    #[test]
    fn decoding() {
        let parsed = parse(INPUT.trim());
        let decoded = decode(parsed);
        assert_eq!(0x70c710 >> 4, 461937);
        let test = DigInstruction {
            dir: Dir::Right,
            len: 461937,
            color: 0x70c710,
        };
        assert_eq!(decoded[0], test);
        let test = DigInstruction {
            dir: Dir::Down,
            len: 56407,
            color: 0x0DC571,
        };
        assert_eq!(decoded[1], test);
    }
    #[test]
    fn vertices_test() {
        let parsed = parse(INPUT.trim());
        let decoded = decode(parsed);
        let vertices = vertices(decoded);
        assert_eq!(vertices[0], I64Vec2::new(461937, 0));
        assert_eq!(vertices[1], I64Vec2::new(461937, 56407));
        assert_eq!(vertices.last(), Some(&I64Vec2::new(0, 0)));
    }
    #[test]
    fn vertices_test_nodecode() {
        let parsed = parse(INPUT.trim());
        let vertices = vertices(parsed);
        assert_eq!(vertices[0], I64Vec2::new(6, 0));
        assert_eq!(vertices[1], I64Vec2::new(6, 5));
        assert_eq!(vertices[2], I64Vec2::new(4, 5));
        assert_eq!(vertices.last(), Some(&I64Vec2::new(0, 0)));
    }
    #[test]
    fn area_small() {
        let parsed = parse(INPUT.trim());
        let vertices = vertices(parsed);
        let area = area(&vertices);
        assert_eq!(area, 62);
    }
    #[test]
    fn area_formula() {
        let parsed = parse(FORMULA.trim());
        let vertices = vertices(parsed);
        let area = area(&vertices);
        assert_eq!(area, 16);
    }
    #[test]
    fn area_big() {
        let parsed = parse(INPUT.trim());
        let decoded = decode(parsed);
        let vertices = vertices(decoded);
        let area = area(&vertices);
        assert_eq!(area, 952408144115);
    }
    #[test]
    fn part2_test() {
        assert_eq!(part2(INPUT.trim()), 952408144115);
    }
}
