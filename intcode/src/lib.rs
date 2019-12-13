use thiserror::Error;
use std::convert::TryFrom;

#[derive(Error, Debug)]
pub enum VMError {
    #[error("The opcode `{opcode}` at address {addr} is invalid")]
    InvalidOpcode {
        opcode: i32,
        addr: i32,
    },
    #[error("Invalid address `{addr}`")]
    InvalidAddress {
        addr: i32,
    }
}

type Result<T> = std::result::Result<T, VMError>;

#[derive(Clone, Copy, PartialEq, Eq)]
enum Opcode {
    Add,
    Mul,
    End,
}

pub struct Vm {
    memory: Vec<i32>,
    pc: i32,
    state: VmState,
}

#[derive(Clone, Copy, PartialEq, Eq)]
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
            state: VmState::Running,
        }
    }
    pub fn step(&mut self) -> Result<VmState> {
        let op = self.read_opcode()?;
        match op {
            Opcode::Add => {
                let arg1 = self.fetch_param()?;
                let arg1 = self.read_at(arg1)?;

                let arg2 = self.fetch_param()?;
                let arg2 = self.read_at(arg2)?;

                let arg3 = self.fetch_param()?;
                let res = arg1 + arg2;
                self.write_at(arg3, res)?;
            },
            Opcode::Mul => {
                let arg1 = self.fetch_param()?;
                let arg1 = self.read_at(arg1)?;

                let arg2 = self.fetch_param()?;
                let arg2 = self.read_at(arg2)?;

                let arg3 = self.fetch_param()?;
                let res = arg1 * arg2;
                self.write_at(arg3, res)?;
            },
            Opcode::End => {
                self.state = VmState::Stopped;
            }
        };
        Ok(self.state)
}
    pub fn run(&mut self) -> Result<i32> {
        loop {
            match self.step()? {
                VmState::Running => {},
                VmState::Stopped => {return self.read_at(0);},
            }
        }
    }
    fn read_at(&self, addr: i32) -> Result<i32> {
        let idx = usize::try_from(addr).map_err(|_| VMError::InvalidAddress{addr: addr})?;
        self.memory.get(idx).map(|i| *i).ok_or_else(|| VMError::InvalidAddress{addr: addr})
    }
    fn write_at(&mut self, addr: i32, val: i32) -> Result<()> {
        let idx = usize::try_from(addr).map_err(|_| VMError::InvalidAddress{addr: addr})?;
        let v = self.memory.get_mut(idx).ok_or_else(|| VMError::InvalidAddress{addr: addr})?;
        *v = val;
        Ok(())
    }
    fn read_opcode(&mut self) -> Result<Opcode> {
        let i = self.read_at(self.pc)?;
        self.pc += 1;
        Ok(match i {
            1  => Opcode::Add,
            2  => Opcode::Mul,
            99 => Opcode::End,
            o  => return Err(VMError::InvalidOpcode{opcode: o, addr: self.pc}),
        })
    }
    fn fetch_param(&mut self) -> Result<i32> {
        let p = self.read_at(self.pc)?;
        self.pc += 1;
        Ok(p)
    }
}
