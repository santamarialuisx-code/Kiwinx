//! ARM64 CPU state and interpretation.

use std::{error::Error, fmt};

use kiwinx_common::GuestAddress;
use kiwinx_memory::{Memory, MemoryError};

const INSTRUCTION_SIZE: u64 = 4;

#[derive(Clone, Debug)]
pub struct Cpu {
    x: [u64; 31],
    pc: GuestAddress,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CpuError {
    Memory(MemoryError),
    UnsupportedInstruction {
        pc: GuestAddress,
        opcode: u32,
    },
    StepLimit {
        steps: usize,
    },
}

impl Cpu {
    pub fn new(entry: GuestAddress) -> Self {
        Self {
            x: [0; 31],
            pc: entry,
        }
    }

    pub fn pc(&self) -> GuestAddress {
        self.pc
    }

    pub fn x(&self, register: usize) -> Option<u64> {
        self.x.get(register).copied()
    }

    pub fn run(&mut self, memory: &Memory, step_limit: usize) -> Result<u64, CpuError> {
        for _ in 0..step_limit {
            let pc = self.pc;
            let opcode = memory.read_u32(pc)?;

            if opcode & 0xFF80_0000 == 0xD280_0000 {
                self.execute_movz(pc, opcode)?;
            } else if opcode & 0xFF00_0000 == 0x9100_0000 {
                self.execute_add_immediate(pc, opcode)?;
            } else if opcode & 0xFFE0_001F == 0xD400_0001 {
                let immediate = (opcode >> 5) & 0xFFFF;
                if immediate != 0 {
                    return Err(CpuError::UnsupportedInstruction { pc, opcode });
                }
                return Ok(self.x[0]);
            } else {
                return Err(CpuError::UnsupportedInstruction { pc, opcode });
            }
        }

        Err(CpuError::StepLimit { steps: step_limit })
    }

    fn execute_movz(&mut self, pc: GuestAddress, opcode: u32) -> Result<(), CpuError> {
        let register = (opcode & 0x1F) as usize;
        let shift = (opcode >> 21) & 0b11;

        if register == 31 || shift != 0 {
            return Err(CpuError::UnsupportedInstruction { pc, opcode });
        }

        let immediate = ((opcode >> 5) & 0xFFFF) as u64;
        self.x[register] = immediate;
        self.advance(pc);
        Ok(())
    }

    fn execute_add_immediate(
        &mut self,
        pc: GuestAddress,
        opcode: u32,
    ) -> Result<(), CpuError> {
        let destination = (opcode & 0x1F) as usize;
        let source = ((opcode >> 5) & 0x1F) as usize;

        if destination == 31 || source == 31 {
            return Err(CpuError::UnsupportedInstruction { pc, opcode });
        }

        let shift = (opcode >> 22) & 1;
        let immediate = ((opcode >> 10) & 0x0FFF) << (shift * 12);
        self.x[destination] = self.x[source].wrapping_add(immediate as u64);
        self.advance(pc);
        Ok(())
    }

    fn advance(&mut self, pc: GuestAddress) {
        self.pc = GuestAddress::new(pc.get() + INSTRUCTION_SIZE);
    }
}

impl From<MemoryError> for CpuError {
    fn from(error: MemoryError) -> Self {
        Self::Memory(error)
    }
}

impl fmt::Display for CpuError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Memory(error) => error.fmt(formatter),
            Self::UnsupportedInstruction { pc, opcode } => write!(
                formatter,
                "unsupported ARM64 instruction {opcode:#010x} at guest PC {:#x}",
                pc.get()
            ),
            Self::StepLimit { steps } => {
                write!(formatter, "guest execution reached the {steps}-instruction limit")
            }
        }
    }
}

impl Error for CpuError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Memory(error) => Some(error),
            Self::UnsupportedInstruction { .. } | Self::StepLimit { .. } => None,
        }
    }
}
