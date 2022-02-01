use std::fs::File;
use std::io::prelude::*;
use std::error::Error;

fn run() -> Result<(), Box<Error>> {
    let filename = "myfilename";
    
    let mut f = File::open(filename)?;
    let mut buffer = String::new();
    f.read_to_string(&mut buffer)?;
    
    println!("{}", buffer);
}

fn main() {
    run().unwrap();
}

