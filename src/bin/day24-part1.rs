use glam::{DMat2, DVec2, DVec3, I64Vec3, Vec3Swizzles};
use itertools::Itertools;
use nalgebra::{Matrix2, Vector2};
use nom::bytes::complete::tag;
use nom::character::complete::{i64, line_ending, multispace1};
use nom::multi::separated_list1;
use nom::sequence::separated_pair;
use nom::{Finish, Parser};
use std::fs::read_to_string;

#[derive(Debug, Clone, Copy, PartialEq)]
struct Hail {
    pos: DVec3,
    vel: DVec3,
}

fn parse(input: &str) -> Vec<Hail> {
    let vec3 = |i| {
        separated_list1(tag(",").and(multispace1), i64)
            .map(|v| I64Vec3::from_slice(&v[..]))
            .parse(i)
    };
    let hail = separated_pair(vec3, tag(" @ "), vec3).map(|(pos, vel)| Hail {
        pos: DVec3::new(pos.x as f64, pos.y as f64, pos.z as f64),
        vel: DVec3::new(vel.x as f64, vel.y as f64, vel.z as f64),
    });
    let mut parser = separated_list1(line_ending, hail);
    let (_rest, res) = parser
        .parse(input)
        .finish()
        .map_err(|e: nom::error::Error<&str>| e)
        .expect("--input parsed--");
    res
}
fn hails_intersection(h1: &Hail, h2: &Hail) -> Option<DVec2> {
    let a = Matrix2::from(DMat2::from_cols(-h1.vel.xy(), h2.vel.xy()));
    let b = Vector2::from((h1.pos - h2.pos).xy());
    let decomp = a.lu();
    decomp
        .solve(&b)
        // consider only future time
        .filter(|t| (t.x >= 0. && t.y >= 0.))
        .map(|t| h1.pos.xy() + t.x * h1.vel.xy())
}
fn part1(hails: &[Hail], bounds: (f64, f64)) -> usize {
    hails
        .iter()
        .combinations(2)
        .filter_map(|pair| hails_intersection(pair[0], pair[1]))
        .filter(|&sol| sol.x > bounds.0 && sol.x < bounds.1 && sol.y > bounds.0 && sol.y < bounds.1)
        //.inspect(|sol| println!("sol={sol}"))
        .count()
}

fn main() {
    let input = read_to_string("inputs/day24-input1.txt").unwrap();
    let parsed = parse(&input);
    let answer = part1(&parsed, (200000000000000., 400000000000000.));
    println!("answer is: {answer}");
}
#[cfg(test)]
mod tests {

    use super::*;

    const INPUT: &str = r#"
19, 13, 30 @ -2,  1, -2
18, 19, 22 @ -1, -1, -2
20, 25, 34 @ -2, -2, -4
12, 31, 28 @ -1, -2, -1
20, 19, 15 @  1, -5, -3
"#;
    #[test]
    fn parsing() {
        let parsed = parse(INPUT.trim());
        assert_eq!(
            parsed[0],
            Hail {
                pos: DVec3::new(19., 13., 30.),
                vel: DVec3::new(-2., 1., -2.),
            }
        );
    }
    #[test]
    fn part1_test() {
        let parsed = parse(INPUT.trim());
        assert_eq!(part1(&parsed, (7., 27.)), 2);
    }
}
