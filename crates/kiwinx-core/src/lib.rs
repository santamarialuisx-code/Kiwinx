//! Headless Kiwinx system composition.

use std::{error::Error, fmt};

use kiwinx_common::GuestAddress;
use kiwinx_cpu::{Cpu, CpuError};
use kiwinx_memory::{Memory, MemoryError};

#[derive(Debug)]
pub struct System {
    memory: Memory,
    cpu: Cpu,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SystemError {
    Memory(MemoryError),
    Cpu(CpuError),
}

impl System {
    pub fn new(memory_size: usize) -> Self {
        Self {
            memory: Memory::new(memory_size),
            cpu: Cpu::new(GuestAddress::new(0)),
        }
    }

    pub fn load(&mut self, address: GuestAddress, bytes: &[u8]) -> Result<(), SystemError> {
        self.memory.write_bytes(address, bytes)?;
        Ok(())
    }

    pub fn run(
        &mut self,
        entry: GuestAddress,
        step_limit: usize,
    ) -> Result<u64, SystemError> {
        self.cpu = Cpu::new(entry);
        self.cpu.run(&self.memory, step_limit)?;
        let exit_code = self.cpu.x(0).expect("X0 always exists");
        Ok(exit_code)
    }
}

impl From<MemoryError> for SystemError {
    fn from(error: MemoryError) -> Self {
        Self::Memory(error)
    }
}

impl From<CpuError> for SystemError {
    fn from(error: CpuError) -> Self {
        Self::Cpu(error)
    }
}

impl fmt::Display for SystemError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Memory(error) => error.fmt(formatter),
            Self::Cpu(error) => error.fmt(formatter),
        }
    }
}

impl Error for SystemError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Memory(error) => Some(error),
            Self::Cpu(error) => Some(error),
        }
    }
}
