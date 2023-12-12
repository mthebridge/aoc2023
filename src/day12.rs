// Part 1 took ages because I missed some key edge cases that only showed up in the real inoput.
// Sort of brute force, although with more logic to spot when we reach an invalid poisiton than
//perhaps we need.
//
// Was worried about part2 until I saw some rumblings about memoization on reddit, and indeed
// throwing in a cache brings the runtime down to a fraction of a second, even with
// reallocating all the time.

use std::collections::HashMap;

// Cache results for performance.
// Hit and miss counts are just for interest/debugging.
#[derive(Debug, Clone, Default)]
struct Cache {
    cache: HashMap<(Vec<char>, Vec<u8>), usize>,
    hits: u64,
    misses: u64,
}

// Work out how many possible runs we could have given this input.
// We look at the next spring, and consider all possible positions that could start given the
// other constraints; then recurse from the point after that with the remaining springs.
// The cache is essential for part 2 to complete in reasonable time, as we can end up
// with billions of possibilities - but many of the later possibilities repeat themselves
// across rows.
fn calculate_possibles(row: &[char], lengths: &[u8], cache: &mut Cache) -> usize {
    if let Some(res) = cache.cache.get(&(row.to_vec(), lengths.to_vec())) {
        cache.hits += 1;
        return *res;
    }
    // Check we have space for all the remaining runs.  Need all their sizes,
    // + 1 for each run after this for the space.
    let space_needed = lengths.iter().sum::<u8>() as usize + lengths.len() - 1;

    // Return count.
    let mut possibles = 0;

    // We can potentially start a run from anywhere here onward as long as we have enough space.
    let this_run_len = lengths[0] as usize;
    for this_run_start in 0..=row.len() - space_needed {
        // If the previous character was a fixed #, we can't start the run here
        // (or anywhere after this!). Break out now.
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

    cache
        .cache
        .insert((row.to_vec(), lengths.to_vec()), possibles);
    cache.misses += 1;
    possibles
}

pub fn run(input_path: String) {
    let input = std::fs::read_to_string(input_path).unwrap();
    let records = input.lines().map(|line| {
        // Parse inputs. We want a set of chars, and a set of integers.
        let (pattern, lengths) = line.split_once(' ').unwrap();
        (
            pattern.chars().collect::<Vec<_>>(),
            lengths
                .split(',')
                .map(|s| s.parse::<u8>().unwrap())
                .collect::<Vec<_>>(),
        )
    });

    let mut cache: Cache = Default::default();
    let mut part1 = 0;

    for (row, lengths) in records.clone() {
        part1 += calculate_possibles(&row, &lengths, &mut cache);
    }

    println!("Part 1: {}", part1);
    println!("  Cache: {} hits {} misses", cache.hits, cache.misses);
    let mut part2 = 0;
    for (row, lengths) in records {
        let mut full_row = row.clone();
        let mut full_lengths = lengths.clone();
        // Add 4 copies of the data, with the patterns separated by another ?.
        for _ in 0..4 {
            full_row.push('?');
            full_row.append(&mut row.clone());
            full_lengths.append(&mut lengths.clone());
        }
        part2 += calculate_possibles(&full_row, &full_lengths, &mut cache);
    }

    println!("Part 2: {}", part2);
    println!("  Cache: {} hits {} misses", cache.hits, cache.misses);
}
