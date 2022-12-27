
use core::str;

// https://adventofcode.com/2022/day/5
// Supply Stacks.
// Given a list of inputs representing jumbled rucksacks, find unique values within substrings of those inputs, and 
// in part 2, return letters in common between groups of 3.
use super::*;
use super::Regex;


struct Cargo {
    stacks: Vec<Vec<char>>
}

impl Cargo {
    fn new(num_stacks: usize) -> Cargo{

        let mut stacks: Vec<Vec<char>> = Vec::new();
        for _ in 0..num_stacks {
            stacks.push(Vec::new());
        }
        Cargo {
            stacks
        }
    }

    fn parse_row(&mut self, row_str: &str) -> Result<(), Error> {
        let re = Regex::new(r"(\w|\s\s\s\s)");
        if let Ok(re) = re {
            for (i,regmatch) in re.find_iter(row_str).into_iter().enumerate() {
                match regmatch.as_str().chars().nth(0) {
                    Some(' ') => (), // We need to match the whitespace to ensure stacks can be 'skipped' over
                    Some(c) => self.stacks[i].push(c),
                    None => ()
                }
            }
            Ok(())
        } else {
            // Recreate Regex error as a 
            return Err(Error::new(ErrorKind::Other, "Could not parse row in correct format."))
        }
    }

    fn move_top_item_between_stacks(&mut self, from_ind: usize, to_ind: usize) {
        if self.stacks[from_ind].len() > 0 {
            let item = self.stacks[from_ind].pop().unwrap();
            self.stacks[to_ind].push(item);
        }
    }
    fn get_top_chars(&self) -> String {
        let mut s = String::new();
        for i in 0..self.stacks.len(){
            s.push(*self.stacks[i].last().unwrap());
        }
        s
    }



}

fn get_last_digit_in_string(s: &str) -> Result<u32, Error> {
    let mut i = 0;
    let mut found = false;
    for c in s.chars() {
        match c.to_digit(10) {
            Some(v) => {i = v; found = true},
            None => ()
        }
    }
    if found {
        Ok(i)
    } else {
        Err(Error::new(ErrorKind::Other, "Could not find usable digit in string."))
    }
}

pub fn run() -> Result<(), Error> {

    // I've split file input into two files for convenience

    let f = File::open("input/day5input_starting.txt")?;
    let buf = BufReader::new(f);

    let mut item_string_list: Vec<_> = buf.lines().map(|s| s.unwrap()).collect();
    let mut cargo = Cargo::new(get_last_digit_in_string(&item_string_list.pop().unwrap())? as usize);
    item_string_list.reverse();
    for line in item_string_list {
        cargo.parse_row(&line)?;
    }

    println!("Result for day 5 = {:}",cargo.get_top_chars());

    let f = File::open("input/day5input_moving.txt")?;
    let buf = BufReader::new(f);
    for line in buf.lines() {
        let line = line?;
        let re = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
        let caps = re.captures(&line).unwrap();  
        let item_move_count = caps.get(1).unwrap().as_str().parse().unwrap();
        let from_stack = caps.get(2).unwrap().as_str().parse::<usize>().unwrap() - 1;//stacks are one-indexed in .txt
        let to_stack = caps.get(3).unwrap().as_str().parse::<usize>().unwrap() - 1; //stacks are one-indexed in .txt

        for _ in 0..item_move_count{
            cargo.move_top_item_between_stacks(from_stack, to_stack);
        }
    }


    println!("Result for day 5 = {:}",cargo.get_top_chars());

    Ok(())
    
}