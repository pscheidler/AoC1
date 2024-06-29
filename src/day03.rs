use std::fs::File;
use std::io::{self, BufRead}; //, Lines, BufReader
use crate::common::{DEBUG_OFF, DEBUG_MIN, DEBUG_ALL};


fn split_bins(input_list: &Vec<String>, position: usize) -> std::io::Result<(Vec<String>, Vec<String>)> {
    let mut with_one: Vec<String> = Vec::new();
    let mut with_zero: Vec<String> = Vec::new();
    for line in input_list {
        if line.chars().nth(position) == Some('1') {
            with_one.push(line.clone());
        } else {
            with_zero.push(line.clone());
        }
    }
    Ok((with_one, with_zero))
}


fn find_common_element(input_list: &Vec<String>, use_most: bool) -> std::io::Result<i32> {
    let mut active_list = input_list;
    let mut active_length = 1000;
    let mut one :Vec<String>;
    let mut zero:Vec<String>;

    let mut position :usize = 0;
    let mut use_one :bool;
    while active_length > 1 {
        (one, zero) = split_bins(active_list, position).expect("Can't handle");
        if use_most {
            use_one = one.len() >= zero.len();
        } else {
            use_one = one.len() < zero.len();
        }
        if use_one {
            active_list = &one;
        } else {
            active_list = &zero;
        }
        position += 1;
        active_length = active_list.len();
    }

    // let result :i32 = active_list[0].clone().parse().expect("Failed");
    let result = i32::from_str_radix(active_list[0].as_str(), 2).expect("Could not parse binary") ;
    //active_list[0].parse().expect("Failed");
    Ok(result)
}

pub fn day03_2(debug_level: u8, file_in: &str) -> std::io::Result<i64> {
    if debug_level > DEBUG_OFF {
        println!("Starting Day 03, Part 2")
    }
    let file = File::open(file_in).expect(&format!("Can't open {file_in}"));
    let lines = io::BufReader::new(file).lines();

    let root_list: Vec<String> = lines.map(|l| l.expect("Cannot parse line"))
        .collect();

    let o2_rating :i64 = find_common_element(&root_list, true).expect("O2 Failed") as i64;
    let co2_rating :i64 = find_common_element(&root_list, false).expect("CO2 Failed") as i64;

    if debug_level > DEBUG_OFF {
        println!("Result {o2_rating}, CO2={co2_rating} product = {}", o2_rating*co2_rating)
    }
    Ok(o2_rating*co2_rating)
}


fn read_binary_line(line: String, majority_values: [i32;12]) -> std::io::Result<[i32;12]> {
    // majority_values is the current list of which is top, 1 (positive value) or 0 (neg)
    let mut local_count = majority_values;
    for (i, bit) in line.chars().enumerate() {
        let is_one = match bit {
            '1' => 1,
            '0' => -1,
            _ => panic!("Illegal character in string {line}")
        };
        local_count[11-i] += is_one;    // Note that the order of the characters is reversed. Bit 0 is the low bit
    }
    Ok(local_count)
}


pub fn day03_1(debug_level: u8, file_in: &str) -> std::io::Result<i64> {
    if debug_level > DEBUG_OFF {
        println!("Starting Day 03, Part 1")
    }
    let file = File::open(file_in).expect(&format!("Can't open {file_in}"));
    let lines = io::BufReader::new(file).lines();

    let mut bit_count :[i32;12] = [0;12];
    for line in lines {
        bit_count = read_binary_line(line?, bit_count)?;
    }

    let mut gamma_rate :i64 = 0;
    let mut epsilon_rate :i64 = 0;
    for bit_val in bit_count.iter().rev() {
        if debug_level == DEBUG_ALL {
            println!("Bit Value = {bit_val}");
        }
        gamma_rate *= 2;
        epsilon_rate *= 2;
        if *bit_val == 0 {
            println!("Error, exact balance of 1s and 0s");
        } else if *bit_val > 0 {
            gamma_rate += 1;
        } else {
            epsilon_rate += 1;
        }
        if debug_level == DEBUG_ALL {
            println!("Gamma = {gamma_rate}, Epsilon = {epsilon_rate}");
        }
    }
    println!("gamma = {gamma_rate}, epsilon = {epsilon_rate}, product = {}", gamma_rate * epsilon_rate);
    Ok(gamma_rate * epsilon_rate)
}