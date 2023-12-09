// This was a pleasant surprise!  Very fast today.
// There might be some clever maths here, but just following the basic approach
// completes in < 1 millisecond.
// Part 2 was a pleasingly simple extension as well.

// Extrapolate the sequence out in both directions.
fn extrapolate_sequence(seq: Vec<i32>) -> (i32, i32) {
    // Step 1 - calculate the difference between each consecutive item.
    // Then repeat on *that* sequence, until we get all zeroes.
    let mut hists = vec![seq];
    loop {
        // Get the differences for the most recent history
        let latest = hists.last().unwrap();
        let diffs = latest
            .windows(2)
            .map(|set| set[1] - set[0])
            .collect::<Vec<_>>();

        if diffs.iter().all(|&d| d == 0) {
            break;
        }
        hists.push(diffs);
    }

    // Now iterate backwards over the history, and store the extra first and last element in each sequence
    // by respectively subtracting and adding the previous difference to the first and last
    // element.
    //
    // We end up with the extra first and last element of the original sequence, whcih is what we want
    // to return.
    hists
        .iter()
        .rev()
        .fold((0, 0), |(first_diff, last_diff), this| {
            (
                this.first().unwrap() - first_diff,
                this.last().unwrap() + last_diff,
            )
        })
}

pub fn run(input_path: String) {
    let input = std::fs::read_to_string(input_path).unwrap();

    // Parse each line into a sequence and calculate its extrapolated
    // first and last.
    // Input has negative numbers, so use signed integers!
    let (first_sum, lasts_sum) = input
        .lines()
        .map(|line| {
            let sequence = line
                .split_whitespace()
                .map(|word| word.parse::<i32>().unwrap())
                .collect::<Vec<_>>();
            extrapolate_sequence(sequence)
        })
        // We want the separate sums of all the firsts and all the lasts.
        .fold((0, 0), |total, next| (total.0 + next.0, total.1 + next.1));

    println!("Part 1: {}", lasts_sum);
    println!("Part 2: {}", first_sum);
}
