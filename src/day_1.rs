use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;

pub fn day_one_run() -> std::io::Result<()>{
    println!("Running day 1.");

    let f = File::open("input/day1input.txt")?;
    let reader = BufReader::new(f);

    // Create a new calorie counter to parse the file
    let mut calorie_counter = CalorieCount {
        current_calorie_count: 0,
        top_calorie_records: [0, 0, 0],
    };

    // Iterate through each line of the input
    for line in reader.lines(){

        let line = line?;

        // If line is a newline or empty, this marks the end of calorie list for this elf
        // Store current value (if high enough) and reset current counter
        if line.trim().is_empty() {
            calorie_counter.store_current_if_top_record();
            calorie_counter.current_calorie_count = 0;
        }

        // Attempts to read calorie count as an integer, adds to calorie counter if so
        let calories = line.trim().parse::<i32>?; // panics if cannot read (some non-newline/numeric value)
        calorie_counter.current_calorie_count += calories;
    }

    // Prints result
    println!("{}",calorie_counter.records_sum());
    Ok(())
}


// Calorie counter for iterating through list of calorie-counts
// Maintains a record of highest 3 found so far
struct CalorieCount {
    current_calorie_count: i32, // current elf's calorie total
    top_calorie_records: [i32; 3], // highest 3 calorie counts found so far, unordered
}

impl CalorieCount {
    

    // If current calorie score is higher any of the records, it replaces the lowest record
    fn store_current_if_top_record(&mut self) {
        let mut lowest_record_index = 0;

        // Check for lowest record
        for (i,calorie_record) in self.top_calorie_records.iter().enumerate() {
            if calorie_record < &self.top_calorie_records[lowest_record_index] {
                lowest_record_index = i;
            }
        }
        // If new value is higher, replace the lowest value
        if self.current_calorie_count > self.top_calorie_records[lowest_record_index] {
            self.top_calorie_records[lowest_record_index] = self.current_calorie_count;
        }
    }

    // Sum of top_calorie_records array
    fn records_sum(&self) -> i32 {
        let mut sum = 0;
        for calorie_record in self.top_calorie_records.iter() {
            sum += calorie_record;
        }
        sum
    }
}