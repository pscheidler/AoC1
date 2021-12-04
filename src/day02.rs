use std::fs::File;
use std::io::{self, BufRead};

struct Command {
    dir : String,
    dist: u32
}

fn read_command(line: String) -> std::io::Result<Command> {
    let mut decoder = line.split(" ");

    let dir :String = String::from(decoder.next().unwrap());
    let dist_string = decoder.next().unwrap();
    Ok(Command{dir, dist:dist_string.parse().unwrap()})
}

pub fn part1() -> std::io::Result<()> {
    let file = File::open("InputData_02.txt")?;
    let lines = io::BufReader::new(file).lines();

    let mut horiz = 0;
    let mut depth = 0;
    let mut aim = 0;

    for line in lines {
        let command: Command = read_command(line?).unwrap();
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
    println!("Result = {}", horiz * depth);
    Ok(())
}