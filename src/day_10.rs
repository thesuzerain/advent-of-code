// https://adventofcode.com/2022/day/10
// Day 10: Cathode-Ray Tube
// Part 1: Given a a list of assembly register instructions, calculate the product
// of the current cycle and value of x for cycles 20,60,100,140,180,220
// Part 2: Given the same list of instructions, assuming that a pixel is drawn if the x register
// matches the cycle count on a given cycle, print a pixel on a screen.

use super::*;
use std::{fmt, ops::Index};
use lazy_static::lazy_static;

pub fn run (part_2 : bool) -> Result<(),Box<dyn error::Error>> {

    // Load input text into file buffer
    let f = File::open("input/day10input.txt").unwrap();
    let buf = BufReader::new(f);

    // Initialize CPU
    let mut cpu = CPU::new();

    // Parse each mock assembly command in list
    for line in buf.lines() {
        cpu.parse_command(&line?)?;
    }

    if part_2 {
        println!("Result for day 10-2:\n{}",cpu.draw_screen());

    } else {
        // Part 1: get accumuulated sum of signal strength at designated intervals described in SIGNAL_STRENGTH_CYCLE_INTERVALS
        println!("Result for day 10-1 = {}",cpu.signal_strength_acc);
    }
    
    Ok(())
}


// CPU cycle intervals upon which to increment the 'signal strength' accumulator (for part 1).
// Should be sorted.
const SIGNAL_STRENGTH_CYCLE_INTERVALS : [usize; 6] = [20,60,100,140,180,220];

// Image dimensions for pixel image being drawn (for part 2)
// The first IMG_WIDTH many pixels compose the first row, the second set will be the second row, etc.
const IMG_WIDTH : usize = 40;
const IMG_HEIGHT : usize = 6;


// CPU simulator that contains single register 'x'.
// It can run CPUCommands to change 'x'', and it keeps track of
// the cycles, signal strength, and pixels being drawn as it does so.
// "Signal strength" => The product of the x register and the cycle count during a given cycle.
// "Pixel" => a binary lit/notlit value that is lit if at a given cycle c, the register x is +/- 1 from c.
#[derive(PartialEq, Debug)]
struct CPU {
    x : i32,
    cycles: usize, // each command costs 1 or more cycles
    signal_strength_acc: i32, // Accumulator of signal strength at cycles in SIGNAL_STRENGTH_CYCLE_INTERVALS
    pixel_array: [bool; IMG_WIDTH * IMG_HEIGHT] // flattened
}

#[derive(Debug)]
enum CPUCommand {
    Addx(i32), // adds the contained value to x
    Noop 
}

#[derive(Debug)]
struct ParseCommandError { s: String}
impl error::Error for ParseCommandError {}
impl fmt::Display for ParseCommandError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f,"could not parse text into command: {}",self.s)
    }
}


impl CPU {

    // Creates a new CPU instance
    // All values are empty
    // 'x' register starts at 1
    fn new() -> CPU {
        CPU { x: 1, cycles: 0, signal_strength_acc: 0, pixel_array: [false; IMG_WIDTH * IMG_HEIGHT] }
    }

    // Parse a line representing a CPU command and applies it to the current instance.
    // Syntax is either:
    // - 'noop' 
    // - 'addx (some number)' 
    fn parse_command(&mut self, line : &str) -> Result<(),ParseCommandError> {
        lazy_static!{
            static ref REGEX_ADDX_PARSE : Regex = Regex::new(r"addx\s([0-9\-]+)").unwrap();
            static ref REGEX_NOOP : Regex = Regex::new(r"noop$").unwrap();
        }

        // Regex capture for 'addx (somenumber)' command
        if let Some(cap) = REGEX_ADDX_PARSE.captures(line) {
            let x = cap.get(1).unwrap(); // Unwraps, as if values were captured then first index must exist, and it must be digital
            self.run_command(CPUCommand::Addx(x.as_str().parse().unwrap()));
            return Ok(());
        }

        // Regex capture for 'noop' command
        if let Some(cap) = REGEX_NOOP.captures(line) {
            self.run_command(CPUCommand::Noop);         
            return Ok(());
   
        }

        // No commands matched
        Err(ParseCommandError{s:line.to_string()})
    }

    // Ticks cycle up
    // Draws pixel and/or  adds to signal strength accumulator if appropriate
    fn tick_cycle(&mut self){

        self.cycles += 1;
        self.draw_pixel_for_current_cycle();

        // Increment signal strength if if its a marked cycle
        if SIGNAL_STRENGTH_CYCLE_INTERVALS.contains(&self.cycles) {
            self.signal_strength_acc += self.x * self.cycles as i32;
        }

    }

    // Ticks cycle up 'amount' many times
    fn tick_cycles(&mut self, amount : i32) {
        for _ in 0..amount {
            self.tick_cycle()
        }
    }

    // Draws a pixel on the image at the index of the current cycle
    // Does so if:
    // - the cycle count can be identified to a pixel on the image (does not exceed the pixel count)
    // - the register x at the time of this cycle occurring is within 1 of the current cycle count
    fn draw_pixel_for_current_cycle(&mut self) {
        if self.cycles > IMG_HEIGHT * IMG_WIDTH {
            return;
        }
        if self.x < 1 && self.x as usize > IMG_HEIGHT * IMG_WIDTH {
            return;
        }

        let x_register = self.x as usize;
        let image_x_pos = (self.cycles-1) % IMG_WIDTH;
        let image_y_pos = (self.cycles-1) / IMG_WIDTH;

        // Draws pixel if in range
        if x_register >= 1 && image_x_pos >= x_register - 1 && image_x_pos <= x_register + 1 {
            self.pixel_array[image_x_pos + IMG_WIDTH*image_y_pos] = true;
        } 
    }

