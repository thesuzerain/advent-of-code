// https://adventofcode.com/2022/day/3
// Day 3: Rucksack Reorganization
// Given a list of letter inputs representing items in jumbled rucksacks...
// Part 1: ... find the sum over all rucksacks for common items between the left and right half of a given rucksack, as integers.
// Part 2: ... find the common item in each group of 3 sequential rucksacks.


use super::*;

// Rucksack 
// A sack of items (as letter characters a-zA-Z) in either the left or right comparment of the sack.
// The two compartments are valid and even in length. To split evenly, the total number of items must even.
struct Rucksack {
    left_compartment : String,
    right_compartment : String,
}


// Run challenge.
// Main entry point to day 3 challenge.
pub fn run(part_2: bool) -> Result<(),Box<dyn error::Error>> {

    let mut priority_sum = 0;

    // Load data from file into buffer and iterate over lines
    let f = File::open("input/day3input.txt")?;
    let buf = BufReader::new(f);

    if part_2 {
        // Part 2
        // Read in rucksacks as groups of 3, and find the one letters that all three of those rucksacks have in common.
        // Create a sum of the priorities of those letters.
        // TODO: not possible for challenge input, but add error handling for if number of rucksacks%3 != 0
        let mut lines = buf.lines();
        while let (Some(a), Some(b), Some(c)) = (lines.next(), lines.next(), lines.next()){
            let (a,b,c) = (a?, b?, c?);

            // Get first identifiable common character between the 3 rucksacks
            let common = get_alphabet_chars_in_common(&a, &b);
            let common = get_alphabet_chars_in_common(&common, &c);

            priority_sum += match common.chars().next() {// According to specification, there should only be one, so we return the 1st
                Some(i) => get_priority(i),
                None => 0,
            }
        }
    } else {
        // Part 1
        // For each rucksack, find the unique char that the two halves have in common.
        // Get sum of the priorities of items in common between halves
            for line in buf.lines() {
            let rucksack = Rucksack::new(line?)?;
            priority_sum += match rucksack.get_misplaced_item() {
                Some(i) => get_priority(i),
                None => 0,
            };
        }
    }
    let part = if part_2 {2} else {1};
    println!("Result for day 3-{part} = {priority_sum}");
    Ok(())
}

impl  Rucksack {

    // Create a new Rucksack 
    // 'item_string' a list of items as a String of upper and lower case letters.
    // 'item_string' must have an even number of characters to split in half.
    fn new(mut item_string: String) -> Result<Rucksack, Error> {
        // TODO: tidy
        let len = item_string.len();
        if len % 2 != 0 {
            Err(std::io::Error::new(ErrorKind::Other,"Rucksack has uneven length of numbers and cannot be equally split."))
        } else {
            let right_compartment = item_string.split_off(len / 2);

            Ok(Rucksack {
                left_compartment: item_string,
                right_compartment,
            })
        }
    }

    // Find character in common between two compartments
    fn get_misplaced_item(&self) -> Option<char> {
        let common = get_alphabet_chars_in_common(&self.left_compartment, &self.right_compartment);
        common.chars().next() // According to specification, there should only be one, so we return the 1st
    }

}

// Returns the 'priority' of a item character (as per specification of puzzle)
// a-z ~ 1-26
// A-Z ~ 27-52
fn get_priority(c: char) -> u32 {
    match  c {
        'a'..='z' => (c as u32 - 'a' as u32) + 1,
        'A'..='Z' => (c as u32 - 'A' as u32) + 27,
        _ => panic!("Non-English alphabetical character {c} found in input string."),
    }
}

// For two strings, return all characters those strings have in common as another String
// Stores integer as indices in a boolean array to keep time ~ O(n)
fn get_alphabet_chars_in_common(s1: &String, s2: &String) -> String {

    let mut existing_items = [false; 52];
    let mut common = String::new(); 

    for c in s1.chars() {
        existing_items[get_priority(c) as usize - 1] = true;
    }
    for c in s2.chars() {
        if existing_items[get_priority(c) as usize - 1] {
            common.push(c);
        }
    }
    common
}