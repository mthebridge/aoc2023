// Well that was easier!
// A bit of quick maths to spot that you're basically solving for:
// "find 0 < i < N such that i*(N-i) is greater than some target"
// This is symmetric, so you only need to find the first value.
// I wondered as I did part 1 if we might need to switch to binary search for
// part 2, but linear search is perfectly fast enough.
// My only slowdown was an off by one in my inequality, and then forgetting to use u64s and
// hitting wrapping for part 2.

// Calculate the disatnce for a given charge time.
// You travel (charge_time) millimeters for (total - charge) milliseconds.
fn calculate_distance(total_time: u64, charge_time: u64) -> u64 {
    (total_time - charge_time) * charge_time
}

// Parse out the numbers
fn parse_line_part1<'a>(line: &'a str) -> impl Iterator<Item = u64> + 'a {
    let (_, numbers) = line.split_once(':').unwrap();
    numbers
        .split_whitespace()
        .map(|s| s.parse::<u64>().unwrap())
}

// Parse out all the numbers into a single number.
fn parse_line_pt2<'a>(line: &'a str) -> u64 {
    let (_, numbers) = line.split_once(':').unwrap();
    let value = numbers
        .chars()
        .filter(|c| c.is_ascii_digit())
        .collect::<String>();
    value.parse::<u64>().unwrap()
}

// Count how many options would beat the target.
fn find_race_winning_options(time: u64, target_distance: u64) -> u64 {
    let min_charge = (0..time)
        .find(|&charge_time| calculate_distance(time, charge_time) > target_distance)
        .unwrap();
    // The distribution is symmetric, so anything up to (total - min) will also win.
    // (Since the formula is i*(N-i)).
    let max_charge = time - min_charge;
    // Inclusive range, so add 1.
    1 + max_charge - min_charge
}

pub fn run(input_path: String) {
    let input = std::fs::read_to_string(input_path).unwrap();
    let mut lines = input.lines();
    let times = lines.next().map(parse_line_part1).unwrap();
    let distances = lines.next().map(parse_line_part1).unwrap();
    let races = times.zip(distances);

    // Part 1 wants product of all potential win counts.
    let part1 = races
        .map(|(t, d)| find_race_winning_options(t, d))
        .product::<u64>();

    // Part 2 wants a single race win count.
    let mut lines = input.lines();
    let real_time = parse_line_pt2(lines.next().unwrap());
    let real_distance = parse_line_pt2(lines.next().unwrap());

    let part2 = find_race_winning_options(real_time, real_distance);

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}
