// https://adventofcode.com/2022/day/6
// Day 6: Tuning Trouble.
// Given a an initial string representing a signal, return the first point in the string where you 
// have 'x' unique characters in a row, representing a 'start-of-packet' marker.
// In part 1, x = 3.
// In part 2, x = 14

use super::*;

// Run challenge.
// Main entry point to day 6 challenge.
pub fn run(part_2: bool) ->Result<(), Box<dyn error::Error>> {

    // Load input file to BufReader
    let f = File::open("input/day6input.txt")?;
    let buf = BufReader::new(f);

    for line in buf.lines() {
        // Read signal characters into a string
        let line = line?;
        let part = if part_2 {2} else {1};
        let marker_length = if part_2 {14} else {4};

        // Get location of start marker in 'line' based on start marker size 'marker_length'
        // Start marker is point where 'marker_length' unique characters in a row first appear
        let start_marker = match get_start_marker(&line, marker_length) {
            Some(t) => t,
            None => {
                println!("Result for day 6-{part} = Could not find a start marker.");
                return Err(Box::new(Error::new(ErrorKind::Other, "Could not find a start marker.")));
            },
        };
        println!("Result for day 6-{part} = {start_marker}");
        break; // Only need first line
    }
    Ok(())
}

// Gets location of start marker of size 'marker_length' for alphabetic string 'stream'
// The start marker represents the first position in the string for which there have been
// 'marker_length' unique characters in a row.
// (return a None if no such start_marker exists).
fn get_start_marker(stream : &str, marker_length: usize) -> Option<usize> {

    // Maintain array of the most recent 'marker_length' many characters
    let mut recents = Vec::new();
    recents.reserve(marker_length-1);

    for (i,c) in stream.chars().enumerate() {
        // Add current value to recent value array, up to size 'marker_length'
        // If max length reached, replace oldest element (FIFO)
        if recents.len() < marker_length {
            recents.push(c);
        } else {
            recents[i % marker_length] = c;
        }

        if recents.len() >= marker_length {
            // If marker is full, check if there are any duplicates in recent character array (making it not a valid marker)
            let duplicate = check_duplicates(recents.to_vec());
            if !duplicate {
                return Some(i + 1); //if valid marker, return index +1 because advent of code design specifies one-indexed
            }
        }
    }
    None
}


// Checks for duplicates elements in a Vector
// Since we are using characters, which can be ordered, do the n log n solution of sorting and iterating through.
fn check_duplicates<T : PartialEq + Ord>(arr : Vec<T>) -> bool {
    // n log n
    let mut arr = Vec::from(arr);
    arr.sort();

    // Iterate to find any duplicates in a row
    for i in 0..arr.len() {
        match (arr.get(i), arr.get(i + 1)) {
            (Some(x), Some(y)) => {
                if x == y { return true }
            }
            _ => continue
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::get_start_marker;
    use crate::day_6::check_duplicates;

    #[test]
    fn check_duplicates_test() {
        // Ensures duplicate function correctly identifies presence of duplicates in lists of ordinal
        assert!(!check_duplicates(vec![0,1,2,3,4,5]));
        assert!(!check_duplicates(vec!['A','B','C','D','E']));
        assert!(check_duplicates(vec!['A','B','E','C','D','E']));
    }


    #[test]
    fn signal_start_markers() {

        // index of start marker of all-unique string should be the same as the marker size
        assert_eq!(get_start_marker("ABCDEF",4), Some(4));
        
        // index of start marker of all-identical string should be None (no marker)
        assert_eq!(get_start_marker("AAAAAA",4), None);

        // simple examples of markers at various points of list of characters
        assert_eq!(get_start_marker("AAABCDEF",4), Some(6));
        assert_eq!(get_start_marker("AAABBBCDEF",4), Some(9));

        // Advent of Code challenge-provided examples
        assert_eq!(get_start_marker("bvwbjplbgvbhsrlpgdmjqwftvncz",4), Some(5));
        assert_eq!(get_start_marker("nppdvjthqldpwncqszvftbrmjlhg",4), Some(6));
        assert_eq!(get_start_marker("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg",4), Some(10));
        assert_eq!(get_start_marker("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw",4), Some(11));
        assert_eq!(get_start_marker("mjqjpqmgbljsphdztnvjfqwrcgsmlb",14), Some(19));
        assert_eq!(get_start_marker("bvwbjplbgvbhsrlpgdmjqwftvncz",14), Some(23));
        assert_eq!(get_start_marker("nppdvjthqldpwncqszvftbrmjlhg",14), Some(23));
        assert_eq!(get_start_marker("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg",14), Some(29));
        assert_eq!(get_start_marker("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw",14), Some(26));

    }

}