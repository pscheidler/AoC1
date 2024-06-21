#![allow(dead_code)]
mod common;
mod day01;
mod day02;
mod day03;

use std::io;
use crate::common::{*};

fn main() -> io::Result<()> {
    // day01::day01_1(DEBUG_MIN, "InputData_01.txt")?;
    // day01::day01_2(DEBUG_MIN, "InputData_01.txt")?;
    // day02::day02_1(DEBUG_MIN, "InputData_02.txt")?;
    // day02::day02_2(DEBUG_MIN, "InputData_02.txt")?;
    // day03::day03_1(DEBUG_ALL, "InputData_03.txt")?;
    day03::day03_2(DEBUG_ALL, "InputData_03.txt")?;
    Ok(())
}
