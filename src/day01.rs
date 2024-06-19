use std::fs::File;
use std::io::{self, BufRead};

use crate::common::{DEBUG_OFF, DEBUG_MIN, DEBUG_ALL};
pub fn day01_1(debug_level: u8, file_in: &str) -> std::io::Result<i32> {
    if debug_level > DEBUG_OFF {
        println!("Starting Day 01");
    }
    // Read in the data
    let file = File::open(file_in).expect(&format!("Can't open file {file_in}"));
    let mut reader = io::BufReader::new(file);
    // Read the first line into the top value separately
    let mut first_line = String::new();
    let len = reader.read_line(&mut first_line)
        .expect(&format!("Problem reading first line of '{first_line}"));    // Read first line
    let mut previous_value :i32 = first_line.trim_end().parse()
        .expect("Problem getting number from first line");
    let mut increase_count :i32 = 0;
    for line_result in reader.lines() {
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
            if debug_level > DEBUG_MIN {
                println!("Danger! Skipping line {line}");
            }
        }
    }
    if debug_level > DEBUG_OFF {
        println!("Got {} increases!", increase_count);
    }
    Ok(increase_count)
}

pub fn day01_2() -> std::io::Result<i32> {
    println!("Starting!");

    let file = File::open("InputData_01.txt")?;
    let mut lines = io::BufReader::new(file).lines();
    let mut values_list :[i32; 3] = [0; 3];
    for index in 0..3 {
        let line :String = lines.next().unwrap()?;
        let reading :i32 = line.parse().unwrap();
        values_list[index] = reading;
    }
    let mut increase_count = 0;
    for line in lines {
        let previous_sum :i32 = values_list.iter().sum();
        let reading = line?;
        let value : i32 = reading.parse().unwrap();
        values_list = [value, values_list[0], values_list[1]];
        let new_sum :i32 = values_list.iter().sum();
        if previous_sum < new_sum {
            increase_count += 1;
        }
        // println!("{}", previous_sum);
    }
    println!("Got {} increases!", increase_count);
    Ok(increase_count)
}