    // Prints the screen of pixels, with lit pixels as '#' and unlit pixels as '.'
    // Pixel image is IMAGE_WIDTH x IMAGE_HEIGHT in size 
    fn draw_screen(&self) -> String {
        let s = self.pixel_array.iter().map(|b| if *b {'#'} else {'.'});
        let mut s : String = s.collect();

        // Retroactively insert newline characters into string to format single line into a rectangular screen
        for i in (1..IMG_HEIGHT).rev() {
            s.insert(i*IMG_WIDTH, '\n')
        }
        s
    }
    
    // Delegates handling of a CPUCommand to a helper function for it, and ticks cycles the appropriate number of times
    fn run_command (&mut self, command : CPUCommand)  {
        match command {
            CPUCommand::Addx(i) => { 
                self.tick_cycles(2);
                self.x += i;
            },
            CPUCommand::Noop => self.tick_cycle(),
        }
    }
}



#[cfg(test)]
mod tests {

    use super::*;

    // Tests functionality of simple CPUCommand logic and tests signal strength accumulator as it does so
    #[test]
    fn test_command_cycles() {

        let mut cpu = CPU::new();
        assert_eq!(cpu.x, 1);
        assert_eq!(cpu.cycles, 0);
        assert_eq!(cpu.signal_strength_acc, 0);
        
        // Run 'noop' 5 times to: advance cycle 5 times
        for _ in 0..5 {
            cpu.run_command(CPUCommand::Noop)
        }
        assert_eq!(cpu.x, 1);
        assert_eq!(cpu.cycles, 5);
        assert_eq!(cpu.signal_strength_acc, 0);

        // Run 'addx' to: add 3 and advance cycle 2 times
        cpu.run_command(CPUCommand::Addx(3));
        assert_eq!(cpu.x, 4);
        assert_eq!(cpu.cycles, 7);
        assert_eq!(cpu.signal_strength_acc, 0);

        for _ in 0..11 {
            cpu.run_command(CPUCommand::Noop)
        }
        assert_eq!(cpu.x, 4);
        assert_eq!(cpu.cycles, 18);
        assert_eq!(cpu.signal_strength_acc, 0);

        // Add 10
        // This reaches 20 cycles and adds the 10 value to x AFTER that, so the x=10 
        // should not be reflected in the single signal strength accumulator
        cpu.run_command(CPUCommand::Addx(10));
        assert_eq!(cpu.x, 14);
        assert_eq!(cpu.cycles, 20);
        assert_eq!(cpu.signal_strength_acc, 20*4);

        for _ in 0..39 {
            cpu.run_command(CPUCommand::Noop)
        }

        // Subtract 5
        // This reaches 60 cycles and subtracts the 5 value from x AFTER that, so the x=5 
        // should not be reflected in the single signal strength accumulator, but the previous +10 should be.
        cpu.run_command(CPUCommand::Addx(-5));
        assert_eq!(cpu.x, 9);
        assert_eq!(cpu.cycles, 61);
        assert_eq!(cpu.signal_strength_acc, 20*4 + 60*14);


    }

    // Tests parsing string commands also function identically
    #[test]
    fn test_parse_assembly_command() -> Result<(), ParseCommandError> {
        let mut cpu = CPU::new();
        
        // Noop should advance the cycle by 1 and make no other changes
        cpu.parse_command("noop")?;
        assert_eq!(cpu.x, 1);
        assert_eq!(cpu.cycles, 1);
        assert_eq!(cpu.signal_strength_acc, 0);


        // Add and subtract values from x, each of which should increment cycle by 2
        cpu.parse_command("addx 3")?;
        assert_eq!(cpu.x, 4);
        assert_eq!(cpu.cycles, 3);
        assert_eq!(cpu.signal_strength_acc, 0);

        cpu.parse_command("addx -13")?;
        assert_eq!(cpu.x, -9);
        assert_eq!(cpu.cycles, 5);
        assert_eq!(cpu.signal_strength_acc, 0);

        Ok(())
    }

    #[test]
    fn test_display_pixels() {
        let mut cpu = CPU::new();
        let mut test_pixel_array = [false; IMG_HEIGHT * IMG_WIDTH];
        assert_eq!(cpu.pixel_array, test_pixel_array );
        
        cpu.run_command(CPUCommand::Noop);
        test_pixel_array[1 - 1] = true;
        assert_eq!(cpu.x, 1);
        assert_eq!(cpu.cycles, 1);
        assert_eq!(cpu.pixel_array, test_pixel_array);

        cpu.run_command(CPUCommand::Addx(3));
        test_pixel_array[2 - 1] = true;
        test_pixel_array[3 - 1] = true;
        assert_eq!(cpu.x, 4);
        assert_eq!(cpu.cycles, 3);
        assert_eq!(cpu.pixel_array, test_pixel_array);

        cpu.run_command(CPUCommand::Noop);
        test_pixel_array[4 - 1] = true;
        assert_eq!(cpu.x, 4);
        assert_eq!(cpu.cycles, 4);
        assert_eq!(cpu.pixel_array, test_pixel_array);


    }

}