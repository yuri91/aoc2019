use anyhow::Result;
use anyhow::anyhow;


fn parse() -> Result<Vec<i64>> {
    std::fs::read_to_string("input")?
        .trim()
        .split(',')
        .map(|s| s.parse().map_err(std::convert::From::from))
        .collect()
}

fn part1(v: Vec<i64>) -> Result<impl std::fmt::Display> {
    let mut vm = intcode::Vm::new(v);
    vm.write_at(1, 12)?;
    vm.write_at(2, 2)?;
    vm.run()?;
    Ok(vm.read_at(0)?)
}

fn part2(v: Vec<i64>) -> Result<impl std::fmt::Display> {
    for noun in 0..100 {
        for verb in 0..100 {
            let mut vm = intcode::Vm::new(v.clone());
            vm.write_at(1, noun)?;
            vm.write_at(2, verb)?;
            vm.run()?;
            if vm.read_at(0)? == 19690720 {
                return Ok(100*noun + verb);
            }
        }
    }
    Err(anyhow!("no solution!"))
}

fn main() -> Result<()> {
    let v = parse()?;
    let p1 = part1(v.clone())?;
    println!("part 1: {}", p1);
    let p2 = part2(v.clone())?;
    println!("part 2: {}", p2);
    Ok(())
}
