use std::fs::File;
use std::cmp::{min, max};
use std::io::{self, BufRead}; //, Lines, BufReader
use crate::common::{DEBUG_OFF, DEBUG_MIN, DEBUG_ALL};
use crate::day01::day01_1;
use crate::day05::GenericLine::{Vertical, Unknown, Horizontal, Diagonal};

#[derive(Debug, Copy, Clone)]
struct Point {
    x: i32,
    y: i32
}

#[derive(Debug, Copy, Clone)]
struct VerticalLine {
    x_offset: i32,
    y_start: i32,
    y_end: i32,
}

fn make_vertical_line(start: Point, end: Point) -> VerticalLine {
    if start.x != end.x {
        panic!("Bad call to make vertical line!")
    }
    let y_start: i32 = min(start.y, end.y);
    let y_end: i32 = max(start.y, end.y);
    VerticalLine{
        x_offset: start.x,
        y_start, y_end,
    }
}

#[derive(Debug, Copy, Clone)]
struct RegularLine {
    offset: i32,
    slope: i32,
    x_start: i32,
    x_end: i32,
}

fn make_regular_line(start: Point, end: Point) -> RegularLine {
    if start.x == end.x {
        panic!("Called make_regular_line for a vertical line!")
    }
    let slope: i32 = (start.y - end.y) / (start.x - end.x);
    let offset:i32 = start.y - start.x * slope;
    let x_start: i32 = min(start.x, end.x);
    let x_end: i32 = max(start.x, end.x);
    RegularLine {
        offset, slope, x_start, x_end,
    }
}

impl RegularLine {
    fn get_regular_intersect(&self, other: &RegularLine) -> Option<Vec<Point>> {
        // This gets the X values where the 2 lines overlap
        let overlap_start: i32 = max(self.x_start, other.x_start);
        let overlap_end: i32 = min(self.x_end, other.x_end);
        // If end < start, then there is no overlap after all
        if overlap_end < overlap_start {
            return None;
        }
        if other.slope == self.slope {
            // lines are parallel, if they don't have the same offset, they don't intersect
            if other.offset != self.offset {
                return None;
            }
            // Lines are parallel and overlap, so the intersection is where the X values overlap
            let mut return_vec: Vec<Point> = Vec::new();
            for x in overlap_start..=overlap_end {
                let y: i32 = self.slope * x + self.offset;
                return_vec.push(Point{x, y});
            }
            return Some(return_vec);
        } else {
            // Lines aren't parallel, so find where they would intersect, and then make sure that
            // point exists for both lines
            // We're trying to find X where S1 * X + Off1 = S2 * X + Off2, so X = (Off2 - Off1) / (S1 - S2)
            let x: i32 = (self.offset - other.offset) / (other.slope - self.slope);
            if x < overlap_start || x > overlap_end {
                return None;
            }
            let y: i32 = self.offset + self.slope * x;
            let mut return_vec: Vec<Point> = Vec::new();
            return_vec.push(Point {x, y});
            Some(return_vec)
        }
    }

    fn get_vertical_intersect(&self, other: &VerticalLine) -> Option<Vec<Point>> {
        if other.x_offset < self.x_start || other.x_offset > self.x_end {
            // Does not overlap in x
            return None;
        }
        let y: i32 = self.slope * other.x_offset + self.offset;
        if other.y_start <= y && y <= other.y_end {
            let mut return_vec: Vec<Point> = Vec::new();
            return_vec.push(Point {x: other.x_offset, y});
            return Some(return_vec);
        }
        None
    }

    fn iter_line_list(&self, line_list: &Vec<GenericLine>) -> Option<Vec<Point>> {
        let mut return_vec: Vec<Point> = Vec::new();
        for old_vent_line in line_list.iter() {
            let result = match old_vent_line {
                Vertical(l2) => self.get_vertical_intersect(l2),
                Horizontal(l2) | Diagonal(l2) => self.get_regular_intersect(l2),
                _ => panic!("Illegal line type")
            };
            match result {
                Some(value) => {
                    let intersect_count = value.iter().count();
                    // println!("Found {intersect_count} hits between {:?} and {:?}", self, old_vent_line);
                    // println!("See {:?} (Reg)", value.first());
                    for point in value.iter() {
                        return_vec.push(*point);
                    }
                }
                None => (),
            }
        }
        if return_vec.len() == 0 {
            return None;
        }
        Some(return_vec)
    }
}

impl VerticalLine {
    fn get_regular_intersect(&self, other: &RegularLine) -> Option<Vec<Point>> {
        other.get_vertical_intersect(self)
    }

    fn get_vertical_intersect(&self, other: &VerticalLine) -> Option<Vec<Point>> {
        if other.x_offset != self.x_offset {
            // Does not overlap in x
            return None;
        }
        let overlap_start: i32 = max(self.y_start, other.y_start);
        let overlap_end: i32 = min(self.y_end, other.y_end);
        if overlap_end < overlap_start {
            return None;
        }
        let mut return_vec: Vec<Point> = Vec::new();
        for y in overlap_start..=overlap_end {
            return_vec.push(Point{x: self.x_offset, y});
        }
        Some(return_vec)
    }

