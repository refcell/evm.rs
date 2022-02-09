//! Opcodes for the EVM.
//!
//! 2 byte (16 bits) opcodes are used for the instruction set.
//! See evm.codes for a comprehensive list of opcodes.
//! Or visit the Ethereum yellowpaper at https://ethereum.github.io/yellowpaper/paper.pdf

/// EVM Opcodes
#[derive(Debug)]
pub enum Opcode {

    // ARITHMETIC OPERATIONS
    STOP(u8),     // 0x00 - Stops Execution
    ADD(u8),      // 0x01
    MUL(u8),      // 0x02
    SUB(u8),      // 0x03
    DIV(u8),      // 0x04
    SDIV(u8),     // 0x05
    MOD(u8),      // 0x06
    SMOD(u8),     // 0x07
    ADDMOD(u8),   // 0x08
    MULMOD(u8),   // 0x09
    EXP(u8),      // 0x0a
    SIGNEXTEND(u8),// 0x0b
    LT(u8),       // 0x10
    GT(u8),       // 0x11
    SLT(u8),      // 0x12
    SGT(u8),      // 0x13
    EQ(u8),       // 0x14
    ISZERO(u8),   // 0x15
    AND(u8),      // 0x16
    OR(u8),       // 0x17
    XOR(u8),      // 0x18
    NOT(u8),      // 0x19
    BYTE(u8),     // 0x1a
    SHL(u8),      // 0x1b
    SHR(u8),      // 0x1c
    SAR(u8),      // 0x1d
    SHA3(u8),     // 0x20

    // CONTEXT
    ADDRESS(u8),  // 0x30
    BALANCE(u8),  // 0x31
    ORIGIN(u8),   // 0x32
    CALLER(u8),   // 0x33
    CALLVALUE(u8),// 0x34
    CALLDATALOAD(u8),// 0x35
    CALLDATASIZE(u8),// 0x36
    CALLDATACOPY(u8),// 0x37
    CODESIZE(u8), // 0x38
    CODECOPY(u8), // 0x39
    GASPRICE(u8), // 0x3a
    EXTCODESIZE(u8),// 0x3b
    EXTCODECOPY(u8),// 0x3c
    RETURNDATASIZE(u8),// 0x3d
    RETURNDATACOPY(u8),// 0x3e
    EXTCODEHASH(u8), // 0x3f
    BLOCKHASH(u8),// 0x40
    COINBASE(u8), // 0x41
    TIMESTAMP(u8),// 0x42
    NUMBER(u8),   // 0x43
    DIFFICULTY(u8),// 0x44
    GASLIMIT(u8), // 0x45
    CHAINID(u8),  // 0x46
    SELFBALANCE(u8),// 0x47
    BASEFEE(u8),  // 0x48

    // STACK OPERATIONS
    POP(u8),      // 0x50 - Removes an item from the stack
    MLOAD(u8),    // 0x51
    MSTORE(u8),   // 0x52
    MSTORE8(u8),  // 0x53
    SLOAD(u8),    // 0x54
    SSTORE(u8),   // 0x55
    JUMP(u8),     // 0x56
    JUMPI(u8),    // 0x57
    PC(u8),       // 0x58
    MSIZE(u8),    // 0x59
    GAS(u8),      // 0x5a
    JUMPDEST(u8), // 0x5b

    // PUSH OPERATIONS
    PUSH1(u8, u8), // 0x60
    PUSH2(u8, u8, u8), // 0x61
    PUSH3(u8),    // 0x62
    PUSH4(u8),    // 0x63
    PUSH5(u8),    // 0x64
    PUSH6(u8),    // 0x65
    PUSH7(u8),    // 0x66
    PUSH8(u8),    // 0x67
    PUSH9(u8),    // 0x68
    PUSH10(u8),   // 0x69
    PUSH11(u8),   // 0x6a
    PUSH12(u8),   // 0x6b
    PUSH13(u8),   // 0x6c
    PUSH14(u8),   // 0x6d
    PUSH15(u8),   // 0x6e
    PUSH16(u8),   // 0x6f
    PUSH17(u8),   // 0x70
    PUSH18(u8),   // 0x71
    PUSH19(u8),   // 0x72
    PUSH20(u8),   // 0x73
    PUSH21(u8),   // 0x74
    PUSH22(u8),   // 0x75
    PUSH23(u8),   // 0x76
    PUSH24(u8),   // 0x77
    PUSH25(u8),   // 0x78
    PUSH26(u8),   // 0x79
    PUSH27(u8),   // 0x7a
    PUSH28(u8),   // 0x7b
    PUSH29(u8),   // 0x7c
    PUSH30(u8),   // 0x7d
    PUSH31(u8),   // 0x7e
    PUSH32(u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8), // 0x7f


    // DUPLICATION OPERATIONS
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
    LOG2(u8),     // 0xa2 - Appends log with two topics
    LOG3(u8),     // 0xa3 - Appends log with three topics
    LOG4(u8),     // 0xa4 - Appends log with four topics

    // CORE OPERATIONS
    CREATE(u8),   // 0xf0
    CALL(u8),     // 0xf1
    CALLCODE(u8), // 0xf2
    RETURN(u8),   // 0xf3
    DELEGATECALL(u8), // 0xf4
    CREATE2(u8), // 0xf5
    STATICCALL(u8),  // 0xfa
    REVERT(u8),  // 0xfd
    INVALID(u8), // 0xfe
    SELFDESTRUCT(u8), // 0xff

    // TODO: Remove these non-existent opcodes
    EOF, // The unspoken opcode
    UNKNOWN(u8),  // Unknown Opcode
}

/// Describe the Opcode
impl Opcode {
  pub fn describe(&self) {
    match self {
        Opcode::STOP(line) => println!("0x{:x}\tSTOP\tHalts execution", line),
        Opcode::ADD(line) => println!("0x{:x}\tADD\tAddition operation", line),
        Opcode::MUL(line) => println!("0x{:x}\tMUL\tMultiplication operation", line),
        Opcode::POP(line) => println!("0x{:x}\tPOP\tRemoves an item from the stack", line),
        Opcode::DUP1(line) => println!("0x{:x}\tDUP1\tDuplicates the first item on the stack", line),
        Opcode::SWAP1(line) => println!("0x{:x}\tSWAP1\tSwaps the first two items on the stack", line),
        Opcode::SSTORE(line) => println!("0x{:x}\tSSTORE\tSaves a word to storage", line),
        Opcode::PUSH1(line, x) => println!("0x{:x}\tPUSH1\tPlace 1-byte item on the stack 0x{:x}", line, x),
        Opcode::PUSH2(line, x0, x1) => println!("0x{:x}\tPUSH2\tPlace 2-bytes item on the stack 0x{:x} 0x{:x}", line, x0, x1),
        Opcode::UNKNOWN(line) => println!("0x{:x}\tUNKNOWN\tUnknown Opcode", line),
        _ => println!("Bad Opcode Parse")
    }
  }
}
