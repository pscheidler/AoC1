/// Goal here is to use a bayesian estimation style algorithm to decode the mixed up signals
/// We have Timing signals with an unknown mapping to Display signals. Initial probability for the mapping is flat
/// We will get clusters of valid Timing signals, and we can map that to Display signals using their length
/// So if we know A,B is valid, we know either one can map to segment c, f (display of 1), so we
/// multiply the odds for c,d by 1 for A,B, all other odds by 0. We also do the inverse, on C,D,E,F,G, we
/// set the odds for c,d to 0.
/// If we know A,B,C is valid, they can map to a,c,f, so we multiply those odds by 1, all others by 0
/// But, C has already had the odds for c,d set to 0, so only a is 1. Ergo C => a
/// We can now clear a on every other segment
/// Starting over, if A,B,C,D,E is valid, that can either display 2 or 3 or 5
/// First, verify that a display is valid, eg 2 is valid if something can display "a"
/// Once the potentials are validated, adjust the odds using the valid potentials or'd together
/// So if 2 and 3 are the remaining potentials we adjust the odds using a,c,d,e,f,g, so we

/// Starting over, it looks like we can just use matrix math. If A, B can form a 1 (c, f), then we
/// get the prob matrix with [1, 1, 0, 0, 0, 0, 0] * T[0, 0, 1, 0, 0, 1, 0]
/// Then we add all the prob matrixes together, and pick the highest element in each row and column
static SEG_MAP: [[i32; 7]; 10] = [
//   a  b  c  d  e  f  g
//   8  6  8  7  4  9  7
    [1, 1, 1, 0, 1, 1, 1],  // 0, sum = 6
    [0, 0, 1, 0, 0, 1, 0],  // 1, sum = 2
    [1, 0, 1, 1, 1, 0, 1],  // 2, sum = 5
    [1, 0, 1, 1, 0, 1, 1],  // 3, sum = 5
    [0, 1, 1, 1, 0, 1, 0],  // 4, sum = 4
    [1, 1, 0, 1, 0, 1, 1],  // 5, sum = 5
    [1, 1, 0, 1, 1, 1, 1],  // 6, sum = 6
    [1, 0, 1, 0, 0, 1, 0],  // 7, sum = 3
    [1, 1, 1, 1, 1, 1, 1],  // 8, sum = 7
    [1, 1, 1, 1, 0, 1, 1]   // 9, sum = 6
];

struct SegmentPotential {
    segs: [i32; 7]
}

fn make_segment_potential() -> SegmentPotential {
    SegmentPotential {segs: [1; 7]}
}

impl SegmentPotential {

    fn check_self(&self) -> bool {
        let opt_left: i32 = self.segs.iter().sum();
        if opt_left == 1 {
            return true;
        } else if opt_left == 0 {
            panic!("No options left!");
        }
        false
    }
    fn set_option(&mut self, option: &[i32; 7]) -> bool {
        self.segs = self.segs.iter()
            .zip(option.iter())
            .map(|x, y| x*y)
            .collect();
        self.check_self()
    }

    fn set_many_options(&mut self, options: &Vec<[i32; 7]>) -> bool {
        let mut could_be: [i32; 7] = [0; 7];
        for opt in options.iter() {
            let temp = could_be.iter().zip(opt.iter())
                .map(|(a, b)| *a | *b)
                .collect();

        }

        self.check_self()
    }

    fn remove_option(&mut self, seg: usize) -> bool {
        self.segs[seg] = 0;
        self.check_self()
    }

    fn get_segment(&self) -> Option<usize> {
        if self.check_self() == false {
            return None;
        }
        let temp: usize = self.segs.iter().position(|&n| n == 1).unwrap();
        Some(temp)
    }
}