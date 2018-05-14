extern crate failure;

use failure::Error;
use std::io::{self, BufRead, BufReader};

fn main() {
    if let Err(err) = run() {
        eprintln!("{:?}", err);
    }
}

fn run() -> Result<(), Error> {
    let reader = BufReader::new(io::stdin());

    let mut total = 0f32;
    for line in reader.lines() {
        let digit = line?.parse::<f32>()?;
        total += digit;
    }

    println!("{}", total);
    Ok(())
}
