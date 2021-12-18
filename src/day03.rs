use std::fs::File;
use std::io::{self, BufRead, Lines, BufReader};
use std::convert::TryFrom;
// use crate::day03::bit_criteria::o2;

#[derive(PartialEq)]
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
            println!("1: {} ({})", value, one.len());
            one.push(*value);
        } else {
            println!("0: {}, ({})", value, zero.len());
            zero.push(*value);
        }
    }
    let keep_one :bool = one.len() > zero.len();
    let is_o2 :bool = kind == BitCriteria::o2;
    if keep_one ^ is_o2 {
        println!("Keeping the ones");
        return one;
    }
    println!("Keeping the zeros");
    return zero;
}

fn file_to_vec(input: &mut Lines<BufReader<File>>) -> Vec<u32> {
    let mut ret_val :Vec<u32> = Vec::new();
    for line in input {
        let line = line.unwrap();
        let parsed = u32::from_str_radix(line.as_str(), 2).unwrap();
        ret_val.push(parsed)
    }
    return ret_val;
}

fn get_value(file_name: std::str, kind: BitCriteria) -> std::io::Result<(i32)> {

}

pub fn part2() -> std::io::Result<()> {
    let file = File::open("InputData_03.txt")?;
    let mut lines = io::BufReader::new(file).lines();

    // let mut fpeek = lines.peekable();
    // let l1 = fpeek.peek();
    // let l1_str = l1.clone();
    // println!("L1 = {}", l1_str);
    // let l1_len = l1_str.len();
    // let bit_count :u8 = l1_len as u8;

    let bit_count = 12;
    println!("Lines are {} chars long", bit_count);

    let mut data_list = file_to_vec(&mut lines);

    for index in (0..(bit_count)).rev() {
        data_list = filter_list(&data_list, index, BitCriteria::o2);
        println!("Checked bit {}, {} items remain", index, data_list.len());
        if data_list.len() == 1 {
            println!("Got {}", data_list[0]);
        }
    }

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