    fn iter_line_list(&self, line_list: &Vec<GenericLine>) -> Option<Vec<Point>> {
        let mut return_vec: Vec<Point> = Vec::new();
        for old_vent_line in line_list.iter() {
            let result = match old_vent_line {
                Vertical(l2) => self.get_vertical_intersect(l2),
                Horizontal(l2) | Diagonal(l2) => self.get_regular_intersect(l2),
                _ => panic!("Illegal line type")
            };
            match result {
                Some(value) => {
                    let intersect_count = value.iter().count();
                    // println!("Found {intersect_count} hits between {:?} and {:?}", self, old_vent_line);
                    // println!("See {:?}", value.first());
                    for point in value.iter() {
                        return_vec.push(*point);
                    }
                }
                None => (),
            }
        }
        if return_vec.len() == 0 {
            return None;
        }
        Some(return_vec)
    }

}

#[derive(Debug, Copy, Clone)]
enum GenericLine {
    Vertical(VerticalLine),
    Horizontal(RegularLine),
    Diagonal(RegularLine),
    Unknown,
}

struct Intersection {
    simple_point: i32,
    count: u8,
}

struct IntersectionHolder {
    intersections: Vec<i32>,
}

fn make_intersection_holder() -> IntersectionHolder {
    IntersectionHolder {intersections: Vec::new()}
}

fn simplify_point(point: &Point) -> i32 {
    point.x * 1000 + point.y
}

impl IntersectionHolder {
    // fn find_intersection(&mut self, point: &Point) -> Option<&mut Intersection> {
    //     let my_simple_point: i32 = simplify_point(point);
    //     for int in self.intersections.iter_mut() {
    //         if int.simple_point == my_simple_point {
    //             return Some(int);
    //         }
    //     }
    //     None
    // }

    fn add_points(&mut self, points: &mut Option<Vec<Point>>) {
        match points {
            None => return,
            Some(point_list) => {
                // We found a list of intersections, if it already exists in our list we should increment, otherwise add it
                for point in point_list.iter() {
                    let simple_point = simplify_point(point);
                    if self.intersections.contains(&simple_point) == false {
                        self.intersections.push(simple_point);
                    }
                }
                //     match self.find_intersection(point) {
                //         None => {
                //             // Intersection is new so we add it in
                //             println!("Adding {:?}", point);
                //             let simple_point: i32 = simplify_point(point);
                //             self.intersections.push(Intersection { simple_point, count: 1 })
                //         },
                //         Some(int) => {
                //             // println!("incrementing {:?}", point);
                //             int.count += 1
                //         },
                //     }
                // }
            }
        }
    }

    fn get_intersection_count(&self) -> i32 {
        let mut counter: i32 = 0;
        let counted = self.intersections.iter().count();
        // The code below shows how to count for more than 1 intersection only
        // println!("Intersections count = {counted}");
        // for int in self.intersections.iter() {
        //     if int.count > 0 {
        //         counter += 1;
        //     }
        // }
        i32::try_from(counted).expect("Could not convert usize")
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
fn process_line(line: &String) -> GenericLine {
    let mut halves = line.split(" -> ");
    let start = process_point(&String::from(halves.next().expect("Couldn't get start point")));
    let end   = process_point(&String::from(halves.next().expect("Couldn't get end point")));
    if start.x == end.x {
        Vertical(make_vertical_line(start, end))
    } else {
        let new_line = make_regular_line(start, end);
        if new_line.slope == 0 {
            Horizontal(new_line)
        } else {
            Diagonal(new_line)
        }
    }
}

fn day05(debug_level: u8, file_in: &str, use_diags: bool) -> std::io::Result<i32> {
    if debug_level > DEBUG_OFF {
        println!("Starting Day 05, Part 1")
    }
    let file = File::open(file_in).expect(&format!("Can't open {file_in}"));
    let mut lines = io::BufReader::new(file).lines();
    let mut line_list: Vec<GenericLine> = Vec::new();
    let mut intersect_holder: IntersectionHolder = make_intersection_holder();

    for line_result in lines {
        let line = line_result.expect("Could not process line");
        let new_vent_line = process_line(&line);
        match new_vent_line {
            // Vertical(l1) => l1.iter_line_list(&line_list),
            // Horizontal(l1) => l1.iter_line_list(&line_list),
            Vertical(l1) => {
                let mut new_points = l1.iter_line_list(&line_list);
                intersect_holder.add_points(&mut new_points);
                line_list.push(new_vent_line);
            },
            Horizontal(l1) => {
                let mut new_points = l1.iter_line_list(&line_list);
                intersect_holder.add_points(&mut new_points);
                line_list.push(new_vent_line);
            },
            Diagonal(l1) => if use_diags {
                let mut new_points = l1.iter_line_list(&line_list);
                intersect_holder.add_points(&mut new_points);
                line_list.push(new_vent_line);
            } else {
                ()
            },
            _ => panic!("Illegal Line Type"),
        };
        // println!("Current count is {}", intersect_holder.get_intersection_count());
        // intersect_holder.error_check();
    }
    let final_count: i32 = intersect_holder.get_intersection_count();
    println!("Counted {final_count}");
    Ok(intersect_holder.get_intersection_count())
}

pub fn day05_1(debug_level: u8, file_in: &str) -> std::io::Result<i32> {
    day05(debug_level, file_in, false)
}

pub fn day05_2(debug_level: u8, file_in: &str) -> std::io::Result<i32> {
    day05(debug_level, file_in, true)
}
