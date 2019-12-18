use thiserror::Error;
use std::convert::TryFrom;
use std::collections::VecDeque;
use log::debug;

#[derive(Error, Debug)]
pub enum VMError {
    #[error("The opcode `{opcode}` at address {addr} is invalid")]
    InvalidOpcode {
        opcode: i64,
        addr: i64,
    },
    #[error("Invalid address `{addr}`")]
    InvalidAddress {
        addr: i64,
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
    Relative,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Opcode {
    Add(ParameterMode, ParameterMode, ParameterMode),
    Mul(ParameterMode, ParameterMode, ParameterMode),
    Input(ParameterMode),
    Output(ParameterMode),
    JumpIfTrue(ParameterMode, ParameterMode),
    JumpIfFalse(ParameterMode, ParameterMode),
    LessThan(ParameterMode, ParameterMode, ParameterMode),
    Equals(ParameterMode, ParameterMode, ParameterMode),
    RelativeBaseOffset(ParameterMode),
    End,
}

pub struct Vm {
    memory: Vec<i64>,
    pc: i64,
    rb: i64,
    state: VmState,
    inputs: VecDeque<i64>,
    outputs: VecDeque<i64>,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum VmState {
    Running,
    Stopped,
    WaitingForInput,
}

impl Vm {
    pub fn new(memory: Vec<i64>) -> Vm {
        Vm {
            memory,
            pc: 0,
            rb: 0,
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
            Opcode::Add(par1, par2, par3) => {
                let arg1 = *self.fetch_param(par1)?;
                let arg2 = *self.fetch_param(par2)?;
                let arg3 = self.fetch_param(par3)?;
                *arg3 = arg1 + arg2;
            },
            Opcode::Mul(par1, par2, par3) => {
                let arg1 = *self.fetch_param(par1)?;
                let arg2 = *self.fetch_param(par2)?;
                let arg3 = self.fetch_param(par3)?;
                *arg3 = arg1 * arg2;
            },
            Opcode::Input(par1) => {
                if let Some(i) = self.inputs.pop_front() {
                    let arg1 = self.fetch_param(par1)?;
                    *arg1 = i;
                } else {
                    self.pc -= 1;
                    self.state = VmState::WaitingForInput;
                }
            },
            Opcode::Output(par1) => {
                let arg1 = *self.fetch_param(par1)?;
                self.outputs.push_back(arg1);
            },
            Opcode::JumpIfTrue(par1, par2) => {
                let arg1 = *self.fetch_param(par1)?;
                let arg2 = self.fetch_param(par2)?;
                if arg1 != 0 {
                    self.pc = *arg2;
                }
            },
            Opcode::JumpIfFalse(par1, par2) => {
                let arg1 = *self.fetch_param(par1)?;
                let arg2 = self.fetch_param(par2)?;
                if arg1 == 0 {
                    self.pc = *arg2;
                }
            },
            Opcode::LessThan(par1, par2, par3) => {
                let arg1 = *self.fetch_param(par1)?;
                let arg2 = *self.fetch_param(par2)?;
                let arg3 = self.fetch_param(par3)?;
                *arg3 = (arg1 < arg2) as i64;
            },
            Opcode::Equals(par1, par2, par3) => {
                let arg1 = *self.fetch_param(par1)?;
                let arg2 = *self.fetch_param(par2)?;
                let arg3 = self.fetch_param(par3)?;
                *arg3 = (arg1 == arg2) as i64;
            },
            Opcode::RelativeBaseOffset(par1) => {
                let arg1 = *self.fetch_param(par1)?;
                self.rb += arg1;
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
    pub fn run_until_output(&mut self) -> Result<Option<i64>> {
        loop {
            if let Some(o) = self.outputs.pop_front() {
                return Ok(Some(o));
            }
            match self.step()? {
                VmState::Running => {},
                VmState::Stopped => {return Ok(None);},
                VmState::WaitingForInput => {return Err(VMError::NoMoreInput);},
            }
        }
    }
    pub fn read_at(&mut self, addr: i64) -> Result<i64> {
        let a = self.access(addr)?;
        Ok(*a)
    }
    pub fn write_at(&mut self, addr: i64, val: i64) -> Result<()> {
        let a = self.access(addr)?;
        *a = val;
        Ok(())
    }

    pub fn is_running(&self) -> bool {
        self.state != VmState::Stopped
    }

    pub fn get_outputs(&mut self) -> impl Iterator<Item=i64> + '_ {
        self.outputs.drain(..)
    }

    pub fn add_inputs(&mut self, inputs: &[i64]) {
        for i in inputs {
            self.inputs.push_back(*i);
        }
    }

    fn access(&mut self, addr: i64) -> Result<&mut i64> {
        debug!("[{}] accessing [{}]", self.pc, addr);
        let idx = usize::try_from(addr).map_err(|_| VMError::InvalidAddress{addr: addr})?;
        if self.memory.len() <= idx {
            self.memory.resize(idx+1, 0);
        }
        Ok(&mut self.memory[idx])
    }
    fn decode_mode(opcode: i64, param: u32) -> Option<ParameterMode> {
        match (opcode / (10i64.pow(param+2))) % 10 {
            0 => { Some(ParameterMode::Position) },
            1 => { Some(ParameterMode::Immediate) },
            2 => { Some(ParameterMode::Relative) },
            _ => { None },
        }
    }
    fn read_opcode(&mut self) -> Result<Opcode> {
        debug!("[{}] reading opcode",self.pc);
        let i = *self.access(self.pc)?;
        self.pc += 1;
        Ok(match i % 100 {
            1  => {
                let mode1 = Self::decode_mode(i, 0).ok_or_else(|| VMError::InvalidOpcode { opcode: i, addr: self.pc-1 })?;
                let mode2 = Self::decode_mode(i, 1).ok_or_else(|| VMError::InvalidOpcode { opcode: i, addr: self.pc-1 })?;
                let mode3 = Self::decode_mode(i, 2).ok_or_else(|| VMError::InvalidOpcode { opcode: i, addr: self.pc-1 })?;
                Opcode::Add(mode1, mode2, mode3)
            },
            2  => {
                let mode1 = Self::decode_mode(i, 0).ok_or_else(|| VMError::InvalidOpcode { opcode: i, addr: self.pc-1 })?;
                let mode2 = Self::decode_mode(i, 1).ok_or_else(|| VMError::InvalidOpcode { opcode: i, addr: self.pc-1 })?;
                let mode3 = Self::decode_mode(i, 2).ok_or_else(|| VMError::InvalidOpcode { opcode: i, addr: self.pc-1 })?;
                Opcode::Mul(mode1, mode2, mode3)
            },
            3  => {
                let mode1 = Self::decode_mode(i, 0).ok_or_else(|| VMError::InvalidOpcode { opcode: i, addr: self.pc-1 })?;
                Opcode::Input(mode1)
            },
            4  => {
                let mode1 = Self::decode_mode(i, 0).ok_or_else(|| VMError::InvalidOpcode { opcode: i, addr: self.pc-1 })?;
                Opcode::Output(mode1)
            },
            5  => {
                let mode1 = Self::decode_mode(i, 0).ok_or_else(|| VMError::InvalidOpcode { opcode: i, addr: self.pc-1 })?;
                let mode2 = Self::decode_mode(i, 1).ok_or_else(|| VMError::InvalidOpcode { opcode: i, addr: self.pc-1 })?;
                Opcode::JumpIfTrue(mode1, mode2)
            },
            6  => {
                let mode1 = Self::decode_mode(i, 0).ok_or_else(|| VMError::InvalidOpcode { opcode: i, addr: self.pc-1 })?;
                let mode2 = Self::decode_mode(i, 1).ok_or_else(|| VMError::InvalidOpcode { opcode: i, addr: self.pc-1 })?;
                Opcode::JumpIfFalse(mode1, mode2)
            },
            7  => {
                let mode1 = Self::decode_mode(i, 0).ok_or_else(|| VMError::InvalidOpcode { opcode: i, addr: self.pc-1 })?;
                let mode2 = Self::decode_mode(i, 1).ok_or_else(|| VMError::InvalidOpcode { opcode: i, addr: self.pc-1 })?;
                let mode3 = Self::decode_mode(i, 2).ok_or_else(|| VMError::InvalidOpcode { opcode: i, addr: self.pc-1 })?;
                Opcode::LessThan(mode1, mode2, mode3)
            },
            8  => {
                let mode1 = Self::decode_mode(i, 0).ok_or_else(|| VMError::InvalidOpcode { opcode: i, addr: self.pc-1 })?;
                let mode2 = Self::decode_mode(i, 1).ok_or_else(|| VMError::InvalidOpcode { opcode: i, addr: self.pc-1 })?;
                let mode3 = Self::decode_mode(i, 2).ok_or_else(|| VMError::InvalidOpcode { opcode: i, addr: self.pc-1 })?;
                Opcode::Equals(mode1, mode2, mode3)
            },
            9  => {
                let mode1 = Self::decode_mode(i, 0).ok_or_else(|| VMError::InvalidOpcode { opcode: i, addr: self.pc-1 })?;
                Opcode::RelativeBaseOffset(mode1)
            },
            99 => Opcode::End,
            o  => return Err(VMError::InvalidOpcode{opcode: o, addr: self.pc-1}),
        })
    }
    fn fetch_param(&mut self, mode: ParameterMode) -> Result<&mut i64> {
        debug!("[{}] fetching {:?}", self.pc, mode);
        let pc = self.pc;
        self.pc += 1;
        let r = match mode {
            ParameterMode::Immediate => {
                self.access(pc)?
            },
            ParameterMode::Position => {
                let pos = *self.access(pc)?;
                self.access(pos)?
            },
            ParameterMode::Relative => {
                let pos = *self.access(pc)?;
                self.access(pos + self.rb)?
            },
        };
        Ok(r)
    }
}
