// Tough for day 1, at least part 2 is - see comment within the main implementation
// for more thoughts on that.
// My main issue was not reading isntructions - for some reason I thought part2 required
// concatenating *all* digits.
//
// Useful things I learnt today: the `String::char_indices()` method.

// Get the calibration value looking only for digits.
fn get_calibration_part1(line: &str) -> u32 {
    let digits = line
        .chars()
        // Nice shortcut here - to_digit() will return None if not a digit, which is
        // exactly what filter_map() needs to exclude from the resulting iterator.
        .filter_map(|c| c.to_digit(10))
        .collect::<Vec<_>>();
    calculate_calibration(&digits)
}

// Make a 2-digit number from first and last digit.  That's just 10*A + B.
fn calculate_calibration(digits: &[u32]) -> u32 {
    if digits.is_empty() {
        0
    } else {
        10 * (digits.first().unwrap()) + digits.last().unwrap()
    }
}

// There might be a crate somewhere for parsing digit names, but this is an easy enough
// alternative.
const DIGIT_NAMES: [(&str, u32); 9] = [
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

// Get the calibration value, looking for numeric and English-word digits.
fn get_calibration_part2(line: &str) -> u32 {
    let mut digits = vec![];
    for (idx, c) in line.char_indices() {
        if c.is_ascii_digit() {
            digits.push(c.to_digit(10).unwrap() as u32)
        } else {
            // I first thought there was inefficiency here - when we hit the match we could skip
            // over the next name.len() entries in the loop, and couldn't be bothered to enhance.
            // But that's wrong, as I found when reading the subreddit - there's a trap that completely
            // passed me by because of my basic implementation. Digit names can overlap,
            // as in "fiveight".  So we do need to check all characters.
            // There is an optimization in that we don't need to find every digit - we could just search
            // for the first match from each end, or only store 2 and overwrite the second each time we
            // get a subsequent match.  But getting the full set is fast enough.
            let substr = line.get(idx..).unwrap();
            for (name, val) in DIGIT_NAMES {
                if substr.starts_with(name) {
                    // A match - add to list.  No need to check any more possible names for this
                    // character.
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

    // Just need the sum of calibration values each time.
    let part1 = input.lines().map(get_calibration_part1).sum::<u32>();
    let part2 = input.lines().map(get_calibration_part2).sum::<u32>();

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}
