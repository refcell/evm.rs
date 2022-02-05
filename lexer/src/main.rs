use std::fs::File;
use std::io::prelude::*;
use std::error::Error;

mod decode;
use decode::decode;

fn run() -> Result<(), Box<dyn Error>> {
    let filename = "SampleContract.bin-runtime";

    let mut f = File::open(filename)?;
    let mut buffer = String::new();
    f.read_to_string(&mut buffer)?;

    // ** Decode the input string as bytes ** //
    let bytes = decode(&buffer)?;

    // ** Print the bytes ** //
    for b in &bytes {
        println!("0x{:x}", b);
    }
    println!("{}", buffer);

    Ok(())
}

fn main() {
    run().unwrap();
}

