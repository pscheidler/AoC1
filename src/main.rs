mod day01_2;
mod day02;

use std::fs::File;
use std::io::{self, BufRead};
//use std::io::prelude::*;

fn day01_1() -> std::io::Result<()> {
    println!("Starting!");

    let file = File::open("InputData_01.txt")?;
    let lines = io::BufReader::new(file).lines();
    let mut previous_value = -1;
    let mut increase_count = 0;
    for line in lines {
        let reading = line?;
        let value : i32 = reading.parse().unwrap();
        if previous_value >= 0 && previous_value < value {
            increase_count += 1;
        }
        previous_value = value;
        println!("{}", value);
    }
    println!("Got {} increases!", increase_count);
    Ok(())
}

fn main() -> std::io::Result<()> {
    // day01_2::day01_2()?;
    day02::part1();
    Ok(())
}
