/// Primary Vm Implementation

use std::fs::File;
use std::io::prelude::*;
use std::error::Error;
use bigint::uint::U256;

use crate::opcodes::Opcode;

use lexer::decode::*;

/// The VM
pub struct Vm {
    pub code: Vec<u8>, // smart contract code
    pub pc: usize, // current instruction
    pub stack: Vec<U256>, // stack
    pub at_end: bool, // End of the VM Stack
}

/// Implement the VM
impl Vm {
    pub fn new_from_file(filename: &str) -> Result<Vm, Box<dyn Error>> {
        let mut f = File::open(filename)?;
        let mut buffer = String::new();
        f.read_to_string(&mut buffer)?;

        let code = decode(&buffer)?;
        Ok(Vm { code: code, pc: 0, stack: Vec::new(), at_end: false})
    }

    /// Execute the current opcode
    /// And increment the program counter
    pub fn next(&mut self) -> Opcode {
        if self.pc >= self.code.len() {
            return Opcode::EOF;
        }
        let addr = self.pc.try_into().unwrap();
        match self.code[self.pc] {
            0x00 => {
              self.pc += 1;
              Opcode::STOP(addr)
            },
            0x01 => {
              self.pc += 1;
              Opcode::ADD(addr)
            },
            0x02 => {
              self.pc += 1;
              Opcode::MUL(addr)
            },
            0x50 => {
              // Pops the item off the stack
              self.pc += 1;
              Opcode::POP(addr)
            }
            0x60 => {
              // Grab the address off the stack
              let val = self.code[self.pc + 1];
              self.pc += 1;
              Opcode::PUSH1(addr, val)
            },
            0x61 => {
              // Grab the address off the stack
              let val = self.code[self.pc + 1];
              let val2 = self.code[self.pc + 2];
              self.pc += 1;
              Opcode::PUSH2(addr, val, val2)
            },
            _ => {
              self.pc += 1;
              Opcode::UNKNOWN(addr)
            }
        }
    }

    /// Interpret Opcodes
    pub fn interpret(&mut self) {
      let maybe_op = self.next();

      // Debug Opcodes
      maybe_op.describe();

      // Interpret Opcodes
      match maybe_op {
        Opcode::PUSH1(addr, value) => {
            // Convert u8 value to U256 and push to stack
            self.stack.push(U256::from(value));
        },
        Opcode::POP(_) => {
          self.stack.pop();
        }
        Opcode::ADD(addr) => {
            // How to recover nicely? There is no meaning in recovering nicely here.
            // Just burn and crash if cannot pop.
            let v1 = self.stack.pop().unwrap();
            let v2 = self.stack.pop().unwrap();
            self.stack.push(v1 + v2);
        },
        Opcode::EOF => {
          self.at_end = true;
        }
        _ => {
            // not implemented, just chill
        }
      }
    }

    /// Print the Stack
    pub fn print_stack(&self) {
      self.stack
          .iter()
          .enumerate()
          .rev()
          .for_each(|(i, x)| {
              let mut bytes = vec![0;32];
              x.to_big_endian(&mut bytes);
              println!("|{}:\t{:?}|", i, bytes)
          });
    }
}
