use std::fs::File;
use std::io::{self, BufRead}; //, Lines, BufReader
use crate::common::{DEBUG_OFF, DEBUG_MIN, DEBUG_ALL};
use crate::day04::NumberStatus::{Selected, Unselected};

#[derive(Copy, Clone)]
#[derive(Debug)]
enum NumberStatus {
    Unselected(i32),
    Selected(i32)
}

#[derive(Copy, Clone)]
#[derive(Debug)]
struct NumberSpot {
    board: i32,
    row: i32,
    col: i32,
}

struct Board {
    numbers: [[NumberStatus; 5]; 5],
    won: bool,
}

impl Board {
    fn set_row_from_nums(&mut self, row_index: usize, row: &Vec<i32>) {
        if row_index >= 5 || row_index < 0 {
            panic!("Got illegal row index of {row_index}");
        }
        if row.len() < 5 {
            panic!("Trying to set Row without enough elements");
        }

        let vec1 = row[..5].iter()
            .map(|x| Unselected(*x))
            .collect::<Vec<NumberStatus>>();
        let vec2: [NumberStatus; 5] = vec1.try_into().expect("Can't");
        self.numbers[row_index] = vec2;
    }

    fn set_row_from_string(&mut self, row_index: usize, row: String) -> Vec<i32> {
        let row_nums = row.split_whitespace()
            .map(|x| x.parse::<i32>().expect("Couldn't Parse"))
            .collect::<Vec<i32>>();
        self.set_row_from_nums(row_index, &row_nums);
        row_nums
    }

    fn select_carefully(&mut self, row: i32, col: i32, number: i32) {
        if self.won {
            return;
        }
        match self.numbers[row as usize][col as usize] {
            Unselected(this_number) => {
                if this_number != number {
                    panic!("Number is not what we selected")
                }
                self.numbers[row as usize][col as usize] = Selected(number)
            },
            _ => panic!("Problem with setting a number"),
        }
    }

    fn check_row(&self, row: usize) -> bool {
        for number in self.numbers[row] {
            if let Selected(_) = number {
            } else {
                return false
            }
        }
        true
    }

    fn check_col(&self, col: usize) -> bool {
        for row in self.numbers {
            if let Selected(_) = row[col] {
            } else {
                return false
            }
        }
        true
    }

    fn check_update(&mut self, row: i32, col:i32) -> bool {
        if self.won {
            return false;
        }
        // let v1 = self.check_row(row as usize);
        // let v2 = self.check_col(col as usize);
        if self.check_row(row as usize) || self.check_col(col as usize) {
            self.won = true;
            return true;
        }
        return false;
    }

    fn get_unselected_sum(&self) -> i32 {
        let mut sum: i32 = 0;
        for row in self.numbers {
            for number in row {
                match number {
                    Unselected(val) => sum += val,
                    _ => {},
                }
            }
        }
        sum
    }
}

fn is_blank(line: &String) -> bool {
    if line.trim().len() < 1 {
        return true;
    }
    false
}

fn day04_setup(file_in: &str) -> (Vec<Board>, [Vec<NumberSpot>; 100], Vec<i32>) {
    let file = File::open(file_in).expect(&format!("Can't open {file_in}"));
    let mut lines = io::BufReader::new(file).lines();
    let selection_string = lines.next().expect("Couldn't access first line")
        .expect("First line is empty");

    lines.next();
    let mut board_list: Vec<Board> = Vec::new();
    const select_helper_init: Vec<NumberSpot> = Vec::new();
    let mut select_helper: [Vec<NumberSpot>; 100] = [select_helper_init; 100];
    const init_value: NumberStatus = Unselected(0);
    {
        let board = Board {
            numbers: [[init_value; 5]; 5],
            won: false,
        };
        board_list.push(board);
    }
    let mut row_index :usize = 0;
    for line_result in lines {
        let line = line_result.expect("Problem accessing line");
        if is_blank(&line) {
            row_index = 0;
            let board = Board {
                numbers: [[init_value; 5]; 5],
                won: false,
            };
            board_list.push(board);
            continue;
        }
        let mut active_board = board_list.last_mut().expect("Could not access");
        let numbers = active_board.set_row_from_string(row_index, line);
        for (index, number) in numbers.iter().enumerate() {
            select_helper[*number as usize].push(NumberSpot {
                board: (board_list.len() - 1) as i32,
                row: row_index as i32,
                col: index as i32,
            });
        }
        row_index += 1;
    }
    let selection_list: Vec<i32> = selection_string.split(",")
        .map(|x| x.parse::<i32>().expect("Couldn't parse selection"))
        .collect();
    return (board_list, select_helper, selection_list) ;
}

pub fn day04_1(debug_level: u8, file_in: &str) -> std::io::Result<i32> {
    if debug_level > DEBUG_OFF {
        println!("Starting Day 04, Part 1")
    }

    let (mut board_list, select_helper, selection_list) = day04_setup(file_in);

    for selection in selection_list.iter() {
        for sel_spots in select_helper[*selection as usize].iter() {
            board_list[sel_spots.board as usize].select_carefully(sel_spots.row, sel_spots.col, *selection);
            let done = board_list[sel_spots.board as usize].check_update(sel_spots.row, sel_spots.col);
            if done {
                let unselected_sum = board_list[sel_spots.board as usize].get_unselected_sum();
                let return_value = unselected_sum * (*selection);
                println!("Number called is {}, sum is {unselected_sum}, return is {return_value}", *selection);
                return Ok(return_value);
            }
        }
    }
    // Something failed
    Ok(-1)
}

pub fn day04_2(debug_level: u8, file_in: &str) -> std::io::Result<i32> {
    if debug_level > DEBUG_OFF {
        println!("Starting Day 04, Part 2")
    }

    let (mut board_list, select_helper, selection_list) = day04_setup(file_in);

    let mut last_board: i32 = 0;
    for selection in selection_list.iter() {
        for sel_spots in select_helper[*selection as usize].iter() {
            board_list[sel_spots.board as usize].select_carefully(sel_spots.row, sel_spots.col, *selection);
            let done = board_list[sel_spots.board as usize].check_update(sel_spots.row, sel_spots.col);
            if done {
                let unselected_sum = board_list[sel_spots.board as usize].get_unselected_sum();
                last_board = unselected_sum * (*selection);
                println!("Number {} wins on board {} with {last_board}", *selection, sel_spots.board);
            }
        }
    }
    println!("Last board score is {last_board}");
    Ok(last_board)
}