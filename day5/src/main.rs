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
    let mut vm = intcode::Vm::new(v.to_owned());
    vm.add_inputs(&[1]);
    vm.run()?;
    let mut outs: Vec<_> = vm.get_outputs().collect();
    let last = outs.pop().ok_or_else(|| anyhow!("no outputs!"))?;
    for (i,o) in outs.into_iter().enumerate() {
        if o != 0  {
            return Err(anyhow!("failed test {} with code {}!", i, o));
        }
    }
    Ok(last)
}

fn part2(v: &[i32]) -> Result<impl std::fmt::Display> {
    let mut vm = intcode::Vm::new(v.to_owned());
    vm.add_inputs(&[5]);
    vm.run()?;
    let mut outs: Vec<_> = vm.get_outputs().collect();
    let last = outs.pop().ok_or_else(|| anyhow!("no outputs!"))?;
    if !outs.is_empty() {
        return Err(anyhow!("More than one output!"));
    }
    Ok(last)
}

fn main() -> Result<()> {
    let v = parse()?;
    let p1 = part1(&v)?;
    println!("part 1: {}", p1);
    let p2 = part2(&v)?;
    println!("part 2: {}", p2);
    Ok(())
}
