// Template.
// A nice day where my part1 extended naturally to part 2 - it felt easier to keep
// track of the empty columns and just add them to the distances as needed than to
// try and redraw the grid.

// Determine the distance between two galaxies, given the set of empty rows and columns
// and the expansion multiplier
fn calculate_distance(
    this: &(usize, usize),
    other: &(usize, usize),
    empty_cols: &[usize],
    empty_rows: &[usize],
    multiplier: usize,
) -> usize {
    let (x1, y1) = this;
    let (x2, y2) = other;

    // Helper to check if a number is within a rnage, when we don't know which end of the range is smaller.
    let is_in_range = |i, t1, t2| (t1 < t2 && i > t1 && i < t2) || (t1 > t2 && i > t2 && i < t1);
    // For each of x and y:
    // - calculate the absolute difference.
    // - add the number of empty rows columns, multipled by the expansion factor.
    //   Subtract 1 to allow for the single row/column already present.
    let xdist = x1.abs_diff(*x2)
        + ((multiplier - 1)
            * empty_cols
                .iter()
                .filter(|&i| is_in_range(i, x1, x2))
                .count());
    let ydist = y1.abs_diff(*y2)
        + ((multiplier - 1)
            * empty_rows
                .iter()
                .filter(|&i| is_in_range(i, y1, y2))
                .count());
    xdist + ydist
}

pub fn run(input_path: String) {
    let input = std::fs::read_to_string(input_path).unwrap();
    let max_rows = input.lines().count();
    let max_cols = input.lines().next().unwrap().len();

    let mut empty_rows = vec![true; max_rows];
    let mut empty_cols = vec![true; max_cols];
    let mut galaxies = vec![];

    // Work out where the galaxies are, and which rows/columns are occupied.
    for (row, line) in input.lines().enumerate() {
        for (col, c) in line.char_indices() {
            if c == '#' {
                galaxies.push((col, row));
                empty_cols[col] = false;
                empty_rows[row] = false;
            }
        }
    }

    // Convert the empty rows and columns to a list of numbers.
    let empty_col_range: Vec<usize> = (0..max_cols).filter(|&i| empty_cols[i]).collect();
    let empty_row_range: Vec<usize> = (0..max_rows).filter(|&i| empty_rows[i]).collect();

    let mut part1 = 0;
    let mut part2 = 0;
    while let Some(this_galaxy) = galaxies.pop() {
        for other_galaxy in &galaxies {
            part1 += calculate_distance(
                &this_galaxy,
                other_galaxy,
                &empty_col_range,
                &empty_row_range,
                2,
            );
            part2 += calculate_distance(
                &this_galaxy,
                other_galaxy,
                &empty_col_range,
                &empty_row_range,
                1_000_000,
            );
        }
    }

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}
