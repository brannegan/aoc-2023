use glam::IVec3;
use itertools::Itertools;
use std::collections::HashSet;
use std::fmt::Display;
use std::fs::read_to_string;

#[derive(Debug, PartialEq, Clone, Copy)]
struct Brick {
    a: IVec3,
    b: IVec3,
}
impl Display for Brick {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}~{}", self.a, self.b)
    }
}
impl Brick {
    fn overlaps(&self, other: &Self) -> bool {
        self.a.x <= other.b.x
            && other.a.x <= self.b.x
            && self.a.y <= other.b.y
            && other.a.y <= self.b.y
    }
    fn fall_down(&mut self) {
        self.a.z -= 1;
        self.b.z -= 1;
    }
}

fn parse(input: &str) -> Vec<Brick> {
    input
        .lines()
        .flat_map(|line| line.split_once('~'))
        .map(|(a, b)| Brick {
            a: IVec3::from_slice(
                &a.splitn(3, ',')
                    .map(|val| val.parse::<i32>().unwrap())
                    .collect::<Vec<_>>(),
            ),
            b: IVec3::from_slice(
                &b.splitn(3, ',')
                    .map(|val| val.parse::<i32>().unwrap())
                    .collect::<Vec<_>>(),
            ),
        })
        .collect()
}

fn bricks_to_fall_count(bricks: &mut [Brick]) -> usize {
    let mut falling_bricks = falling_bricks_ids(bricks);
    let mut all_fallen_bricks: HashSet<usize> = HashSet::new();
    while !falling_bricks.is_empty() {
        all_fallen_bricks.extend(&falling_bricks);
        falling_bricks
            .into_iter()
            .for_each(|i| bricks[i].fall_down());
        falling_bricks = falling_bricks_ids(bricks);
    }
    all_fallen_bricks.len()
}

fn part2(mut bricks: Vec<Brick>) -> usize {
    //initial falling
    let _ = bricks_to_fall_count(&mut bricks);

    let mut total_falls_count = 0;
    for i in 0..bricks.len() {
        let mut bricks_minus_one = bricks.clone();
        bricks_minus_one.swap_remove(i);
        total_falls_count += bricks_to_fall_count(&mut bricks_minus_one);
    }
    total_falls_count
}

fn falling_bricks_ids(bricks: &[Brick]) -> Vec<usize> {
    let highest_z = bricks.iter().max_by_key(|brick| brick.b.z).unwrap().b.z;
    (1..=highest_z)
        .rev()
        .tuple_windows()
        .flat_map(move |(z_u, z_d)| {
            bricks
                .iter()
                .enumerate()
                .filter(move |(_, above)| above.a.z == z_u)
                .filter(move |(_, above)| {
                    //346

                    !bricks
                        .iter()
                        .filter(|brick| brick.b.z == z_d)
                        .any(|below| below.overlaps(above))
                })
                .map(|(i, _)| i)
        })
        .collect()
}

fn main() {
    let input = read_to_string("inputs/day22-input1.txt").unwrap();
    let parsed = parse(&input);
    let answer = part2(parsed);
    println!("answer is: {answer}");
}
#[cfg(test)]
mod tests {

    use std::ops::Not;

    use super::*;

    const INPUT: &str = r#"
1,0,1~1,2,1
0,0,2~2,0,2
0,2,3~2,2,3
0,0,4~0,2,4
2,0,5~2,2,5
0,1,6~2,1,6
1,1,8~1,1,9
"#;
    #[test]
    fn parsing() {
        let parsed = parse(INPUT.trim());
        assert_eq!(
            parsed[0],
            Brick {
                a: IVec3::new(1, 0, 1),
                b: IVec3::new(1, 2, 1)
            }
        );
    }
    #[test]
    fn overlaps() {
        let a = Brick {
            a: IVec3::new(1, 0, 1),
            b: IVec3::new(1, 2, 1),
        };
        let b = Brick {
            a: IVec3::new(0, 0, 2),
            b: IVec3::new(2, 0, 2),
        };
        let d = Brick {
            a: IVec3::new(0, 0, 0),
            b: IVec3::new(0, 2, 0),
        };
        let e = Brick {
            a: IVec3::new(2, 0, 0),
            b: IVec3::new(2, 2, 0),
        };
        let f = Brick {
            a: IVec3::new(0, 1, 4),
            b: IVec3::new(2, 1, 4),
        };
        let g = Brick {
            a: IVec3::new(1, 1, 5),
            b: IVec3::new(1, 1, 5),
        };
        assert!(a.overlaps(&b));
        assert!(d.overlaps(&e).not());
        assert!(g.overlaps(&f));
        assert!(f.overlaps(&g));
    }
    #[test]
    fn part2_test() {
        let parsed = parse(INPUT.trim());
        assert_eq!(part2(parsed), 7);
    }
}
