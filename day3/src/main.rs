use anyhow::{anyhow, Result};
use std::collections::HashSet;

fn parse() -> Result<Vec<Vec<Segment>>> {
    std::fs::read_to_string("input")?
        .trim()
        .split('\n')
        .map(|l| l.split(',').map(Segment::parse).collect())
        .collect()
}

#[derive(Clone, Copy)]
enum Segment {
    H(i32),
    V(i32),
}
impl Segment {
    fn parse(s: &str) -> Result<Segment> {
        let dir = s.bytes().nth(0).ok_or_else(|| anyhow!("parsing error"))?;
        let n: i32 = s[1..].parse()?;
        Ok(match dir {
            b'R' => Segment::H(n),
            b'L' => Segment::H(-n),
            b'U' => Segment::V(n),
            b'D' => Segment::V(-n),
            _ => {
                return Err(anyhow!("parsing_error"));
            }
        })
    }
}

fn collect_wire(v: Vec<Segment>) -> HashSet<(i32, i32)> {
    let mut s = HashSet::new();
    let mut pos = (0, 0);
    for i in v {
        match i {
            Segment::H(n) => {
                let dir = n.signum();
                for _ in 0..n.abs() {
                    pos = (pos.0 + dir, pos.1);
                    s.insert(pos);
                }
            }
            Segment::V(n) => {
                let dir = n.signum();
                for _ in 0..n.abs() {
                    pos = (pos.0, pos.1 + dir);
                    s.insert(pos);
                }
            }
        }
    }
    s
}
fn length_manhattan(v: &(i32, i32)) -> i32 {
    v.0.abs() + v.1.abs()
}

fn part1(v: Vec<Vec<Segment>>) -> Result<impl std::fmt::Display> {
    assert_eq!(v.len(), 2);
    let mut v = v.into_iter();
    let s1 = collect_wire(v.next().unwrap());
    let s2 = collect_wire(v.next().unwrap());

    s1.intersection(&s2)
        .map(length_manhattan)
        .min()
        .ok_or_else(|| anyhow!("no intersection!"))
}

fn length_wire(v: &(i32, i32), w: &[Segment]) -> i32 {
    let mut pos = (0, 0);
    let mut len = 0;
    for s in w {
        match s {
            Segment::H(n) => {
                let dir = n.signum();
                for _ in 0..n.abs() {
                    pos = (pos.0 + dir, pos.1);
                    len += 1;
                    if pos == *v {
                        return len;
                    }
                }
            }
            Segment::V(n) => {
                let dir = n.signum();
                for _ in 0..n.abs() {
                    pos = (pos.0, pos.1 + dir);
                    len += 1;
                    if pos == *v {
                        return len;
                    }
                }
            }
        }
    }
    unreachable!("not an actual point on the wire!");
}

fn part2(v: Vec<Vec<Segment>>) -> Result<impl std::fmt::Display> {
    assert_eq!(v.len(), 2);
    let mut v = v.into_iter();
    let v1 = v.next().unwrap();
    let v2 = v.next().unwrap();
    let s1 = collect_wire(v1.clone());
    let s2 = collect_wire(v2.clone());
    let length_delay = |p| {
        length_wire(p, &v1) + length_wire(p, &v2)
    };

    s1.intersection(&s2)
        .map(length_delay)
        .min()
        .ok_or_else(|| anyhow!("no intersection!"))
}

fn main() -> Result<()> {
    let v = parse()?;
    let p1 = part1(v.clone())?;
    println!("part 1: {}", p1);
    let p2 = part2(v)?;
    println!("part 2: {}", p2);
    Ok(())
}
