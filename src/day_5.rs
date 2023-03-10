
// https://adventofcode.com/2022/day/5
// Day 5: Supply Stacks
// Given a an initial starting state of items in stacks, and an input describing how items should be moved amongst them,
// return a String consisting of the 'topmost' item (character) in every stack.
// Part 1: items are moved one at a time. 
// Part 2, they may move as groups.

use core::str;
use std::fmt;
use super::*;
use super::Regex;
use lazy_static::lazy_static;

// Cargo with a number of LIFO stacks that can have objects amongst the stack tops.
// Items are repesented by chars
struct Cargo {
    stacks: Vec<Vec<char>> // set of many different stacks of items
}

// Run challenge.
// Main entry point to day 5 challenge.
pub fn run(part_2: bool) -> Result<(), Box<dyn error::Error>> {

    // File input has been split into two files for convenience
    let f = File::open("input/day5input_starting.txt")?;
    let buf = BufReader::new(f);

    // Initialize new Cargo instance with given size
    let mut item_string_list: Vec<_> = buf.lines().map(|s| s.unwrap()).collect();
    let mut cargo = Cargo::new(get_last_digit_in_string(&item_string_list.pop().unwrap())? as usize);

    // Load items into Cargo stacks from the bottom up
    item_string_list.reverse();
    for line in item_string_list {
        cargo.parse_row(&line)?;
    }
    
    // Load items into Cargo stacks from the bottom up
    let f = File::open("input/day5input_moving.txt")?;
    let buf = BufReader::new(f);
    for line in buf.lines() {

        // Use regex to match text to expetected challenge text
        let line = line?;
        cargo.parse_command(&line, part_2);        
    }
    let part = if part_2 {2} else {1};
    println!("Result for day 5-{part} = {}",cargo.get_top_chars());
    Ok(())
    
}

impl Cargo {
    // Creates new Cargo with 'num_stacks' many stacks
    fn new(num_stacks: usize) -> Cargo{
        let mut stacks: Vec<Vec<char>> = Vec::new();
        for _ in 0..num_stacks {
            stacks.push(Vec::new());
        }
        Cargo {
            stacks
        }
    }

    // Parses row of items into items on the top of stacks.
    // Line must follow space-separated format (i is item, which is either a word char or a space):
    // 'i i i i i i'
    // Each item is added to the top of its corresponding stack in order, starting from the first stack. 
    // If a space is found for an item, that stack is skipped.
    // eg: 'i i   i i'
    fn parse_row(&mut self, row_str: &str) -> Result<(), Error> {
        lazy_static! {
            static ref REGEX_CAPTURE_STACKS: Regex = Regex::new(r"(\w|\s\s\s\s)").unwrap();
        }

        // Search for word chars or several spaces in a row (representing a skipped stack)
        for (i,regmatch) in REGEX_CAPTURE_STACKS.find_iter(row_str).into_iter().enumerate() {
            match regmatch.as_str().chars().nth(0) {
                Some(' ') => (), // match the whitespace to ensure stacks can be 'skipped' over
                Some(c) => self.stacks[i].push(c),
                None => ()
            }
        }
        Ok(())
    }

    // Parses movement command and applies it to Cargo instance
    // Command 'command_str must follow format  "move a from b to c" where 'a' is the number of items, and items are moved from stack 'b' to 'c'.
    // In part 1, groups of items are moved one at a time.
    // In part 2, groups of items are moved together.
    fn parse_command(&mut self, command_str: &str, part_2: bool) {
        // Parse movement command from input file into correct 
        lazy_static! {
            static ref REGEX_MOVE_FROM: Regex = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
        }
        let caps = REGEX_MOVE_FROM.captures(&command_str).unwrap();  
        let item_move_count = caps.get(1).unwrap().as_str().parse().unwrap();
        let from_stack = caps.get(2).unwrap().as_str().parse::<usize>().unwrap() - 1;//stacks are one-indexed in .txt
        let to_stack = caps.get(3).unwrap().as_str().parse::<usize>().unwrap() - 1; //stacks are one-indexed in .txt

        if part_2 {
            // Part 2 - move n-sized groups of objects, keeping the same order.
            self.move_top_n_items_between_stacks(from_stack, to_stack, item_move_count).unwrap();
        } else {
            // Part 1 - move n-sized number of objects, one at a time
            for _ in 0..item_move_count{
                self.move_top_item_between_stacks(from_stack, to_stack);
            }
        }
    }

    // Moves last item in stack 'from_ind' to the top of stack 'to_ind'
    // Panics if from_ind or to_ind exceeds the number of stacks
    fn move_top_item_between_stacks(&mut self, from_ind: usize, to_ind: usize) {
        if self.stacks[from_ind].len() > 0 {
            let item = self.stacks[from_ind].pop().unwrap();
            self.stacks[to_ind].push(item);
        }
    }
    

