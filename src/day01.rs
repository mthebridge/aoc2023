// A bunch of optmiziations that could be made here if we cared...
//  - Don't need to collect the digits into vectors. Could do a
//  single pass over each line and only  keep first/last at the cost of some more code.
// - There's defintiely optimization in the string matching/searching!
fn get_calibration_part1(line: &str) -> u64 {
    let digits = line
        .chars()
        .filter(|&c| c.is_ascii_digit())
        .map(|c| c.to_digit(10).unwrap() as u64)
        .collect::<Vec<_>>();
    calculate_calibration(&digits)
}

// Make a 2-digit number from first and last digit.  That's just 10*A + B.
// Panics if empty.
fn calculate_calibration(digits: &[u64]) -> u64 {
    10 * (digits.first().expect("No digits!")) + digits.last().expect("No digits!")
}

// There might be a crate somewhere for parsing digit names, but this is an easy enough
// alternative.
const DIGIT_NAMES: [(&str, u64); 9] = [
    ("one", 1),
    ("two", 2),
    ("three", 3),
    ("four", 4),
    ("five", 5),
    ("six", 6),
    ("seven", 7),
    ("eight", 8),
    ("nine", 9),
];

fn get_calibration_part2(line: &str) -> u64 {
    let mut digits = vec![];
    for (idx, c) in line.char_indices() {
        if c.is_ascii_digit() {
            digits.push(c.to_digit(10).unwrap() as u64)
        } else {
            // There's inefficiency here - when we hit the match we could skip
            // over the next name_length entries in the loop.  But it's fast enough
            // as it is.
            let substr = line.get(idx..).unwrap();
            for (name, val) in DIGIT_NAMES {
                if substr.starts_with(name) {
                    // A match - add to list.  No need to check any more names.
                    digits.push(val);
                    break;
                }
            }
        }
    }
    calculate_calibration(&digits)
}

pub fn run(input_path: String) {
    let input = std::fs::read_to_string(input_path).unwrap();

    let part1 = input.lines().map(get_calibration_part1).sum::<u64>();
    let part2 = input.lines().map(get_calibration_part2).sum::<u64>();

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}
