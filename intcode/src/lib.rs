use thiserror::Error;

#[derive(Error, Debug)]
pub enum VMError {
    #[error("The opcode `{opcode}` at offset {offset} is invalid")]
    InvalidOpcode {
        opcode: i32,
        offset: usize,
    }
}

type Result<T> = std::result::Result<T, VMError>;


enum Instruction {
    Add(i32, i32, i32),
    Mul(i32, i32, i32),
    End,
    Invalid(i32),
}
impl Instruction {
    fn fetch(mem: &[i32]) -> (Instruction, usize) {
        match mem[0] {
            1  => (Instruction::Add(mem[1], mem[2], mem[3]), 4),
            2  => (Instruction::Mul(mem[1], mem[2], mem[3]), 4),
            99 => (Instruction::End, 1),
            i  => (Instruction::Invalid(i), 0),
        }
    }
}

pub struct Vm {
    memory: Vec<i32>,
    pc: usize,
}

pub enum VmState {
    Running,
    Stopped,
}
impl Vm {
    pub fn new(mut memory: Vec<i32>, arg1: i32, arg2: i32) -> Vm {
        memory[1] = arg1;
        memory[2] = arg2;
        Vm {
            memory,
            pc: 0,
        }
    }
    pub fn step(&mut self) -> Result<VmState> {
        let (inst, inc) = Instruction::fetch(&self.memory[self.pc..]);
        match inst {
            Instruction::Add(arg1, arg2, res) => {
                self.memory[res as usize] = self.memory[arg1 as usize] + self.memory[arg2 as usize];
            },
            Instruction::Mul(arg1, arg2, res) => {
                self.memory[res as usize] = self.memory[arg1 as usize] * self.memory[arg2 as usize];
            },
            Instruction::End => {
                return Ok(VmState::Stopped);
            },
            Instruction::Invalid(o) => {
                return Err(VMError::InvalidOpcode {
                    opcode: o,
                    offset: self.pc,
                });
            },
        };
        self.pc += inc;
        Ok(VmState::Running)
}
    pub fn run(&mut self) -> Result<i32> {
        loop {
            match self.step()? {
                VmState::Running => {},
                VmState::Stopped => {return Ok(self.memory[0]);},
            }
        }
    }
}
