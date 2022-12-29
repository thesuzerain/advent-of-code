// https://adventofcode.com/2022/day/4
// Camp Cleanup.
// Given a list of inputs representing jumbled rucksacks, find unique values within substrings of those inputs, and 
// in part 2, return letters in common between groups of 3.
use super::*;


// A pair of Elves (elves 'a' and 'b') who each encompass a range of values a_0 -> a_1 and b_0 -> b_1
struct ElfPair {
    a_0: i32, // elf a, range start
    a_1: i32, // elf a, range end
    b_0: i32,
    b_1: i32
}
impl ElfPair {
    
    // Converts a string representing an ElfPair into an ElfPair struct
    // String consists of two comma-separated ranges where each range is two integers hyphen-separated
    // s => "1-5,2-8"
    fn new (s: &str) -> Result<ElfPair, Error> {
        let mut s = s.split(',');
        let (a, b) = match (s.next(), s.next()) {
            (Some(s_0), Some(s_1)) => (s_0,s_1),
            (None, _) | (_, None) => return Err(Error::new(ErrorKind::Other, "Line was not formatted correctly and could not recognize two distinct ranges."))
        };
    
        let (a_0, a_1) = unwrap_into_range(a)?;
        let (b_0, b_1) = unwrap_into_range(b)?;

        if b_1 < b_0 || a_1 < a_0{
            return Err(Error::new(ErrorKind::Other, "The second value must be higher than the first in the given range."))
        }

        Ok(ElfPair {
            a_0,
            a_1,
            b_0,
            b_1
        })
    }

    // Checks whether one of the ranges defined in ElfPair totally encompasses another
    fn check_encompass(&self) -> bool {
        if self.a_0 >= self.b_0 && self.a_1 <= self.b_1 {
            return true
        }
        if self.a_0 <= self.b_0 && self.a_1 >= self.b_1 {
            return true
        }
        return false
    }
    // Checks whether one of the ranges defined in ElfPair shares overlap with another
    fn check_overlap(&self) -> bool {
        if self.a_1 < self.b_0 || self.b_1 < self.a_0 {
            return false
        }
        return true
    }



}

// Unravels a range string slice into two separate integers
// eg: '2-5' => (2, 5)
fn unwrap_into_range(s: &str) -> Result<(i32, i32), Error> {
    let mut s = s.split('-');
    let (s_0, s_1) = match (s.next(), s.next()) {
        (Some(s_0), Some(s_1)) => (s_0,s_1),
        (None, _) | (_, None) => return Err(Error::new(ErrorKind::Other, "One of the ElfPairs could not be formatted into a number range."))
    };
    Ok((s_0.parse().expect("Range value was not an integer."),
        s_1.parse().expect("Range value was not an integer.")))

}

pub fn run(part_2: bool) -> Result<(), Box<dyn error::Error>> { 
    let mut counter = 0;

    let f = File::open("input/day4input.txt") ?;
    let buf = BufReader::new(f);

    // For each line, extract an ElfPair and apply either the part 1 check (whether their schedules encompass each other),
    // or the part 2 check (Whether their schedules overlap)
    for line in buf.lines() {
        let line = line?;
        let elfpair = ElfPair::new(&line)?;
        if (!part_2 && elfpair.check_encompass()) || (part_2 && elfpair.check_overlap() ) {
            counter += 1;
        }
    }
    let part = if part_2 {2} else {1};
    println!("Result for day 4-{part} = {counter}");

    Ok(())
}