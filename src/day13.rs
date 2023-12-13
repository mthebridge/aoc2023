// Found this easier today.
// Algorithm seemed fairly "obvious" - no need to worry about being O(n^2) given
// the small sizes of the input maps.
// Tranposing the nested vectors is a bit tedious but it makes life easier than trying to
// avoid the reallocation.
// Clearly part2 needed to be cleverer than "try replacing all dots with hashes in turn" -
// but it's actually not that much more code in the end to check.

// Walk the lines looking for a reflection.
fn find_reflection_line(line_map: &[Vec<char>], part2: bool) -> Option<usize> {
    let max = line_map.len();
    (0..max - 1).find(|&i| {
        // Check if the reflection is immediately to the right of (or below) this line.
        // We don't want to go off the edge of the map so calcualte how far we can
        // safely go.
        let distance_to_check = (max - 1 - i).min(i + 1);
        // For part1, just check all lines match reflectively
        if !part2 {
            (0..distance_to_check).all(|j| {
                // println!("Checking {} ==  {}", i + j + 1, i - j);
                line_map[i + j + 1] == line_map[i - j]
            })
        } else {
            // For part 2, this is a match if exactly 1 line differs, *and* it differs by a
            // single character.
            let mut found_diff = false;
            for j in 0..distance_to_check {
                let (prev, next) = (&line_map[i + j + 1], &line_map[i - j]);
                let num_char_diffs = (0..prev.len()).filter(|&x| prev[x] != next[x]).count();
                if num_char_diffs == 1 && !found_diff {
                    // Single difference - could be a reflection line
                    found_diff = true
                } else if num_char_diffs >= 1 {
                    // Too many differences, not a reflection - bail now
                    return false;
                } else {
                    // No differences.  Keep checking.
                }
            }
            // This is only the right line if we found a single difference
            found_diff
        }
    })
}

// Find the x or y location of the reflection
fn get_mirror_reflection_val(mirror: &str, part2: bool) -> usize {
    let row_map = mirror
        .lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    // First check the rows.
    if let Some(row) = find_reflection_line(&row_map, part2) {
        // Add one for zero-indxing, and multiply by 100 because a row.
        100 * (row + 1)
    } else {
        // Transpose the map. and check the columns.
        let col_map = (0..row_map[0].len())
            .map(|col| {
                (0..row_map.len())
                    .map(|row| row_map[row][col])
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        // Again remember + 1 for zero-indexing
        find_reflection_line(&col_map, part2).unwrap() + 1
    }
}

pub fn run(input_path: String) {
    let input = std::fs::read_to_string(input_path).unwrap();
    let mirrors = input.split("\n\n");

    let part1 = mirrors
        .clone()
        .map(|mirror| get_mirror_reflection_val(mirror, false))
        .sum::<usize>();
    println!("Part 1: {}", part1);

    let part2 = mirrors
        .map(|mirror| get_mirror_reflection_val(mirror, true))
        .sum::<usize>();

    println!("Part 2: {}", part2);
}
