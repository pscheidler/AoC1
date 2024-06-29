use std::collections::VecDeque;
use std::fs::File;
use std::io::{self, BufRead};

use crate::common::{DEBUG_OFF, DEBUG_MIN, DEBUG_ALL};
pub fn day01_1(debug_level: u8, file_in: &str) -> std::io::Result<i32> {
    if debug_level > DEBUG_OFF {
        println!("Starting Day 01, Part 1");
    }
    // Read in the data
    let file = File::open(file_in).expect(&format!("Can't open file {file_in}"));
    let mut lines = io::BufReader::new(file).lines();
    // Read the first line into the top value separately
    let first_line = lines.next()
        .expect("Problem running next on first line")
        .expect("Problem reading first line out");
    let mut previous_value :i32 = first_line.parse()
        .expect("Problem getting number from first line");
    let mut increase_count :i32 = 0;
    // Parse through the lines and get the number of increases
    for line_result in lines {
        // Check for problems reading the line
        let line = line_result.expect("Problem reading line");  // panic on any big line errors
        // Check for problems parsing the line into an int32
        if let Ok(value) = line.parse::<i32>() {
            // Parse successful
            if debug_level == DEBUG_ALL {
                println!("Read {value}");
            }
            //
            if previous_value >= 0 && previous_value < value {
                increase_count += 1;
            }
            previous_value = value;
        } else {
            // Could not parse the line
            if debug_level > DEBUG_MIN {
                println!("Danger! Skipping line '{line}'");
            }
        }
    }
    if debug_level > DEBUG_OFF {
        println!("Got {} increases!", increase_count);
    }
    Ok(increase_count)
}

pub fn day01_2(debug_level: u8, file_in: &str) -> std::io::Result<i32> {
    if debug_level > DEBUG_OFF {
        println!("Starting Day 01, Part 2");
    }
    let buffer_len = 3;
    let file = File::open(file_in).expect(&format!("Problem opening '{file_in}'"));
    let mut lines = io::BufReader::new(file).lines();
    let mut value_buffer = VecDeque::<i32>::with_capacity(4);
    let mut buffer_sum :i32 = 0;
    let mut increase_count :i32 = 0;
    // Get the buffer filled and find the first value
    for index in 0..buffer_len {
        let value :i32 = lines.next()
            .expect("Can't access line using next")
            .expect("Error reading line")
            .parse()
            .expect("Can't parse line");
        println!("Priming with {value}");
        value_buffer.push_back(value);
        buffer_sum += value;
    }

    for line_result in lines {
        let line = line_result.expect("Problem reading line");  // panic on any big line errors
        if let Ok(new_value) = line.parse::<i32>() {
            // Parse successful
            if debug_level == DEBUG_ALL {
                println!("Read {new_value}");
            }
            // Stash the existing value, pup the current value out, and see if the windowed sum has increased
            // Note that adding the new value in is done last
            let temp_sum = buffer_sum;
            let pop_value = value_buffer.pop_front().ok_or(0).expect("Can't pop value for some reason");
            println!("Popped {pop_value}");
            buffer_sum -= pop_value;
            buffer_sum += new_value;
            value_buffer.push_back(new_value);
            if buffer_sum > temp_sum {
                increase_count += 1;
                if debug_level == DEBUG_ALL {
                    let added = buffer_sum;
                    println!("Increased from {temp_sum} to {added}");
                }
            }
            // Push the new value back on the stack
        }
    }
    if debug_level > DEBUG_OFF {
        println!("Got {increase_count} increases!");
    }
    Ok(increase_count)
}
