use anyhow::{anyhow, Result};
use num_rational::Rational32;
use std::collections::HashSet;
use std::collections::HashMap;

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
struct Pos {
    x: i32,
    y: i32,
}
impl Pos {
    fn new(x: i32, y: i32) -> Pos {
        Pos {
            x,
            y,
        }
    }
    fn slope_to(self, other: Pos) -> (i32, i32) {
        let dx = other.x - self.x;
        let dy = other.y - self.y;
        if dx == 0 {
            (0, dy.signum())
        } else {
            let r = Rational32::new(dy.abs(), dx.abs());
            (*r.denom()*dx.signum(), *r.numer()*dy.signum())
        }
    }
    fn distance2_to(self, other: Pos) -> i32 {
        let y = other.y - self.y;
        let x = other.x - self.x;
        y*y + x*x
    }
}

fn parse() -> Result<Vec<Pos>> {
    Ok(std::fs::read_to_string("input")?
        .trim()
        .split('\n')
        .enumerate()
        .flat_map(|(y,l)| l.chars().enumerate().map(move |(x,v)| (x,y,v)))
        .filter(|(_,_,v)| *v == '#')
        .map(|(x,y,_)| Pos::new(x as i32,y as i32))
        .collect())
}

fn part1(positions: Vec<Pos>) -> Result<impl std::fmt::Display> {
    let mut max = 0;
    let mut max_pos = Pos::new(0,0);
    for &p in &positions {
        let mut slopes = HashSet::new();
        for &target in &positions {
            if target == p {
                continue;
            }
            let slope = p.slope_to(target);
            slopes.insert(slope);
        }
        if max < slopes.len() {
            max = slopes.len();
            max_pos = p;
        }
        max = std::cmp::max(max, slopes.len());
    }
    Ok(format!("pos: {},{} - {} asteroids", max_pos.x, max_pos.y, max))
}


fn part2(positions: Vec<Pos>) -> Result<impl std::fmt::Display> {
    let p = Pos::new(14, 17);
    let mut slopes: HashMap<_, Vec<_>> = HashMap::new();
    for &target in &positions {
        if target == p {
            continue;
        }
        let slope = p.slope_to(target);
        let entry = slopes.entry(slope).or_default();
        entry.push(target);
    }
    let mut sorted: Vec<_> = slopes.into_iter().collect();
    sorted.sort_by_key(|(slope, _)| {
        if slope.0 < 0 {
            (3, Rational32::new(slope.1, slope.0))
        } else if slope.0 > 0 {
            (1, Rational32::new(slope.1, slope.0))
        } else if slope.1 > 0 {
            (2, Rational32::new(0, 1))
        } else {
            (0, Rational32::new(0, 1))
        }
    });
    for (_, v) in &mut sorted {
        v.sort_by_key(|target| -p.distance2_to(*target));
    }
    let mut count = 0;
    'main: loop {
        for (_, v) in &mut sorted {
            if let Some(target) = v.pop() {
                count+=1;
                if count == 200 {
                    return Ok(target.x*100+target.y);
                }
            }
        }
    }
}

fn main() -> Result<()> {
    let v = parse()?;
    let p1 = part1(v.clone())?;
    println!("part 1: {}", p1);
    let p2 = part2(v)?;
    println!("part 2: {}", p2);
    Ok(())
}
