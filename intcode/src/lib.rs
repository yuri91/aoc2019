use thiserror::Error;
use std::convert::TryFrom;
use std::collections::VecDeque;
use log::debug;

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

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum ParameterMode {
    Immediate,
    Position,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Opcode {
    Add(ParameterMode, ParameterMode),
    Mul(ParameterMode, ParameterMode),
    Input,
    Output(ParameterMode),
    JumpIfTrue(ParameterMode, ParameterMode),
    JumpIfFalse(ParameterMode, ParameterMode),
    LessThan(ParameterMode, ParameterMode),
    Equals(ParameterMode, ParameterMode),
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
    pub fn new(memory: Vec<i32>) -> Vm {
        Vm {
            memory,
            pc: 0,
            state: VmState::Running,
            inputs: VecDeque::new(),
            outputs: VecDeque::new(),
        }
    }
    pub fn step(&mut self) -> Result<VmState> {
        debug!("[{}] stepping", self.pc);
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
        debug!("[{}] executing {:?}", self.pc, op);
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
                if let Some(i) = self.inputs.pop_front() {
                    let arg1 = self.fetch_param(ParameterMode::Immediate)?;
                    self.write_at(arg1, i)?;
                } else {
                    self.pc -= 1;
                    self.state = VmState::WaitingForInput;
                }
            },
            Opcode::Output(par1) => {
                let arg1 = self.fetch_param(par1)?;
                self.outputs.push_back(arg1);
            },
            Opcode::JumpIfTrue(par1, par2) => {
                let arg1 = self.fetch_param(par1)?;
                let arg2 = self.fetch_param(par2)?;
                if arg1 != 0 {
                    self.pc = arg2;
                }
            },
            Opcode::JumpIfFalse(par1, par2) => {
                let arg1 = self.fetch_param(par1)?;
                let arg2 = self.fetch_param(par2)?;
                if arg1 == 0 {
                    self.pc = arg2;
                }
            },
            Opcode::LessThan(par1, par2) => {
                let arg1 = self.fetch_param(par1)?;
                let arg2 = self.fetch_param(par2)?;
                let arg3 = self.fetch_param(ParameterMode::Immediate)?;
                let res = (arg1 < arg2) as i32;
                self.write_at(arg3, res)?;
            },
            Opcode::Equals(par1, par2) => {
                let arg1 = self.fetch_param(par1)?;
                let arg2 = self.fetch_param(par2)?;
                let arg3 = self.fetch_param(ParameterMode::Immediate)?;
                let res = (arg1 == arg2) as i32;
                self.write_at(arg3, res)?;
            },
            Opcode::End => {
                self.state = VmState::Stopped;
            }
        };
        Ok(self.state)
}
    pub fn run(&mut self) -> Result<()> {
        loop {
            match self.step()? {
                VmState::Running => {},
                VmState::Stopped => {return Ok(());},
                VmState::WaitingForInput => {return Err(VMError::NoMoreInput);},
            }
        }
    }

    pub fn is_running(&self) -> bool {
        self.state != VmState::Stopped
    }

    pub fn get_outputs(&mut self) -> impl Iterator<Item=i32> + '_ {
        self.outputs.drain(..)
    }

    pub fn add_inputs(&mut self, inputs: &[i32]) {
        for i in inputs {
            self.inputs.push_back(*i);
        }
    }

    pub fn read_at(&self, addr: i32) -> Result<i32> {
        let idx = usize::try_from(addr).map_err(|_| VMError::InvalidAddress{addr: addr})?;
        if let Some(i) = self.memory.get(idx) {
            debug!("[{}] reading [{}]={}", self.pc, addr, i);
            Ok(*i)
        } else {
            debug!("[{}] reading [{}]=invalid", self.pc, addr);
            Err(VMError::InvalidAddress{addr: addr})
        }
    }
    pub fn write_at(&mut self, addr: i32, val: i32) -> Result<()> {
        debug!("[{}] writing [{}]={}", self.pc, addr, val);
        let idx = usize::try_from(addr).map_err(|_| VMError::InvalidAddress{addr: addr})?;
        let v = self.memory.get_mut(idx).ok_or_else(|| VMError::InvalidAddress{addr: addr})?;
        *v = val;
        Ok(())
    }
    fn read_opcode(&mut self) -> Result<Opcode> {
        debug!("[{}] reading opcode",self.pc);
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
            5  => {
                let mode1 = if (i / 100) % 10 == 0 { ParameterMode::Position } else { ParameterMode::Immediate };
                let mode2 = if (i / 1000) % 10 == 0 { ParameterMode::Position } else { ParameterMode::Immediate };
                Opcode::JumpIfTrue(mode1, mode2)
            },
            6  => {
                let mode1 = if (i / 100) % 10 == 0 { ParameterMode::Position } else { ParameterMode::Immediate };
                let mode2 = if (i / 1000) % 10 == 0 { ParameterMode::Position } else { ParameterMode::Immediate };
                Opcode::JumpIfFalse(mode1, mode2)
            },
            7  => {
                let mode1 = if (i / 100) % 10 == 0 { ParameterMode::Position } else { ParameterMode::Immediate };
                let mode2 = if (i / 1000) % 10 == 0 { ParameterMode::Position } else { ParameterMode::Immediate };
                Opcode::LessThan(mode1, mode2)
            },
            8  => {
                let mode1 = if (i / 100) % 10 == 0 { ParameterMode::Position } else { ParameterMode::Immediate };
                let mode2 = if (i / 1000) % 10 == 0 { ParameterMode::Position } else { ParameterMode::Immediate };
                Opcode::Equals(mode1, mode2)
            },
            99 => Opcode::End,
            o  => return Err(VMError::InvalidOpcode{opcode: o, addr: self.pc-1}),
        })
    }
    fn fetch_param(&mut self, mode: ParameterMode) -> Result<i32> {
        debug!("[{}] fetching {:?}", self.pc, mode);
        let p = self.read_at(self.pc)?;
        self.pc += 1;
        if mode == ParameterMode::Immediate {
            Ok(p)
        } else {
            self.read_at(p)
        }
    }
}
