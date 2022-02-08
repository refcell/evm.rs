//! Opcodes for the EVM.
//!
//! 2 byte (16 bits) opcodes are used for the instruction set.
//! See evm.codes for a comprehensive list of opcodes.
//! Or visit the Ethereum yellowpaper at https://ethereum.github.io/yellowpaper/paper.pdf

/// EVM Opcodes
#[derive(Debug)]
pub enum Opcode {

    EOF, // The unspoken opcode

    UNKNOWN(u8),  // Unknown Opcode

    POP(u8),      // 0x50 - Removes an item from the stack

    STOP(u8),     // 0x00 - Stops Execution
    ADD(u8),      // 0x01
    MUL(u8),      // 0x02

    PUSH1(u8, u8), // 0x60
    PUSH2(u8, u8, u8), // 0x61
    PUSH32(u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8), // 0x7f 
}

/// Describe the Opcode
impl Opcode {
  pub fn describe(&self) {
    match self {
        Opcode::STOP(line) => println!("0x{:x}\tSTOP\tHalts execution", line),
        Opcode::ADD(line) => println!("0x{:x}\tADD\tAddition operation", line),
        Opcode::MUL(line) => println!("0x{:x}\tMUL\tMultiplication operation", line),
        Opcode::POP(line) => println!("0x{:x}\tPOP\tRemoves an item from the stack", line),
        Opcode::PUSH1(line, x) => println!("0x{:x}\tPUSH1\tPlace 1-byte item on the stack 0x{:x}", line, x),
        Opcode::PUSH2(line, x0, x1) => println!("0x{:x}\tPUSH2\tPlace 2-bytes item on the stack 0x{:x} 0x{:x}", line, x0, x1),
        Opcode::UNKNOWN(line) => println!("0x{:x}\tUNKNOWN\tUnknown Opcode", line),
        _ => println!("Bad Opcode Parse")
    }
  }
}
