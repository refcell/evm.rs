//! Opcodes for the EVM.
//!
//! 2 byte (16 bits) opcodes are used for the instruction set.
//! See evm.codes for a comprehensive list of opcodes.
//! Or visit the Ethereum yellowpaper at https://ethereum.github.io/yellowpaper/paper.pdf

/// EVM Opcodes
pub enum Opcode {

    STOP, // 0x00
    ADD, // 0x01
    MUL, // 0x02

    PUSH1(u8), // 0x60
    PUSH2(u8, u8), // 0x61

    PUSH32(u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8), // 0x7f 
}
