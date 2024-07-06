/// The plan is to read in the system settings (Display Signals) and apply rule based tests to see
/// what the mapping is. To make the system more flexible (and interesting), each test will be
/// in a separate function, and it will store the results in the object being tested.
///
/// We read in the Display Signals. We have a mapping of Timing Signals to numbers, and by looking
/// at what combinations of Timing Signals and Display Signals are valid, we can figure out how
/// the Display maps to Timing.
///
/// We have a struct for the Display, which lists the id (eg "A"), where it is used (eg if AB is
/// a valid combo, it lists [1, 1, 0, 0, 0, 0, 0], and what Timing Signal it may map to (if we
/// know it does could be anything but c, this would be [1, 1, 0, 1, 1, 1, 1].
///
/// 3 types of tests, Timing to Display, Display to Timing, and Already Selected
/// Timing to Display - For each timing group, gather all Display groups of the same length that could
/// fit it (meaning each Display Signal is covered by at least one Timing signal) Clear any remaining
/// Definitely Nots (a Timing is not used in any of the groups), set any Definitely Is (a Timing only
/// appears for one Display, in all options)
/// Display to Timing - Similar to Timing to Display, but flipped, so we gather all Timing options
/// for a given Display group
/// Already Selected - When we know a Display definitely corresponds to a given Timing, we clear that
/// Timing from all other Displays
/// The easy tests will look for unique lengths of Timing signals (eg 1, which is cf, the only combo
/// of length 2), and assign potential mappings based on that. Once a mapping is definitely known
/// that option will be removed for all other Display signals. Then, we need to go through and re-
/// evaluate if the potential mappings are still valid for a signal (eg, if a signal has a valid
/// combo of length 5, but that combo does not contain c, it must be 5
/// Keep going through the process until it is done, I guess

use std::iter::zip;

struct DisplayGroup {
    id: String,
    signals: [i8; 7],
    potential_timing: Vec<[i8; 7]>,
}

struct DisplaySignal {
    id: char,
    potential_timing_signals: [i8; 7],
    // where_used: Vec<[i8; 7]>,
    // known: bool,
}

fn make_display_signal(id: char) -> DisplaySignal {
    DisplaySignal {id, potential_timing_signals: [1, 1, 1, 1, 1, 1, 1] }
    // , where_used: Vec::new(), known: false}
}

fn make_display_group(id: String) -> DisplayGroup {
    let mut signals: [i8; 7] = [0;7];
    for letter in id.chars() {
        match letter {
            'a' => signals[0] = 1,
            'b' => signals[1] = 1,
            'c' => signals[2] = 1,
            'd' => signals[3] = 1,
            'e' => signals[4] = 1,
            'f' => signals[5] = 1,
            'g' => signals[6] = 1,
            _ => panic!("Got illegal letter"),
        }
    }
    DisplayGroup {id, signals, potential_timing: Vec::new()}
}

