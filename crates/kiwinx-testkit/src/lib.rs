//! Synthetic ARM64 programs shared by Kiwinx tests and demos.

/// Encodes `MOVZ X0, #immediate` without a shift.
pub const fn movz_x0(immediate: u16) -> u32 {
    0xD280_0000 | ((immediate as u32) << 5)
}

/// Encodes `ADD X0, X0, #immediate` without a shift.
///
/// Panics when the immediate does not fit the instruction's unsigned 12-bit field.
pub const fn add_x0_x0(immediate: u16) -> u32 {
    assert!(immediate <= 0x0FFF);
    0x9100_0000 | ((immediate as u32) << 10)
}

/// Encodes the synthetic `SVC #0` exit instruction.
pub const SVC_EXIT: u32 = 0xD400_0001;

/// Converts ARM64 instruction words to little-endian guest bytes.
pub fn words_to_bytes(words: &[u32]) -> Vec<u8> {
    words.iter().flat_map(|word| word.to_le_bytes()).collect()
}

/// Builds `MOVZ X0, initial; ADD X0, X0, increment; SVC #0`.
pub fn exit_after_add(initial: u16, increment: u16) -> Vec<u8> {
    words_to_bytes(&[
        movz_x0(initial),
        add_x0_x0(increment),
        SVC_EXIT,
    ])
}
