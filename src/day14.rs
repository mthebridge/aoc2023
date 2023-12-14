// Template.
// Copy to daynum.rs, and uncomment relevant lins in main to add

fn tilt_line(line: &[char]) -> Vec<char> {
    // Iterate down the line.  For each rock `O`, we want to work out where it
    // would end up rolled north, which is:
    // - space after the last fixed rock (#)
    // - plus one for each rock between the fixed and the last
    // - sum them as we go
    let mut last_fixed = 0;
    let mut rocks_this_stretch = 0;
    let mut new_line = line.to_vec();
    for (i, &c) in line.iter().enumerate() {
        match c {
            'O' => {
                let new_pos = last_fixed + rocks_this_stretch;
                // println!("Rock at {i} now at {new_pos}");
                if new_pos != i {
                    new_line[new_pos] = c;
                    new_line[i] = '.';
                }
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

    new_line
}

fn calculate_load(line: &[char]) -> usize {
    let length = line.len();
    line.iter()
        .enumerate()
        .map(|(i, &c)| if c == 'O' { length - i } else { 0 })
        .sum()
}

fn transpose<T: Copy>(grid: Vec<Vec<T>>, rev: bool) -> Vec<Vec<T>> {
    (0..grid[0].len())
    .map(|i| {
            if rev {
            (0..grid.len()).rev()
                .map(|j| grid[j][i])
                .collect::<Vec<_>>()
            } else {
                (0..grid.len())
                .map(|j| grid[j][i])
                .collect::<Vec<_>>()
            }
        })
        .collect::<Vec<_>>()
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
        .map(|col| {
            let new_col = tilt_line(col);
            calculate_load(&new_col)
        })
        .sum::<usize>();
    println!("Part 1: {}", part1);

    let mut new_grid = columns;
    for i in 0..1_000_000_000 {
        if i % 1000000 == 0 {
            println!("Loop {i}");
        }
        // Tilt N
        new_grid = new_grid.iter().map(|col| tilt_line(col)).collect();
        // Tilt E
        new_grid = transpose(new_grid, false)
            .iter()
            .map(|row| tilt_line(row))
            .collect();
        // Tilt S.  Transpose columns
        new_grid = transpose(new_grid, true)
            .iter_mut()
            .map(|col| {
                col.reverse();
                let mut new = tilt_line(col);
                new.reverse();
                new
            })
            .collect();
        // Tilt W. Then transpose columns.
        new_grid = transpose(new_grid, false)
            .iter_mut()
            .map(|col| {
                col.reverse();
                let mut new = tilt_line(col);
                new.reverse();
                new
            })
            .collect();
        new_grid = transpose(new_grid, true);
    }

    let part2 = new_grid
        .iter()
        .map(|col| calculate_load(col))
        .sum::<usize>();
    println!("Part 2: {}", part2);
}
