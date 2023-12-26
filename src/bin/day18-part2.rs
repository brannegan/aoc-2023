use std::fs::read_to_string;

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
    len: i32,
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
fn part1(parsed: &[DigInstruction]) -> usize {
    let mut pit = dig(parsed);
    fill_area(&mut pit);
    volume(&pit)
}
fn volume(pit: &[Vec<u32>]) -> usize {
    pit.iter()
        .flat_map(|row| row.iter())
        .filter(|val| **val > 0)
        .count()
}
fn fill_area(pit: &mut [Vec<u32>]) {
    let h = pit.len();
    let w = pit[0].len();
    for r in 1..h - 1 {
        let mut edges_count = 0;
        let mut down = false;
        let mut up = false;
        for c in 1..w - 1 {
            down = down || pit[r + 1][c] > 0 && pit[r][c] > 0;
            up = up || pit[r - 1][c] > 0 && pit[r][c] > 0;
            if up && down {
                edges_count += 1;
                down = false;
                up = false;
            }
            if edges_count % 2 == 1 && pit[r][c] == 0 {
                pit[r][c] = 1;
                down = false;
                up = false;
            } else if edges_count % 2 == 0 && pit[r][c] == 0 {
                down = false;
                up = false;
            }
        }
    }
}
fn dig(parsed: &[DigInstruction]) -> Vec<Vec<u32>> {
    let ((max_w, max_h), (min_w, min_h)) = dimensions(parsed);

    let mut res: Vec<Vec<u32>> =
        vec![vec![0; (max_w - min_w + 3) as usize]; (max_h - min_h + 3) as usize];
    parsed.iter().fold(
        (
            min_h.unsigned_abs() as usize + 1,
            min_w.unsigned_abs() as usize + 1,
        ),
        |mut acc: (usize, usize), instr: &DigInstruction| {
            match instr.dir {
                Dir::Up => {
                    (acc.0 - instr.len as usize..=acc.0).for_each(|i| res[i][acc.1] = instr.color);
                    acc.0 -= instr.len as usize;
                }
                Dir::Down => {
                    (acc.0..=acc.0 + instr.len as usize).for_each(|i| res[i][acc.1] = instr.color);
                    acc.0 += instr.len as usize;
                }
                Dir::Right => {
                    (acc.1..=acc.1 + instr.len as usize).for_each(|i| res[acc.0][i] = instr.color);
                    acc.1 += instr.len as usize;
                }
                Dir::Left => {
                    (acc.1 - instr.len as usize..=acc.1).for_each(|i| res[acc.0][i] = instr.color);
                    acc.1 -= instr.len as usize;
                }
            };
            acc
        },
    );

    res
}
fn _print_field(f: &[Vec<u32>]) {
    for row in f {
        for c in row {
            if c != &0 {
                print!("#")
            } else {
                print!(".")
            }
        }
        println!()
    }
}
fn dimensions(parsed: &[DigInstruction]) -> ((i32, i32), (i32, i32)) {
    let mut w = 0i32;
    let mut h = 0i32;
    parsed.iter().fold(
        ((0, 0), (i32::MAX, i32::MAX)),
        |((mut max_w, mut max_h), (mut min_w, mut min_h)), instr| {
            match instr.dir {
                Dir::Up => {
                    h -= instr.len;
                }
                Dir::Down => {
                    h += instr.len;
                }
                Dir::Right => {
                    w += instr.len;
                }
                Dir::Left => {
                    w -= instr.len;
                }
            };
            max_w = max_w.max(w);
            max_h = max_h.max(h);
            min_w = min_w.min(w);
            min_h = min_h.min(h);
            ((max_w, max_h), (min_w, min_h))
        },
    )
}

fn main() {
    let input = read_to_string("inputs/day18-input1.txt").unwrap();
    let parsed = parse(&input);
    let answer = part1(&parsed);
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

    #[test]
    fn parsing() {
        let parsed = parse(INPUT.trim());
        let test = DigInstruction {
            dir: Dir::Right,
            len: 6,
            color: 0x70c710,
        };
        assert_eq!(parsed[0], test);
    }
    #[test]
    fn part1_test() {
        let parsed = parse(INPUT.trim());
        assert_eq!(part1(&parsed), 62);
    }
}
