//! Primary Vm Implementation

use crate::opcodes::Opcode;

//! The VM
struct Vm {
    code: Vec<u8>, // smart contract code
    pc: usize, // current instruction
}

//! Implement the VM
impl Vm {
    pub fn new_from_file(filename: &str) -> Result<Vm, Box<Error>> {
        let mut f = File::open(filename)?;
        let mut buffer = String::new();
        f.read_to_string(&mut buffer)?;

        let code = decode(&buffer)?;
        Ok(Vm { code: code, pc: 0})
    }

    /// Execute the current opcode
    /// And increment the program counter
    pub fn next(&mut self) -> Option<Opcode> {
        match self.code[self.pc] {
            0x00 => {
              self.pc += 1;
              Some(Opcode::STOP)
            },
            0x01 => {
              self.pc += 1;
              Some(Opcode::ADD)
            },
            0x02 => {
              self.pc += 1;
              Some(Opcode::MUL)
            },
            0x60 => {
              // Grab the address off the stack
              let val = self.code[self.pc + 1];
              self.pc += 1;
              Some(Opcode::PUSH1(val))
            },
            0x61 => {
              // Grab the address off the stack
              let val = self.code[self.pc + 1];
              let val2 = self.code[self.pc + 2];
              self.pc += 1;
              Some(Opcode::PUSH2(val, val2))
            },
            _ => {
              self.pc += 1;
              None
            }
        }
    }
}