static TIME_SIG_TO_NUMBERS: [[i8; 7]; 10] = [
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

fn init_display_to_timing(display_input: &mut [DisplayGroup; 10], timing_input: &[[i8; 7];10]) {
    for disp in display_input {
        let active_let: i8 = disp.signals.iter().sum();
        for time in timing_input {
            if time.iter().sum::<i8>() == active_let {
                disp.potential_timing.push(*time);
            }
        }
    }
}

fn check_display(display_input: &DisplayGroup, disp_sigs: &mut [DisplaySignal; 7]) {
    let mut total_potentials: [i8; 7] = [0; 7];
    println!("Checking {}", display_input.id);
    let mut counter: i8 = 0;
    for time in display_input.potential_timing.iter() {
        println!("Count be {} {} {} {} {} {} {}", time[0], time[1], time[2], time[3], time[4], time[5], time[6]);
        counter += 1;
        for (index, letter) in time.iter().enumerate() {
            if *letter > 0 {
                total_potentials[index] = 1;
            }
        }
    }
    println!("Found {counter} matches, total pot = {} {} {} {} {} {} {}", total_potentials[0], total_potentials[1],
             total_potentials[2], total_potentials[3], total_potentials[4], total_potentials[5], total_potentials[6]);
    let match_len = display_input.potential_timing.len();
    // TODO: Check that some of the signals in display cover the signals in time
    for (active, signal) in zip(display_input.signals, disp_sigs) {
        if active == 0 {
            if match_len == 1 {
                print!("Testing off {}", signal.id);
                for (disp_pot, letter_pot) in zip(total_potentials, &mut signal.potential_timing_signals) {
                    if disp_pot == 1 {
                        *letter_pot = 0;
                    }
                }
            }
        } else {
            print!("Testing act {}", signal.id);
            for (disp_pot, letter_pot) in zip(total_potentials, &mut signal.potential_timing_signals) {
                if disp_pot == 0 {
                    *letter_pot = 0;
                }
            }
        }
        println!(" Pot is now {} {} {} {} {} {} {}", signal.potential_timing_signals[0], signal.potential_timing_signals[1],
                 signal.potential_timing_signals[2], signal.potential_timing_signals[3], signal.potential_timing_signals[4],
                 signal.potential_timing_signals[5], signal.potential_timing_signals[6]);
    }
}

struct SegmentPotential {
    segs: [i32; 7]
}

fn make_segment_potential() -> SegmentPotential {
    SegmentPotential {segs: [1; 7]}
}

// impl SegmentPotential {
//
//     fn check_self(&self) -> bool {
//         let opt_left: i32 = self.segs.iter().sum();
//         if opt_left == 1 {
//             return true;
//         } else if opt_left == 0 {
//             panic!("No options left!");
//         }
//         false
//     }
//     fn set_option(&mut self, option: &[i32; 7]) -> bool {
//         self.segs = self.segs.iter()
//             .zip(option.iter())
//             .map(|x, y| x*y)
//             .collect();
//         self.check_self()
//     }
//
//     fn set_many_options(&mut self, options: &Vec<[i32; 7]>) -> bool {
//         let mut could_be: [i32; 7] = [0; 7];
//         for opt in options.iter() {
//             let temp = could_be.iter().zip(opt.iter())
//                 .map(|(a, b)| *a | *b)
//                 .collect();
//
//         }
//
//         self.check_self()
//     }
//
//     fn remove_option(&mut self, seg: usize) -> bool {
//         self.segs[seg] = 0;
//         self.check_self()
//     }
//
//     fn get_segment(&self) -> Option<usize> {
//         if self.check_self() == false {
//             return None;
//         }
//         let temp: usize = self.segs.iter().position(|&n| n == 1).unwrap();
//         Some(temp)
//     }
// }

pub fn day08_1() {
    // let mut d0 = make_display_group("cdeba".to_string());
    // let mut d1 = make_display_group("gadec".to_string());
    // let mut d2 = make_display_group("dafcgb".to_string());
    // let mut d3 = make_display_group("bd".to_string());
    // let mut d4 = make_display_group("cdb".to_string());
    // let mut d5 = make_display_group("cdebga".to_string());
    // let mut d6 = make_display_group("fadecg".to_string());
    // let mut d7 = make_display_group("adcfbeg".to_string());
    // let mut d8 = make_display_group("bacfe".to_string());
    // let mut d9 = make_display_group("bedg".to_string());

    let mut test_input: [DisplayGroup; 10] = [
        make_display_group("cdeba".to_string()),
        make_display_group("gadec".to_string()),
        make_display_group("dafcgb".to_string()),
        make_display_group("bd".to_string()),
        make_display_group("cdb".to_string()),
        make_display_group("cdebga".to_string()),
        make_display_group("fadecg".to_string()),
        make_display_group("adcfbeg".to_string()),
        make_display_group("bacfe".to_string()),
        make_display_group("bedg".to_string()),
    ];

    // let mut la: DisplaySignal = make_display_signal('a');
    // let mut lb: DisplaySignal = make_display_signal('b');
    // let mut lc: DisplaySignal = make_display_signal('c');
    // let mut ld: DisplaySignal = make_display_signal('d');
    // let mut le: DisplaySignal = make_display_signal('e');
    // let mut lf: DisplaySignal = make_display_signal('f');
    // let mut lg: DisplaySignal = make_display_signal('g');

    let mut disp_sigs: [DisplaySignal; 7] = [
        make_display_signal('a'),
        make_display_signal('b'),
        make_display_signal('c'),
        make_display_signal('d'),
        make_display_signal('e'),
        make_display_signal('f'),
        make_display_signal('g'),
    ];
    init_display_to_timing(&mut test_input, &TIME_SIG_TO_NUMBERS);

    for i in 1..3 {
        for disp_group in test_input.iter_mut() {
            check_display(disp_group, &mut disp_sigs);
        }
    }
}

