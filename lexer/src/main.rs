use std::fs::File;
use std::io::prelude::*;
use std::error::Error;

fn run() -> Result<(), Box<dyn Error>> {
    let filename = "SampleContract.bin-runtime";
    
    let mut f = File::open(filename)?;
    let mut buffer = String::new();
    f.read_to_string(&mut buffer)?;
    
    println!("{}", buffer);

    Ok(())
}

fn main() {
    run().unwrap();
}

