use glam::{i64, DVec3, I64Vec3};
use nalgebra::{Matrix6, Vector3, Vector6};
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
fn part2(hails: &[Hail]) -> i64 {
    // p0 + ti * v0 = pi + ti * vi -> p0 - pi = ti * (vi - v0)
    // -> [p0 - pi] ||(parralel) [vi - v0] because _ti_ is scalar
    // -> [p0 - pi] x [vi - v0] = 0
    // -> [p0 - p1] x [v1 - v0] = 0 && [p0 - p2] x [v2 - v0] = 0
    // -> -p0 x v0 + pi x v0 + vi x p0 - pi x vi = 0
    // -> pi x v0 + vi x p0 = p0 x v0 + pi x vi
    // -> vi x p0 + pi x v0 = p0 x v0 + pi x vi
    //    viy*p0z - viz*p0y + piy*v0z - piz*v0y,
    //    viz*p0x - vix*p0z + piz*v0x - pix*v0z,
    //    vix*p0y - viy*p0x + pix*v0y - piy*v0x
    // ->  0  *p0x - viz*p0y + viy*p0z + 0  *v0x - piz*v0y + piy*v0z,
    //     viz*p0x + 0  *p0y - vix*p0z + piz*v0x + 0  *v0y - pix*v0z,
    //    -viy*p0x + vix*p0y + 0  *p0z - piy*v0x + pix*v0y + 0  *v0z
    // a x b = [ay*bz - az*by, az*bx - ax*bz, ax*by - ay*bx]
    let p0 = hails[0].pos;
    let v0 = hails[0].vel;
    let p1 = hails[1].pos;
    let v1 = hails[1].vel;
    let p2 = hails[2].pos;
    let v2 = hails[2].vel;

    let b1 = Vector3::from(-p0.cross(v0) + p1.cross(v1));
    let b2 = Vector3::from(-p0.cross(v0) + p2.cross(v2));
    let b = Vector6::from_iterator(b1.iter().copied().chain(b2.into_iter().copied()));
    let v1 = v1 - v0;
    let p1 = p1 - p0;
    let v2 = v2 - v0;
    let p2 = p2 - p0;

    #[rustfmt::skip]
    let a = Matrix6::from_row_slice(&[
            0.,    v1.z, -v1.y,  0.,   -p1.z,  p1.y,
           -v1.z,  0.,    v1.x,  p1.z,  0.,   -p1.x,
            v1.y, -v1.x,  0.,   -p1.y,  p1.x,  0.,
            0.,    v2.z, -v2.y,  0.,   -p2.z,  p2.y,
           -v2.z,  0.,    v2.x,  p2.z,  0.,   -p2.x,
            v2.y, -v2.x,  0.,   -p2.y,  p2.x,  0.,
    ]);

    let res = a.try_inverse().unwrap() * b;
    let rock = Hail {
        pos: DVec3::from(Vector3::from_iterator(
            res.view((0, 0), (3, 1)).into_iter().copied(),
        )),
        vel: DVec3::from(Vector3::from_iterator(
            res.view((3, 0), (3, 1)).into_iter().copied(),
        )),
    };
    let tx = ((hails[0].pos.x - rock.pos.x) / (rock.vel.x - hails[0].vel.x)).round();
    let ty = ((hails[0].pos.y - rock.pos.y) / (rock.vel.y - hails[0].vel.y)).round();
    let tz = ((hails[0].pos.z - rock.pos.z) / (rock.vel.z - hails[0].vel.z)).round();
    assert_eq!(tx, ty);
    assert_eq!(tx, tz);
    // fix floating point error by recalculating rock position as i64vec3
    let rock_pos = hails[0].pos.as_i64vec3() + tx as i64 * (hails[0].vel - rock.vel).as_i64vec3();
    assert_eq!(
        rock_pos + tx as i64 * rock.vel.as_i64vec3(),
        hails[0].pos.as_i64vec3() + tx as i64 * hails[0].vel.as_i64vec3()
    );
    rock_pos.x + rock_pos.y + rock_pos.z
}

fn main() {
    let input = read_to_string("inputs/day24-input1.txt").unwrap();
    let parsed = parse(&input);
    let answer = part2(&parsed);
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
    fn part2_test() {
        let parsed = parse(INPUT.trim());
        assert_eq!(part2(&parsed), 47);
    }
}
