use anyhow::Result;
use anyhow::anyhow;

enum Instruction {
    Add(usize, usize, usize),
    Mul(usize, usize, usize),
    End,
    Invalid,
}
impl Instruction {
    fn fetch(mem: &[usize]) -> (Instruction, usize) {
        match mem[0] {
            1  => (Instruction::Add(mem[1], mem[2], mem[3]), 4),
            2  => (Instruction::Mul(mem[1], mem[2], mem[3]), 4),
            99 => (Instruction::End, 1),
            _ => (Instruction::Invalid, 0),
        }
    }
}

struct IntcodeVm {
    memory: Vec<usize>,
    pc: usize,
}

enum VmState {
    Running,
    Stopped,
    Trapped,
}
impl IntcodeVm {
    fn new(mut memory: Vec<usize>, arg1: usize, arg2: usize) -> IntcodeVm {
        memory[1] = arg1;
        memory[2] = arg2;
        IntcodeVm {
            memory,
            pc: 0,
        }
    }
    fn step(&mut self) -> VmState {
        let (inst, inc) = Instruction::fetch(&self.memory[self.pc..]);
        self.pc += inc;
        match inst {
            Instruction::Add(arg1, arg2, res) => {
                self.memory[res] = self.memory[arg1] + self.memory[arg2];
            },
            Instruction::Mul(arg1, arg2, res) => {
                self.memory[res] = self.memory[arg1] * self.memory[arg2];
            },
            Instruction::End => {
                return VmState::Stopped;
            },
            Instruction::Invalid => {
                return VmState::Trapped;
            },
        };
        VmState::Running
}
    fn run(&mut self) -> Result<usize> {
        loop {
            match self.step() {
                VmState::Running => {},
                VmState::Stopped => {return Ok(self.memory[0]);},
                VmState::Trapped => {return Err(anyhow!("invalid instruction"));},
            }
        }
    }
}

fn parse() -> Result<Vec<usize>> {
    std::fs::read_to_string("input")?
        .trim()
        .split(',')
        .map(|s| s.parse().map_err(std::convert::From::from))
        .collect()
}

fn part1(v: &[usize]) -> Result<impl std::fmt::Display> {
    let mut vm = IntcodeVm::new(v.to_owned(), 12, 2);
    vm.run()
}

fn part2(v: &[usize]) -> Result<impl std::fmt::Display> {
    for noun in 0..100 {
        for verb in 0..100 {
            let mut vm = IntcodeVm::new(v.to_owned(), noun, verb);
            if vm.run().expect("failed to run") == 19690720 {
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
