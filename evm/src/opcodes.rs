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


    STOP(u8),     // 0x00 - Stops Execution
    ADD(u8),      // 0x01
    MUL(u8),      // 0x02

    POP(u8),      // 0x50 - Removes an item from the stack

    // DUPLICATIONS
    DUP1(u8),     // 0x80 - Duplicates the first item on the stack
    DUP2(u8),     // 0x81 - Duplicates the second item on the stack
    DUP3(u8),     // 0x82 - Duplicates the third item on the stack
    DUP4(u8),     // 0x83 - Duplicates the fourth item on the stack
    DUP5(u8),     // 0x84 - Duplicates the fifth item on the stack
    DUP6(u8),     // 0x85 - Duplicates the sixth item on the stack
    DUP7(u8),     // 0x86 - Duplicates the seventh item on the stack
    DUP8(u8),     // 0x87 - Duplicates the eighth item on the stack
    DUP9(u8),     // 0x88 - Duplicates the ninth item on the stack
    DUP10(u8),    // 0x89 - Duplicates the tenth item on the stack
    DUP11(u8),    // 0x8a - Duplicates the eleventh item on the stack
    DUP12(u8),    // 0x8b - Duplicates the twelfth item on the stack
    DUP13(u8),    // 0x8c - Duplicates the thirteenth item on the stack
    DUP14(u8),    // 0x8d - Duplicates the fourteenth item on the stack
    DUP15(u8),    // 0x8e - Duplicates the fifteenth item on the stack
    DUP16(u8),    // 0x8f - Duplicates the sixteenth item on the stack

    // SWAPS
    SWAP1(u8),    // 0x90 - Swaps the first two items on the stack
    SWAP2(u8),    // 0x91 - Swaps the first and third items on the stack
    SWAP3(u8),    // 0x92 - Swaps the first and fourth items on the stack
    SWAP4(u8),    // 0x93 - Swaps the first and fifth items on the stack
    SWAP5(u8),    // 0x94 - Swaps the first and sixth items on the stack
    SWAP6(u8),    // 0x95 - Swaps the first and seventh items on the stack
    SWAP7(u8),    // 0x96 - Swaps the first and eighth items on the stack
    SWAP8(u8),    // 0x97 - Swaps the first and ninth items on the stack
    SWAP9(u8),    // 0x98 - Swaps the first and tenth items on the stack
    SWAP10(u8),   // 0x99 - Swaps the first and eleventh items on the stack
    SWAP11(u8),   // 0x9a - Swaps the first and twelfth items on the stack
    SWAP12(u8),   // 0x9b - Swaps the first and thirteenth items on the stack
    SWAP13(u8),   // 0x9c - Swaps the first and fourteenth items on the stack
    SWAP14(u8),   // 0x9d - Swaps the first and fifteenth items on the stack
    SWAP15(u8),   // 0x9e - Swaps the first and sixteenth items on the stack
    SWAP16(u8),   // 0x9f - Swaps the first and seventeenth items on the stack

    // LOGS
    LOG0(u8),     // 0xa0 - Appends log with no topics
    LOG1(u8),     // 0xa1 - Appends log with one topic

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
