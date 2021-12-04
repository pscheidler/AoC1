use std::fs::File;
use std::io::{self, BufRead};

pub fn day01_2() -> std::io::Result<()> {
    println!("Starting!");

    let file = File::open("InputData_01.txt")?;
    let mut lines = io::BufReader::new(file).lines();
    let mut values_list :[i32; 3] = [0; 3];
    for index in 0..3 {
        let line :String = lines.next().unwrap()?;
        let reading :i32 = line.parse().unwrap();
        values_list[index] = reading;
    }
    let mut increase_count = 0;
    for line in lines {
        let previous_sum :i32 = values_list.iter().sum();
        let reading = line?;
        let value : i32 = reading.parse().unwrap();
        values_list = [value, values_list[0], values_list[1]];
        let new_sum :i32 = values_list.iter().sum();
        if previous_sum < new_sum {
            increase_count += 1;
        }
        // println!("{}", previous_sum);
    }
    println!("Got {} increases!", increase_count);
    Ok(())
}
