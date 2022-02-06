/// Main EVM

use std::error::Error;

use evm::vm::*;
use evm::opcodes::Opcode;

fn run() -> Result<(), Box<dyn Error>> {
    let filename = "SampleContract.bin-runtime";

    // ** Create the Vm from file
    if let Ok(mut vm) = Vm::new_from_file(filename) {
        loop {
            match vm.next() {
                Some(Opcode::EOF) => break,
                Some(x) => x.describe(),
                None => {}
            }
        }
    } else {
        println!("Failed to create Vm from file!");
    }

    Ok(())
}

fn main() {
    run().unwrap();
}