    // Moves top 'num_items' in stack 'from_ind' to the top of stack 'to_ind'
    // Panics if from_ind or to_ind exceeds the number of stacks
    fn move_top_n_items_between_stacks(&mut self, from_ind: usize, to_ind: usize, num_items: usize) -> Result<(), StackTooSmallError> {        
        if self.stacks[from_ind].len() >= num_items {

            // Create iter_mut of Vec<Vec<>> and use nth to leverage unsafe code in iter_mut to 
            // get two mutable references to different elemenets of self.stacks
            let mut stacks_iter = self.stacks.iter_mut();
            let from_stack;
            let to_stack;

            // nth consumes mut_iter up to nth element, so extract elements in correct order
            // second nth is reduced by index of first and 1 more (as nth is removed)
            if to_ind > from_ind {
                from_stack = stacks_iter.nth(from_ind).unwrap();
                to_stack = stacks_iter.nth(to_ind - from_ind - 1).unwrap();
            } else if from_ind > to_ind {
                to_stack = stacks_iter.nth(to_ind).unwrap();
                from_stack = stacks_iter.nth(from_ind - to_ind - 1).unwrap();    
            } else {
               return Ok(()); // Do nothing if we are taking and moving to the same stack
            }
            
            // Extract items to move, unwrapping is fine as we've already checked length
            let from_length = from_stack.len();
            let items = from_stack.get(from_length-num_items..).unwrap();

            // Put them into 'to' stack
            to_stack.extend_from_slice(items);            

            // Remove items from 'from' stack
            from_stack.truncate(from_length - num_items);

            Ok(())
        } else {
            Err(StackTooSmallError)
        }
    }

    // Gets the top character of each stack (as a String)
    fn get_top_chars(& self) -> String {
        let mut s = String::new();
        for stack in self.stacks.iter() {
            s.push(*stack.last().unwrap());
        }
        s
    }
}

// Gets last instance of a digit in a string of characters
// eg: 'fineoin423inod43dfe' -> 3
fn get_last_digit_in_string(s: &str) -> Result<u32, Error> {
    let mut i = 0;
    let mut found = false;
    for c in s.chars() {
        match c.to_digit(10) {
            Some(v) => {i = v; found = true},
            None => ()
        }
    }
    if !found {
       return Err(Error::new(ErrorKind::Other, "Could not find usable digit in string."))
    }
    Ok(i)
}

#[derive(Debug, Clone)]
struct StackTooSmallError;
impl fmt::Display for StackTooSmallError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "number of items exceeded list size")
    }
}    
impl error::Error for StackTooSmallError {}


#[cfg(test)]
mod tests {

    use super::*;
    // Create a dummy cargo instance for testing, and fill it with some basic example data.
    fn create_cargo() -> Result<Cargo,Error> {
        let mut cargo = Cargo::new(3);
        cargo.parse_row("[A] [B] [C]")?;
        cargo.parse_row("[D]     [E]")?;
        cargo.parse_row("[F]     [G]")?;
        cargo.parse_row("[H] [I]    ")?;
        Ok(cargo)
    }

    #[test]
    // Test creating a cargo instance and parsing a row string to it
    // 'parse_row'
    fn create_cargo_test() -> Result<(),Error> {
        let mut cargo = create_cargo().expect("Could not create basic cargo instance.");

        // Assert initial conditions
        assert_eq!(cargo.stacks.get(0).unwrap(), &vec!['A','D','F','H']);
        assert_eq!(cargo.stacks.get(1).unwrap(), &vec!['B','I']);
        assert_eq!(cargo.stacks.get(2).unwrap(), &vec!['C','E','G']);

        // Parse an additional row and confirm they are added correctly
        cargo.parse_row("    [J] [K]")?;
        assert_eq!(cargo.stacks.get(0).unwrap(), &vec!['A','D','F','H']);
        assert_eq!(cargo.stacks.get(1).unwrap(), &vec!['B','I','J']);
        assert_eq!(cargo.stacks.get(2).unwrap(), &vec!['C','E','G', 'K']);
        Ok(())
    }

    // Test stack movement logic, both indivieaul items and groups
    // 'move_top_item_between_stacks'
    // 'move_top_items_between_stacks'
    #[test]
    fn move_stacks_test() {
        let mut cargo = create_cargo().expect("Could not create basic cargo instance.");

        // Move top item from 0 to 1
        cargo.move_top_item_between_stacks(0, 1);
        assert_eq!(cargo.stacks.get(0).unwrap(), &vec!['A','D','F']);
        assert_eq!(cargo.stacks.get(1).unwrap(), &vec!['B','I','H']);

        // Move top item from 0 to 1
        cargo.move_top_item_between_stacks(2, 0);
        assert_eq!(cargo.stacks.get(0).unwrap(), &vec!['A','D','F', 'G']);
        assert_eq!(cargo.stacks.get(1).unwrap(), &vec!['B','I','H']);
        assert_eq!(cargo.stacks.get(2).unwrap(), &vec!['C','E']);

        // Move top 3 items in a group from the top of stack 0 to 2
        cargo.move_top_n_items_between_stacks(0,2,3).unwrap();
        assert_eq!(cargo.stacks.get(0).unwrap(), &vec!['A']);
        assert_eq!(cargo.stacks.get(1).unwrap(), &vec!['B','I','H']);
        assert_eq!(cargo.stacks.get(2).unwrap(), &vec!['C','E','D','F', 'G']);

        // Move top 5 items in a group from the top of stack 2 to `
        cargo.move_top_n_items_between_stacks(2,1,5).unwrap();
        assert_eq!(cargo.stacks.get(0).unwrap(), &vec!['A']);
        assert_eq!(cargo.stacks.get(1).unwrap(), &vec!['B','I','H','C','E','D','F', 'G']);
        assert_eq!(cargo.stacks.get(2).unwrap(), &Vec::<char>::new());
    }
}

