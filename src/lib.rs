
mod day_1;
mod day_2;
mod day_3;
mod day_4;
mod day_5;
mod day_6;
mod day_7;
mod day_8;
mod day_9;
mod day_10;

use std::error;

use regex::Regex;

use std::io::prelude::*;
use std::io::BufReader;
use std::io::{Error, ErrorKind};
use std::fs::File;

// Run all challenge files up to the current date
// 'specific_challenge' - index of specific challenge to run
pub fn run_challenges(specific_challenge: usize) -> Result<(), Box<dyn error::Error>> {
    let functions: Vec<&dyn Fn(bool) -> Result<(), Box<dyn error::Error>>> =  vec![
        &day_1::run,
        &day_2::run,
        &day_3::run,
        &day_4::run,
        &day_5::run,
        &day_6::run,
        &day_7::run,
        &day_8::run,
        &day_9::run,
        &day_10::run
    ];
    
    if specific_challenge > 0 {
        run_challenge_parts(functions[specific_challenge])?;
    } else {
        for f in functions {
            run_challenge_parts(f)?;
        }
    }
    Ok(())
}

// Runs both part_1 and part_2 of provided challenge function
// 'f' - function that accepts a boolean (for 'part_2') that corresponds to the day's challengs
fn run_challenge_parts(f : &dyn Fn(bool) -> Result<(), Box<dyn error::Error>>) -> Result<(),Box<dyn error::Error>> {
    for part in [false, true] {
        match f(part) {
            Ok(()) => (),
            Err(e) => return Err(e)
        }
    }
    Ok(())
}