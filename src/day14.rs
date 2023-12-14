// Template.
// Copy to daynum.rs, and uncomment relevant lins in main to add

fn calculate_column_load(column: &[char]) -> usize {
    // Iterate down the column.  For each rock `O`, we want to work out where it
    // would end up rolled north, which is:
    // - space after the last fixed rock (#)
    // - plus one for each rock between the fixed and the last
    // - sum them as we go
    let length = column.len();
    let mut load = 0;
    let mut last_fixed = 0;
    let mut rocks_this_stretch = 0;
    // println!("New column: {length} spaces");
    for (i, &c) in column.iter().enumerate() {
        match c {
            'O' => {
                let new_pos = last_fixed + rocks_this_stretch;
                // println!("Rock at {i} now at {new_pos}");
                load += length - new_pos;
                rocks_this_stretch += 1;
            }
            '#' => {
                rocks_this_stretch = 0;
                last_fixed = i + 1
            }
            '.' => (),
            _ => panic!("Invalid character"),
        }
    }

    load
}

pub fn run(input_path: String) {
    let input = std::fs::read_to_string(input_path).unwrap();
    let mut columns: Vec<Vec<char>> = vec![];

    for row in input.lines() {
        for (x, c) in row.char_indices() {
            if let Some(col) = columns.get_mut(x) {
                col.push(c);
            } else {
                // New column
                columns.push(vec![c])
            }
        }
    }

    let part1 = columns
        .iter()
        .map(|col| calculate_column_load(col))
        .sum::<usize>();
    println!("Part 1: {}", part1);

    let part2 = 0;
    println!("Part 2: {}", part2);
}
