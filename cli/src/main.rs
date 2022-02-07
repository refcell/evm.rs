/// Main EVM

use std::env;
use std::error::Error;

use evm::vm::*;
use evm::opcodes::Opcode;

fn debug(vm: &mut Vm) {
    loop {
        match vm.next() {
            Opcode::EOF => break,
            x => x.describe(),
        }
    }
}

fn interpret(vm: &mut Vm) {
    while !vm.at_end {
        vm.interpret();
    }
    vm.print_stack();
}

fn run() -> Result<(), Box<dyn Error>> {
    // Parse function args
    let args: Vec<String> = env::args().collect();

    // Validate Arguments
    if args.len() < 3 {
        println!("Usage: {} <function> <filename>", args[0]);
        return Ok(());
    }

    let function = args[1].clone();
    let filename = args[2].clone();

    // ** Create the Vm from file
    if let Ok(mut vm) = Vm::new_from_file(&filename) {
        match &*function {
            "debug" => debug(&mut vm),
            "run" => interpret(&mut vm),
            _ => panic!("Expect either 'debug' or 'run' for first parameter")
        }
    } else {
        println!("Failed to parse file bytecode!");
    }

    Ok(())
}

fn main() {
    run().unwrap();
}
