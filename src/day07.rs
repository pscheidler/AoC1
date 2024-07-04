/// Goal is to take in an array of points (eg 16,1,2,0,4,2,7,1,2,14) and find the point they can travel
/// to with the lowest cost (2 in the previous example, which has a total cost of 37)
/// So Min(sum(abs(X - P) for X in Input)
/// It looks like this is just the Median? We could also find it with a gradient search

/// My homemade search algorithm: Take the set of points,
/// Pick a starting point S, if cost of S+1 > cost of S, S -= N, else S += N
/// N = N / 2
/// When N = 1, keep going until you find the minimum
use std::fs::File;
use std::io::{self, BufRead}; //, Lines, BufReader
use crate::common::{DEBUG_OFF, DEBUG_MIN, DEBUG_ALL};

fn cost_function(target: i32, start: &Vec<i32>, part1: bool) -> i32 {
    if part1 {
        start.iter().map(|x| (x - target).abs()).sum()
    } else {
        start.iter().map(|x| (0..=(x - target).abs()).sum::<i32>()).sum()
    }
}

fn process_input(line_in:String) -> Vec<i32> {
    line_in.split(",")
        .map(|x| i32::from_str_radix(x, 10).expect("Could not turn into int"))
        .collect()
}

fn check_point(target: i32, step: i32, starting_points: &Vec<i32>, part1: bool) -> [i32; 3] {

    let return_value: [i32; 3] = [cost_function(target-step, starting_points, part1),
        cost_function(target, starting_points, part1),
        cost_function(target+step, starting_points, part1)];
    return_value
}

pub fn day07(debug_level: u8, file_in: &str, part1: bool) -> std::io::Result<i32> {
    if debug_level > DEBUG_OFF {
        println!("Starting Day 06, Part 1")
    }
    let file = File::open(file_in).expect(&format!("Can't open {file_in}"));
    let mut lines = io::BufReader::new(file).lines();
    let starting_points: Vec<i32> = process_input(lines.next().expect("Could not get line").expect("Could not get line 2"));
    let start_max = starting_points.iter().max().unwrap();
    let start_min = starting_points.iter().min().unwrap();
    let mut step_size: i32 = (start_max - start_min) / 2;
    let mut search_point: i32 = start_min + step_size;
    loop {
        if step_size > 1 {
            step_size = step_size / 2;
        }
        let near_area: [i32; 3] = check_point(search_point, 1, &starting_points, part1);
        println!("Checking at {search_point}, has fuel at {}", near_area[1]);
        if near_area[1] < near_area[0] && near_area[1] < near_area[2] {
            println!("Found hit at {search_point}, fuel = {}!", near_area[1]);
            return Ok(near_area[1]);
        }
        if near_area[1] == near_area[0] || near_area[1] == near_area[2] {
            println!("Found plateau at {search_point}, fuel = {}!", near_area[1]);
            return Ok(near_area[1]);
        }
        if near_area[0] < near_area[1] {
            search_point -= step_size;
        } else {
            search_point += step_size;
        }
    }
}