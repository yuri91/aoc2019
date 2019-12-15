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
    Add(ParameterMode, ParameterMode),
    Mul(ParameterMode, ParameterMode),
    Input,
    Output(ParameterMode),
    JumpIfTrue(ParameterMode, ParameterMode),
    JumpIfFalse(ParameterMode, ParameterMode),
    LessThan(ParameterMode, ParameterMode),
    Equals(ParameterMode, ParameterMode),
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
                let res = (arg1 < arg2) as i64;
                self.write_at(arg3, res)?;
            },
            Opcode::Equals(par1, par2) => {
                let arg1 = self.fetch_param(par1)?;
                let arg2 = self.fetch_param(par2)?;
                let arg3 = self.fetch_param(ParameterMode::Immediate)?;
                let res = (arg1 == arg2) as i64;
                self.write_at(arg3, res)?;
            },
            Opcode::RelativeBaseOffset(par1) => {
                let arg1 = self.fetch_param(par1)?;
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

    pub fn read_at(&self, addr: i64) -> Result<i64> {
        let idx = usize::try_from(addr).map_err(|_| VMError::InvalidAddress{addr: addr})?;
        if let Some(i) = self.memory.get(idx) {
            debug!("[{}] reading [{}]={}", self.pc, addr, i);
            Ok(*i)
        } else {
            debug!("[{}] reading [{}]=invalid", self.pc, addr);
            Err(VMError::InvalidAddress{addr: addr})
        }
    }
    pub fn write_at(&mut self, addr: i64, val: i64) -> Result<()> {
        debug!("[{}] writing [{}]={}", self.pc, addr, val);
        let idx = usize::try_from(addr).map_err(|_| VMError::InvalidAddress{addr: addr})?;
        let v = self.memory.get_mut(idx).ok_or_else(|| VMError::InvalidAddress{addr: addr})?;
        *v = val;
        Ok(())
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
        let i = self.read_at(self.pc)?;
        self.pc += 1;
        Ok(match i % 100 {
            1  => {
                let mode1 = Self::decode_mode(i, 0).ok_or_else(|| VMError::InvalidOpcode { opcode: i, addr: self.pc-1 })?;
                let mode2 = Self::decode_mode(i, 1).ok_or_else(|| VMError::InvalidOpcode { opcode: i, addr: self.pc-1 })?;
                Opcode::Add(mode1, mode2)
            },
            2  => {
                let mode1 = Self::decode_mode(i, 0).ok_or_else(|| VMError::InvalidOpcode { opcode: i, addr: self.pc-1 })?;
                let mode2 = Self::decode_mode(i, 1).ok_or_else(|| VMError::InvalidOpcode { opcode: i, addr: self.pc-1 })?;
                Opcode::Mul(mode1, mode2)
            },
            3  => Opcode::Input,
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
                Opcode::LessThan(mode1, mode2)
            },
            8  => {
                let mode1 = Self::decode_mode(i, 0).ok_or_else(|| VMError::InvalidOpcode { opcode: i, addr: self.pc-1 })?;
                let mode2 = Self::decode_mode(i, 1).ok_or_else(|| VMError::InvalidOpcode { opcode: i, addr: self.pc-1 })?;
                Opcode::Equals(mode1, mode2)
            },
            9  => {
                let mode1 = Self::decode_mode(i, 0).ok_or_else(|| VMError::InvalidOpcode { opcode: i, addr: self.pc-1 })?;
                Opcode::RelativeBaseOffset(mode1)
            },
            99 => Opcode::End,
            o  => return Err(VMError::InvalidOpcode{opcode: o, addr: self.pc-1}),
        })
    }
    fn fetch_param(&mut self, mode: ParameterMode) -> Result<i64> {
        debug!("[{}] fetching {:?}", self.pc, mode);
        let p = self.read_at(self.pc)?;
        self.pc += 1;
        match mode {
            ParameterMode::Immediate => {
                Ok(p)
            },
            ParameterMode::Position => {
                self.read_at(p)
            },
            ParameterMode::Relative => {
                self.read_at(p).map(|a| a + self.rb)
            },
        }
    }
}
