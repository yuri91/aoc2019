use anyhow::{anyhow, Result};
use std::str::FromStr;
use num::integer::lcm;
use nom::{
    IResult,
    character::complete::digit1,
    bytes::complete::tag,
    combinator::{opt, map_res, recognize},
    sequence::pair
};

fn parse() -> Result<Vec<Vec3>> {
    std::fs::read_to_string("input")?
        .trim()
        .lines()
        .map(FromStr::from_str)
        .collect()
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct Vec3 {
    x: i64,
    y: i64,
    z: i64,
}
impl Vec3 {
    fn new(x: i64, y: i64, z: i64) -> Vec3 {
        Vec3 {
            x,
            y,
            z,
        }
    }
    fn zero() -> Vec3 {
        Vec3::new(0, 0, 0,)
    }
    fn delta(&self, other: &Vec3) -> Vec3 {
        Vec3 {
            x: if self.x < other.x { 1 } else if self.x == other.x { 0 } else { -1 },
            y: if self.y < other.y { 1 } else if self.y == other.y { 0 } else { -1 },
            z: if self.z < other.z { 1 } else if self.z == other.z { 0 } else { -1 },
        }
    }
    fn add(&mut self, other: &Vec3) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }
}

fn parser(s: &str) -> IResult<&str, Vec3> {
    let int_parse = map_res(recognize(pair(opt(tag("-")), digit1)), |s: &str| s.parse::<i64>());
    let (s,_) = tag("<x=")(s)?;
    let (s,x) = int_parse(s)?;
    let (s,_) = tag(", y=")(s)?;
    let (s,y) = int_parse(s)?;
    let (s,_) = tag(", z=")(s)?;
    let (s,z) = int_parse(s)?;
    let (s,_) = tag(">")(s)?;
    Ok((s, Vec3::new(x,y,z)))
}

impl FromStr for Vec3 {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Vec3> {
        let (_, v) = parser(s).map_err(|_|anyhow!("failed to pars Vec3"))?;
        Ok(v)
    }
}

fn step(positions: &mut Vec<Vec3>, velocities: &mut Vec<Vec3>) {
    for i in 0..positions.len() {
        for j in 0..positions.len() {
            if i == j {
                continue;
            }
            let delta = positions[i].delta(&positions[j]);
            velocities[i].add(&delta);
        }
    }
    for i in 0..positions.len() {
        positions[i].add(&velocities[i]);
    }
}

fn energy(positions: &Vec<Vec3>, velocities: &Vec<Vec3>) -> i64 {
    let mut en = 0;

    for (p,v) in positions.iter().zip(velocities.iter()) {
        let enp = p.x.abs() + p.y.abs() + p.z.abs();
        let enk = v.x.abs() + v.y.abs() + v.z.abs();
        en += enp*enk;
    }
    en
}


fn part1(mut positions: Vec<Vec3>) -> Result<impl std::fmt::Display> {
    let mut velocities = Vec::new();
    velocities.resize_with(positions.len(), Vec3::zero);
    for _ in 0..1000 {
        step(&mut positions, &mut velocities);
    }
    Ok(energy(&positions, &velocities))
}

fn find_period(mut positions: Vec<i64>) -> i64 {
    let init_pos = positions.clone();
    let mut velocities = Vec::new();
    velocities.resize(positions.len(), 0);
    let init_vel = velocities.clone();
    let mut count = 0;
    loop {
        for i in 0..positions.len() {
            for j in 0..positions.len() {
                if i == j {
                    continue;
                }
                let delta = if positions[i] < positions[j] { 1 } else if positions[i] == positions[j] { 0 } else { -1 };
                velocities[i] += delta;
            }
        }
        for i in 0..positions.len() {
            positions[i] += velocities[i];
        }
        count += 1;
        if positions == init_pos && velocities == init_vel {
            break;
        }
    }
    count
}
fn part2(positions: Vec<Vec3>) -> Result<impl std::fmt::Display> {
    let positions_x = positions.iter().map(|p| p.x).collect();
    let positions_y = positions.iter().map(|p| p.y).collect();
    let positions_z = positions.iter().map(|p| p.z).collect();
    let px = find_period(positions_x);
    let py = find_period(positions_y);
    let pz = find_period(positions_z);
    let period = lcm(lcm(px, py), pz);
    Ok(period)
}

fn main() -> Result<()> {
    let v = parse()?;
    let p1 = part1(v.clone())?;
    println!("part 1: {}", p1);
    let p2 = part2(v)?;
    println!("part 2: {}", p2);
    Ok(())
}
