use thiserror::Error;
use std::convert::TryFrom;
use std::collections::VecDeque;

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
    },
    #[error("The VM is stopped")]
    Stopped,
    #[error("The VM is waiting for input, but none is available")]
    NoMoreInput,
}

type Result<T> = std::result::Result<T, VMError>;

#[derive(Clone, Copy, PartialEq, Eq)]
enum ParameterMode {
    Immediate,
    Position,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Opcode {
    Add(ParameterMode, ParameterMode),
    Mul(ParameterMode, ParameterMode),
    Input,
    Output(ParameterMode),
    End,
}

pub struct Vm {
    memory: Vec<i32>,
    pc: i32,
    state: VmState,
    inputs: VecDeque<i32>,
    outputs: VecDeque<i32>,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum VmState {
    Running,
    Stopped,
    WaitingForInput,
}

impl Vm {
    pub fn new(mut memory: Vec<i32>, arg1: i32, arg2: i32) -> Vm {
        memory[1] = arg1;
        memory[2] = arg2;
        Vm {
            memory,
            pc: 0,
            state: VmState::Running,
            inputs: VecDeque::new(),
            outputs: VecDeque::new(),
        }
    }
    pub fn step(&mut self) -> Result<VmState> {
        match self.state {
            VmState::Stopped => {
                return Err(VMError::Stopped);
            },
            VmState::WaitingForInput => {
                if self.inputs.is_empty() {
                    return Ok(VmState::WaitingForInput);
                }
                self.state = VmState::Running;
            },
            VmState::Running => {},
        }
        let op = self.read_opcode()?;
        match op {
            Opcode::Add(par1, par2) => {
                let arg1 = self.fetch_param(par1)?;
                let arg2 = self.fetch_param(par2)?;
                let arg3 = self.fetch_param(ParameterMode::Immediate)?;
                let res = arg1 + arg2;
                self.write_at(arg3, res)?;
            },
            Opcode::Mul(par1, par2) => {
                let arg1 = self.fetch_param(par1)?;
                let arg2 = self.fetch_param(par2)?;
                let arg3 = self.fetch_param(ParameterMode::Immediate)?;
                let res = arg1 * arg2;
                self.write_at(arg3, res)?;
            },
            Opcode::Input => {
                if let Some(i) = self.inputs.pop_back() {
                    let arg1 = self.fetch_param(ParameterMode::Immediate)?;
                    self.write_at(arg1, i)?;
                } else {
                    self.state = VmState::WaitingForInput;
                }
            },
            Opcode::Output(par1) => {
                let arg1 = self.fetch_param(par1)?;
                self.outputs.push_back(arg1);
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
                VmState::WaitingForInput => {return Err(VMError::NoMoreInput);},
            }
        }
    }
    pub fn get_outputs(&mut self) -> impl Iterator<Item=i32> + '_ {
        self.outputs.drain(..)
    }

    pub fn add_inputs(&mut self, inputs: &[i32]) {
        for i in inputs {
            self.inputs.push_back(*i);
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
        Ok(match i % 100 {
            1  => {
                let mode1 = if (i / 100) % 10 == 0 { ParameterMode::Position } else { ParameterMode::Immediate };
                let mode2 = if (i / 1000) % 10 == 0 { ParameterMode::Position } else { ParameterMode::Immediate };
                Opcode::Add(mode1, mode2)
            },
            2  => {
                let mode1 = if (i / 100) % 10 == 0 { ParameterMode::Position } else { ParameterMode::Immediate };
                let mode2 = if (i / 1000) % 10 == 0 { ParameterMode::Position } else { ParameterMode::Immediate };
                Opcode::Mul(mode1, mode2)
            },
            3  => Opcode::Input,
            4  => {
                let mode1 = if (i / 100) % 10 == 0 { ParameterMode::Position } else { ParameterMode::Immediate };
                Opcode::Output(mode1)
            },
            99 => Opcode::End,
            o  => return Err(VMError::InvalidOpcode{opcode: o, addr: self.pc}),
        })
    }
    fn fetch_param(&mut self, mode: ParameterMode) -> Result<i32> {
        let p = self.read_at(self.pc)?;
        self.pc += 1;
        if mode == ParameterMode::Immediate {
            Ok(p)
        } else {
            self.read_at(p)
        }
    }
}
