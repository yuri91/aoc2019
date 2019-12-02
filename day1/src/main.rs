use anyhow::Result;

fn parse() -> Result<Vec<i32>> {
    std::fs::read_to_string("input")?
        .trim()
        .split('\n')
        .map(|s| s.parse().map_err(std::convert::From::from))
        .collect()
}

fn fuel(mass: i32) -> i32 {
    mass/3 - 2
}

fn fuel_adj(mut mass: i32) -> i32 {
    let mut tot = 0;
    while mass > 0 {
        mass = std::cmp::max(fuel(mass), 0);
        tot += mass;
    }
    tot
}

fn part1(v: &[i32]) -> impl std::fmt::Display {
    v.iter().cloned().map(fuel).sum::<i32>()
}

fn part2(v: &[i32]) -> impl std::fmt::Display {
    v.iter().cloned().map(fuel_adj).sum::<i32>()
}

fn main() -> Result<()> {
    let v = parse()?;
    let p1 = part1(&v);
    println!("part 1: {}", p1);
    let p2 = part2(&v);
    println!("part 2: {}", p2);
    Ok(())
}
