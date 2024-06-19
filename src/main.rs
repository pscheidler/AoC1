#![allow(dead_code)]
mod common;
mod day01;
mod day02;
mod day03;

use std::io;
use crate::common::{*};

fn main() -> io::Result<()> {
    day01::day01_1(DEBUG_MIN, "InputData_01.txt")?;
    // day01_2::day01_2()?;
    // day03::part2()?;
    Ok(())
}
