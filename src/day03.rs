use std::fs::File;
use std::io::{self, BufRead, Lines, BufReader};
use std::convert::TryFrom;
use std::fmt::Error;
// use crate::day03::bit_criteria::o2;

#[derive(PartialEq)]
#[derive(Copy, Clone)]
enum BitCriteria {
    o2,
    co2,
}

fn filter_list(list_in: &Vec<u32>, index: u8, kind: BitCriteria) -> Vec<u32> {
    let mut one :Vec<u32> = Vec::new();
    let mut zero :Vec<u32> = Vec::new();

    let mask :u32 = 1 << index;

    for value in list_in {
        if (value & mask) > 0 {
            // println!("1: {} ({})", value, one.len());
            one.push(*value);
        } else {
            // println!("0: {}, ({})", value, zero.len());
            zero.push(*value);
        }
    }
    println!("1s = {}, 0s = {}", one.len(), zero.len());
    let keep_one :bool = one.len() >= zero.len();
    let is_co2 :bool = kind == BitCriteria::co2;
    if keep_one ^ is_co2 {
        println!("Keeping the ones");
        return one;
    }
    println!("Keeping the zeros");
    return zero;
}

fn file_to_vec(file_name: &str) -> std::io::Result<(Vec<u32>, i32)> {
    let file = File::open(file_name)?;
    let mut lines = io::BufReader::new(file).lines();
    let mut ret_val :Vec<u32> = Vec::new();
    let mut line_len :i32 = 0;
    for line in lines {
        let line = line.unwrap();
        let parsed = u32::from_str_radix(line.as_str(), 2).unwrap();
        line_len = line.len() as i32;
        ret_val.push(parsed)
    }
    Ok((ret_val, line_len))
}

fn get_value(file_name: &str, kind: BitCriteria) -> std::io::Result<i32> {
    let (mut data_list, bit_count) = file_to_vec(file_name)?;

    println!("Found length of {}", bit_count);

    for index in (0..(bit_count)).rev() {
        data_list = filter_list(&data_list, index as u8, kind);
        println!("Checked bit {}, {} items remain", index, data_list.len());
        if data_list.len() == 1 {
            println!("Got {}", data_list[0]);
            return Ok(data_list[0] as i32)
        }
    }
    panic!("Didn't find any value!")
}

pub fn part2() -> std::io::Result<()> {
    let input_file = "InputData_03.txt";
    let o2Value  = get_value(&input_file, BitCriteria::o2)?;
    let co2Value  = get_value(&input_file, BitCriteria::co2)?;
    let result = o2Value * co2Value;
    println!("Result is {}", result);
    Ok(())
}

fn read_binary_line(line: String, current_value: [i32;12]) -> std::io::Result<[i32;12]> {
    let mut local_count = current_value;
    for (i, bit) in line.chars().enumerate() {
        if bit == '1' {
            local_count[11-i] += 1;
        } else {
            local_count[11-i] -= 1;
        }
    }
    Ok(local_count)
}

pub fn part1() -> std::io::Result<()> {
    let file = File::open("InputData_03.txt")?;
    let lines = io::BufReader::new(file).lines();

    let mut bit_count :[i32;12] = [0;12];
    for line in lines {
        bit_count = read_binary_line(line?, bit_count)?;
    }
    let mut gamma_rate :i64 = 0;
    let mut epsilon_rate :i64 = 0;
    println!("2^4={}", i32::pow(2, 4));
    for (i, bit_val) in bit_count.iter().enumerate() {
        println!("{}, {}", bit_val, i);
        if *bit_val == 0 {
            println!("Error, exact balance of 1s and 0s in bit {}", i);
        } else if *bit_val > 0 {
            gamma_rate += i64::pow(2,   u32::try_from(i).unwrap());
        } else {
            epsilon_rate += i64::pow(2, u32::try_from(i).unwrap());
        }
        println!("{}, {}", gamma_rate, epsilon_rate);
    }
    println!("gamma = {}, epsilon = {}, product = {}", gamma_rate, epsilon_rate, gamma_rate * epsilon_rate);
    Ok(())
}