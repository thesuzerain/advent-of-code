mod day_1;
mod day_2;

use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;

fn main() -> std::io::Result<()>{
    println!("Hello, world!");
    day_1::run()?;
    day_2::run()?;

    Ok(())
}



