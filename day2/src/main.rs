use anyhow::Result;
use anyhow::anyhow;


fn parse() -> Result<Vec<i32>> {
    std::fs::read_to_string("input")?
        .trim()
        .split(',')
        .map(|s| s.parse().map_err(std::convert::From::from))
        .collect()
}

fn part1(v: &[i32]) -> Result<impl std::fmt::Display> {
    let mut vm = intcode::Vm::new(v.to_owned(), 12, 2);
    Ok(vm.run()?)
}

fn part2(v: &[i32]) -> Result<impl std::fmt::Display> {
    for noun in 0..100 {
        for verb in 0..100 {
            let mut vm = intcode::Vm::new(v.to_owned(), noun, verb);
            if vm.run()? == 19690720 {
                return Ok(100*noun + verb);
            }
        }
    }
    Err(anyhow!("no solution!"))
}

fn main() -> Result<()> {
    let v = parse()?;
    let p1 = part1(&v)?;
    println!("part 1: {}", p1);
    let p2 = part2(&v)?;
    println!("part 2: {}", p2);
    Ok(())
}
