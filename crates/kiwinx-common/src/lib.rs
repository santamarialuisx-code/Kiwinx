//! Shared types for Kiwinx subsystems.

/// An address in the emulated guest's address space.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct GuestAddress(u64);

impl GuestAddress {
    pub const fn new(value: u64) -> Self {
        Self(value)
    }

    pub const fn get(self) -> u64 {
        self.0
    }
}
