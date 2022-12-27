mod day_1;
mod day_2;
mod day_3;
mod day_4;
mod day_5;
use std::error;


use regex::Regex;

use std::io::prelude::*;
use std::io::BufReader;
use std::io::{Error, ErrorKind};
use std::fs::File;

fn main() -> std::io::Result<()>{
    println!("Hello, world!");
    day_1::run()?;
    day_2::run(true)?;
    day_3::run(true)?;
    day_4::run(true)?;
    day_5::run()?;

    Ok(())
}



