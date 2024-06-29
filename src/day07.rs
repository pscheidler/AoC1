/// Goal is to take in an array of points (eg 16,1,2,0,4,2,7,1,2,14) and find the point they can travel
/// to with the lowest cost (2 in the previous example, which has a total cost of 37)
/// So Min(sum(abs(X - P) for X in Input)
/// It looks like this is just the Median? We could also find it with a gradient search

use std::fs::File;
use std::io::{self, BufRead}; //, Lines, BufReader
use crate::common::{DEBUG_OFF, DEBUG_MIN, DEBUG_ALL};
