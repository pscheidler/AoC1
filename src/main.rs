#![allow(dead_code)]
mod common;
mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;

use std::io;
use crate::common::{*};


fn main() -> io::Result<()> {
    // day01::day01_1(DEBUG_MIN, "InputData_01.txt")?;
    // day01::day01_2(DEBUG_MIN, "InputData_01.txt")?;
    // day02::day02_1(DEBUG_MIN, "InputData_02.txt")?;
    // day02::day02_2(DEBUG_MIN, "InputData_02.txt")?;
    // day03::day03_1(DEBUG_ALL, "InputData_03.txt")?;
    // day03::day03_2(DEBUG_ALL, "InputData_03.txt")?;
    // day04::day04_1(DEBUG_ALL, "InputData_04.txt")?;
    // day04::day04_2(DEBUG_ALL, "InputData_04.txt")?;
    // day05::day05_1(DEBUG_ALL, "InputData_05.txt")?;
    day05::day05_2(DEBUG_ALL, "InputData_05.txt")?;
    // day05::day05_2(DEBUG_ALL, "TestInput_05.txt")?;
    Ok(())
}
