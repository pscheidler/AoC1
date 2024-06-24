use std::fs::File;
use std::cmp::{min, max};
use std::io::{self, BufRead}; //, Lines, BufReader
use crate::common::{DEBUG_OFF, DEBUG_MIN, DEBUG_ALL};
use crate::day05::Direction::{Diagonal, Horizontal, Unknown, Vertical};

#[derive(Debug)]
enum Direction {
    Horizontal,
    Vertical,
    Diagonal,
    Unknown,
}

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32
}
#[derive(Debug)]
struct VentLine {
    start: Point,
    end: Point,
    direction: Direction,
}

trait VentLineBase {

}
impl VentLine {
    fn set_dircetion(&mut self) {
        if self.start.y == self.end.y {
            self.direction = Horizontal;
        } else if self.start.x == self.end.x {
            self.direction = Vertical;
        } else {
            self.direction = Diagonal;
        }
    }

    /// Make sure the line's start x < end x (or start y < end y for vertical lines only), and set the direction
    fn update(&mut self) {
        self.set_dircetion();
        match self.direction {
            Vertical => {
                if self.start.y > self.end.y {
                    (self.start.y, self.end.y) = (self.end.y, self.start.y);
                    // let temp = self.start.y;
                    // self.start.y = self.end.y;
                    // self.end.y = temp;
                }
            }
            _ => {
                if self.start.x > self.end.y {
                    (self.start, self.end) = (self.end, self.start);
                }
            }
        }
    }

    /// Find the intersection of 2 parallel lines, horizontal or vertical
    /// The parallel axis must already have been checked, this just looks at the moving axis
    /// We assume start < end
    fn intersect_parallel(&self, my_start: i32, my_end: i32, other_start: i32, other_end: i32) -> Option<Vec<i32>> {
        let start = max(my_start, other_start);
        let end = min(my_end, other_end);
        if end < start {
            return None;
        }
        let return_vector: Vec<i32> = (start..=end).collect();
        Some(return_vector)
    }

    fn intersects(&self, other: &VentLine) -> Option<Vec<Point>> {
        match (self.direction, other.direction) {

        }
        None
    }
}

/// separate X,Y string into 2 points
fn process_point(point: &String) -> Point {
    let mut point_parts = point.split(",");
    let x: i32 = point_parts.next().expect("Couldn't process X point")
        .parse().expect("Couldn't parse X point");
    let y: i32 = point_parts.next().expect("Couldn't process Y point")
        .parse().expect("Couldn't parse Y point");
    Point {
        x, y
    }
}

/// Process a line of format X,Y -> X,Y to turn it into the line format
fn process_line(line: &String) -> VentLine {
    let mut halves = line.split(" -> ");
    let start = process_point(&String::from(halves.next().expect("Couldn't get start point")));
    let end   = process_point(&String::from(halves.next().expect("Couldn't get end point")));
    let mut return_line = VentLine {
        start,
        end,
        direction: Unknown
    };
    return_line.set_dircetion();
    return_line
}

pub fn day05_1(debug_level: u8, file_in: &str) -> std::io::Result<i32> {
    if debug_level > DEBUG_OFF {
        println!("Starting Day 05, Part 1")
    }
    let file = File::open(file_in).expect(&format!("Can't open {file_in}"));
    let mut lines = io::BufReader::new(file).lines();
    let mut line_list: Vec<VentLine> = Vec::new();

    for line_result in lines {
        let line = line_result.expect("Could not process line");
        line_list.push(process_line(&line));
        dbg!(line_list.last());
    }
    Ok(0)
}