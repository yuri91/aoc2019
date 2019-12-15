use anyhow::Result;
use anyhow::anyhow;
use permutohedron::heap_recursive;


fn parse() -> Result<Vec<i64>> {
    std::fs::read_to_string("input")?
        .trim()
        .split(',')
        .map(|s| s.parse().map_err(std::convert::From::from))
        .collect()
}

fn run_amps(prog: Vec<i64>, params: Vec<i64>) -> Result<i64> {
    let mut val = 0;
    for p in params {
        let mut vm = intcode::Vm::new(prog.clone());
        vm.add_inputs(&[p, val]);
        vm.run()?;
        val = vm.get_outputs().next().ok_or_else(|| anyhow!("no output"))?;
    };
    Ok(val)
}
fn part1(v: Vec<i64>) -> Result<impl std::fmt::Display> {
    let mut data = [0, 1, 2, 3, 4];
    let mut perms = Vec::new();
    heap_recursive(&mut data, |perm| {
        perms.push(perm.to_vec());
    });
    let results = perms.into_iter().map(|p| run_amps(v.clone(), p)).collect::<Result<Vec<_>>>()?;
    Ok(results.into_iter().max().unwrap())
}

fn run_amps_loop(prog: Vec<i64>, params: Vec<i64>) -> Result<i64> {
    let mut amps: Vec<_> = params.into_iter().map(|p| {
        let mut vm = intcode::Vm::new(prog.clone());
        vm.add_inputs(&[p]);
        vm
    }).collect();
    amps[0].add_inputs(&[0]);
    let mut running = true;
    while running {
        running = false;
        for i in 0..5i64 {
            if !amps[i as usize].is_running() {
                continue;
            }
            let ins: Vec<_> = amps[((i - 1 + 5) % 5) as usize].get_outputs().collect();
            amps[i as usize].add_inputs(&ins);
            loop {
                match amps[i as usize].step()? {
                    intcode::VmState::Running => { running = true;},
                    intcode::VmState::Stopped => { break; },
                    intcode::VmState::WaitingForInput => { break; },
                }
            }
        }
    }
    let val = amps[4].get_outputs().next().ok_or_else(|| anyhow!("no output"))?;
    Ok(val)
}
fn part2(v: Vec<i64>) -> Result<impl std::fmt::Display> {
    let mut data = [5, 6, 7, 8, 9];
    let mut perms = Vec::new();
    heap_recursive(&mut data, |perm| {
        perms.push(perm.to_vec());
    });
    let results = perms.into_iter().map(|p| run_amps_loop(v.clone(), p)).collect::<Result<Vec<_>>>()?;
    Ok(results.into_iter().max().unwrap())
}

fn main() -> Result<()> {
    let v = parse()?;
    let p1 = part1(v.clone())?;
    println!("part 1: {}", p1);
    let p2 = part2(v)?;
    println!("part 2: {}", p2);
    Ok(())
}
