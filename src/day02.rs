use std::fs::File;
use std::io::{self, BufRead};
use crate::common::{DEBUG_MIN, DEBUG_OFF, DEBUG_ALL};

struct Command {
    dir : String,
    dist: u32
}

fn read_command(debug_level: u8, line: String) -> std::io::Result<Command> {
    let mut decoder = line.split(" ");

    let dir :String = String::from(decoder.next().expect("Can't get first element from line"));
    let dist :u32 = decoder.next()
        .expect("Missing element from line")
        .parse()
        .expect("Can't parse line");
    if debug_level == DEBUG_ALL {
        println!("Turned {line} into {dir}, {dist}")
    }
    Ok(Command{dir, dist})
}

pub fn day02_1(debug_level: u8, file_in: &str) -> std::io::Result<u32> {
    if debug_level > DEBUG_OFF {
        println!("Starting Day 02, Part 1")
    }
    let file = File::open(file_in).expect(&format!("Can't open {file_in}"));
    let lines = io::BufReader::new(file).lines();

    let mut horiz = 0;
    let mut depth = 0;

    for line_result in lines {
        let line = line_result.expect("Can't read line");
        let command: Command = read_command(debug_level, line).unwrap();
        if command.dir == "forward" {
            horiz += command.dist;
        } else if command.dir == "up" {
            depth -= command.dist;
        } else if command.dir == "down" {
            depth += command.dist;
        } else {
            println!("Unknown command {}", command.dir);
        }
    }
    if debug_level > DEBUG_OFF {
        println!("Result = {}", horiz * depth);
    }
    Ok(horiz * depth)
}


pub fn day02_2(debug_level: u8, file_in: &str) -> std::io::Result<u32> {
    if debug_level > DEBUG_OFF {
        println!("Starting Day 02, Part 1")
    }
    let file = File::open(file_in).expect(&format!("Can't open {file_in}"));
    let lines = io::BufReader::new(file).lines();

    let mut horiz = 0;
    let mut depth = 0;
    let mut aim = 0;

    for line_result in lines {
        let line = line_result.expect("Can't read line");
        let command: Command = read_command(debug_level, line).unwrap();
        if command.dir == "forward" {
            horiz += command.dist;
            depth += command.dist * aim;
        } else if command.dir == "up" {
            aim -= command.dist;
        } else if command.dir == "down" {
            aim += command.dist;
        } else {
            println!("Unknown command {}", command.dir);
        }
    }
    if debug_level > DEBUG_OFF {
        println!("Result = {}", horiz * depth);
    }
    Ok(horiz * depth)
}