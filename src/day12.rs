// Template.
// Copy to daynum.rs, and uncomment relevant lines in main to add

// Work out how many possible runs we could have givemn this input.
fn calculate_possibles(row: &[char], lengths: &[u8]) -> usize {
    // Check we have space for all the remaining runs.  Need all their sizes,
    // + 1 for each run after this.
    let space_needed = lengths.iter().sum::<u8>() as usize + lengths.len() - 1;

    let mut possibles = 0;
    let this_run_len = lengths[0] as usize;

    // dbg!(row, lengths, runs_after_this, space_needed);
    // We can potentially start a run from anywhere here on as long as we have enough space.
    for this_run_start in 0..=row.len() - space_needed {
        // If the previous character was a fixed #, we can't start here (or anywhere after this!)
        // Break out now.
        if this_run_start > 0 && row[this_run_start - 1] == '#' {
            break;
        }
        // The next run could start here if all the following hold:
        // The next run-length fields are not empty (.)
        // The single field afterwards is not full (#), or we're at the end of the row.
        if row[this_run_start..this_run_start + this_run_len]
            .iter()
            .all(|&r| r != '.')
            && (this_run_start + this_run_len == row.len()
                || row[this_run_start + this_run_len] != '#')
        {
            if lengths.len() == 1 {
                // This is the final run, so a valid option - if there are not more `']`
                // println!("Valid final run at {this_run_start}");
                if row[this_run_start + this_run_len..]
                    .iter()
                    .all(|&r| r != '#')
                {
                    possibles += 1;
                }
            } else {
                // This could be a valid option.  Check positions for the remaining runs.
                // Skip 1 further off the end to account for the space.
                let new_run_start = this_run_start + this_run_len + 1;
                // println!("Consider next run starting at {new_run_start}");
                possibles += calculate_possibles(&row[new_run_start..], &lengths[1..])
            }
        }
    }
    // dbg!(row.len(), possibles);
    possibles
}

pub fn run(input_path: String) {
    let input = std::fs::read_to_string(input_path).unwrap();
    let records = input.lines().map(|line| {
        let (pattern, lengths) = line.split_once(' ').unwrap();
        (
            pattern.chars().collect::<Vec<_>>(),
            lengths
                .split(',')
                .map(|s| s.parse::<u8>().unwrap())
                .collect::<Vec<_>>(),
        )
    });

    let part1 = records
        .map(|(row, lengths)| {
            let res = calculate_possibles(&row, &lengths);
            // println!("{} {:?}: {}", row.iter().collect::<String>(), &lengths, res);
            res
        })
        .sum::<usize>();

    let part2 = 0;

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}
