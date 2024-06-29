use std::fs::File;
use std::{io, usize, io::BufRead};
use crate::common::DEBUG_OFF;

const SCHEDULE_SIZE: usize = 9;

fn process_input(line_in: String) -> [i64; SCHEDULE_SIZE] {
    let mut return_array: [i64; SCHEDULE_SIZE] = [0; SCHEDULE_SIZE];
    for day_str in line_in.split(",") {
        let day_int: usize = usize::from_str_radix(day_str, 10).expect("Cannot convert to int");
        if day_int > 7 {
            panic!("Read value of {day_int}, too big!");
        }
        return_array[day_int] += 1;
    }
    return_array
}

pub fn day06_1(debug_level: u8, file_in: &str, days: i32) -> std::io::Result<i64> {
    if debug_level > DEBUG_OFF {
        println!("Starting Day 06, Part 1")
    }
    let file = File::open(file_in).expect(&format!("Can't open {file_in}"));
    let reset_time: usize = 6;
    let mut lines = io::BufReader::new(file).lines();
    let mut schedule: [i64; SCHEDULE_SIZE] = process_input(lines.next().expect("Could not get line").expect("Could not get line 2"));
    for counter in 0..days {
        let breeders: i64 = schedule[0];
        for index in 0..SCHEDULE_SIZE-1 {
            schedule[index] = schedule[index+1];
        }
        schedule[reset_time] += breeders;
        schedule[SCHEDULE_SIZE-1] = breeders;
    }
    let final_count: i64 = schedule.iter().sum();
    println!("Result is {final_count}");
    Ok(final_count)
    // crate::day05::day05(debug_level, file_in, false)
}


pub fn day06_2(debug_level: u8, file_in: &str, days: i32) -> std::io::Result<i64> {
    if debug_level > DEBUG_OFF {
        println!("Starting Day 06, Part 2")
    }
    let file = File::open(file_in).expect(&format!("Can't open {file_in}"));
    let reset_time: usize = 6;
    let mut lines = io::BufReader::new(file).lines();
    let mut schedule: [i64; SCHEDULE_SIZE] = process_input(lines.next().expect("Could not get line").expect("Could not get line 2"));
    for counter in 0..days {
        let breeders: i64 = schedule[0];
        for index in 0..SCHEDULE_SIZE-1 {
            schedule[index] = schedule[index+1];
        }
        schedule[reset_time] += breeders;
        schedule[SCHEDULE_SIZE-1] = breeders;
    }
    let final_count: i64 = schedule.iter().sum();
    println!("Result is {final_count}");
    Ok(final_count)
    // crate::day05::day05(debug_level, file_in, false)
}
