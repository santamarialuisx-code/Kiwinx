//! Bounded guest memory.

use std::{error::Error, fmt, ops::Range};

use kiwinx_common::GuestAddress;

#[derive(Clone, Debug)]
pub struct Memory {
    base: GuestAddress,
    bytes: Vec<u8>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MemoryError {
    OutOfBounds { address: GuestAddress, len: usize },
}

impl Memory {
    pub fn new(size: usize) -> Self {
        Self::with_base(GuestAddress::new(0), size)
    }

    pub fn with_base(base: GuestAddress, size: usize) -> Self {
        Self {
            base,
            bytes: vec![0; size],
        }
    }

    pub fn base(&self) -> GuestAddress {
        self.base
    }

    pub fn len(&self) -> usize {
        self.bytes.len()
    }

    pub fn is_empty(&self) -> bool {
        self.bytes.is_empty()
    }

    pub fn read_bytes(
        &self,
        address: GuestAddress,
        len: usize,
    ) -> Result<&[u8], MemoryError> {
        let range = self.range(address, len)?;
        Ok(&self.bytes[range])
    }

    pub fn write_bytes(
        &mut self,
        address: GuestAddress,
        bytes: &[u8],
    ) -> Result<(), MemoryError> {
        let range = self.range(address, bytes.len())?;
        self.bytes[range].copy_from_slice(bytes);
        Ok(())
    }

    pub fn read_u32(&self, address: GuestAddress) -> Result<u32, MemoryError> {
        let bytes: [u8; 4] = self
            .read_bytes(address, 4)?
            .try_into()
            .expect("a four-byte slice always converts to a four-byte array");
        Ok(u32::from_le_bytes(bytes))
    }

    pub fn write_u32(
        &mut self,
        address: GuestAddress,
        value: u32,
    ) -> Result<(), MemoryError> {
        self.write_bytes(address, &value.to_le_bytes())
    }

    fn range(&self, address: GuestAddress, len: usize) -> Result<Range<usize>, MemoryError> {
        let error = MemoryError::OutOfBounds { address, len };
        let offset = address
            .get()
            .checked_sub(self.base.get())
            .and_then(|value| usize::try_from(value).ok())
            .ok_or(error)?;
        let end = offset.checked_add(len).ok_or(error)?;

        if end > self.bytes.len() {
            return Err(error);
        }

        Ok(offset..end)
    }
}

impl fmt::Display for MemoryError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::OutOfBounds { address, len } => write!(
                formatter,
                "guest memory access out of bounds: address {:#x}, length {len} bytes",
                address.get()
            ),
        }
    }
}

impl Error for MemoryError {}
