// Template.
// Copy to daynum.rs, and uncomment relevant lines in main to add

use std::collections::HashMap;

type Cache<'a> = HashMap<(&'a [char], &'a [u8]), usize>;

// Work out how many possible runs we could have givemn this input.
fn calculate_possibles(row: &[char], lengths: &[u8], cache: &mut Cache) -> usize {

    if let Some(res) = cache.get(&(row, lengths)) {
        return *res
    }
    // Check we have space for all the remaining runs.  Need all their sizes,
    // + 1 for each run after this.
    let space_needed = lengths.iter().sum::<u8>() as usize + lengths.len() - 1;

    let mut possibles = 0;
    let this_run_len = lengths[0] as usize;

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
                // This is the final run, so a valid option - if there are not more `#`
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
                possibles += calculate_possibles(&row[new_run_start..], &lengths[1..], cache)
            }
        }
    }
    cache.insert((row, lengths), possibles);
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

    let mut cache: Cache = HashMap::new();

    let part1 = records.clone()
        .map(|(row, lengths)| {
            let res = calculate_possibles(&row, &lengths);
            // println!("{} {:?}: {}", row.iter().collect::<String>(), &lengths, res);
            res
        })
        .sum::<usize>();


    println!("Part 1: {}", part1);

    let part2 = records
    .enumerate()
    .map(|(i, (row, lengths))| {
        let mut full_row = row.clone();
        let mut full_lengths = lengths.clone();
        // Add 4 copies.
        for _ in 0..4 {
            full_row.push('?');
            full_row.append(&mut row.clone());
            full_lengths.append(&mut lengths.clone());
        }
        let res = calculate_possibles(&full_row, &full_lengths);
        println!("Entry {i} has {res} options");
        res
    })
    .sum::<usize>();
    println!("Part 2: {}", part2);
}
