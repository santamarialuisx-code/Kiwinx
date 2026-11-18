use kiwinx_common::GuestAddress;
use kiwinx_memory::{Memory, MemoryError};

#[test]
fn reads_and_writes_little_endian_u32() {
    let mut memory = Memory::new(8);

    memory
        .write_u32(GuestAddress::new(2), 0xA1B2_C3D4)
        .unwrap();

    assert_eq!(
        memory.read_bytes(GuestAddress::new(2), 4).unwrap(),
        &[0xD4, 0xC3, 0xB2, 0xA1]
    );
    assert_eq!(
        memory.read_u32(GuestAddress::new(2)).unwrap(),
        0xA1B2_C3D4
    );
}

#[test]
fn rejected_write_leaves_memory_unchanged() {
    let mut memory = Memory::new(8);

    let error = memory
        .write_u32(GuestAddress::new(6), 0xDEAD_BEEF)
        .unwrap_err();

    assert_eq!(
        error,
        MemoryError::OutOfBounds {
            address: GuestAddress::new(6),
            len: 4,
        }
    );
    assert_eq!(
        memory.read_bytes(GuestAddress::new(0), 8).unwrap(),
        &[0; 8]
    );
}
