// Got there in the end.
// Assumes that the cycle eventually stabilises, which it does.
// Lots of probably unnecessary reallocating into vectors.

use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    N,
    S,
    E,
    W,
}

fn tilt_grid(grid: Vec<Vec<char>>, dir: Direction) -> Vec<Vec<char>> {
    match dir {
        Direction::N => grid.iter().map(|col| tilt_line(col, false)).collect(),
        Direction::E => transpose(
            transpose(grid)
                .iter()
                .map(|row| tilt_line(row, true))
                .collect(),
        ),
        Direction::S => grid.iter().map(|col| tilt_line(col, true)).collect(),
        Direction::W => transpose(
            transpose(grid)
                .iter()
                .map(|row| tilt_line(row, false))
                .collect(),
        ),
    }
}

#[allow(dead_code)]
fn print_grid(grid: &[Vec<char>]) {
    for y in 0..grid[0].len() {
        for x in 0..grid.len() {
            print!("{}", grid[x][y]);
        }
        println!("");
    }
}

fn tilt_line(line: &[char], rev: bool) -> Vec<char> {
    // Iterate down the line.  For each rock `O`, we want to work out where it
    // would end up rolled towards the end, which is:
    // - space after the last fixed rock (#)
    // - plus one for each rock between the fixed and the last
    let mut last_fixed = 0;
    let mut rocks_this_stretch = 0;
    let line: Vec<_> = if rev {
        line.iter().rev().enumerate().collect()
    } else {
        line.iter().enumerate().collect()
    };
    let mut new_line: Vec<_> = line.iter().map(|x| *x.1).collect();
    for (i, &c) in line {
        match c {
            'O' => {
                let new_pos = last_fixed + rocks_this_stretch;
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

    if rev {
        new_line.reverse();
    }
    new_line
}

fn calculate_load(grid: &[Vec<char>]) -> usize {
    grid.iter()
        .map(|col| {
            let length = col.len();
            col.iter()
                .enumerate()
                .map(|(i, &c)| if c == 'O' { length - i } else { 0 })
                .sum::<usize>()
        })
        .sum::<usize>()
}

fn transpose<T: Copy>(grid: Vec<Vec<T>>) -> Vec<Vec<T>> {
    (0..grid[0].len())
        .map(|i| (0..grid.len()).map(|j| grid[j][i]).collect::<Vec<_>>())
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
    let part1_grid = tilt_grid(columns.clone(), Direction::N);
    let part1 = calculate_load(&part1_grid);
    println!("Part 1: {}", part1);

    let mut new_grid = columns;
    let mut cache = HashMap::new();
    let mut target = None;
    for i in 0.. {
        if let Some(last) = cache.get(&new_grid.clone()) {
            // Hit the cache.  No point keeping going, we'll just cycle again.
            // Work out how many more steps we need to reach the expected end point.
            let real_target = target.get_or_insert_with(|| {
                let cycle = i - last;
                // We need to find the correct offest that will match the billionth step.
                // To get that:
                //  - Work out what 1 billion would be mod cycle
                // - Subtract the offset of the last.
                let offset = (1_000_000_000 - last) % cycle;
                i + cycle + offset
            });
            if *real_target == i {
                break;
            }
        }

        cache.insert(new_grid.clone(), i);

        // Tilt the grid
        new_grid = tilt_grid(new_grid, Direction::N);
        new_grid = tilt_grid(new_grid, Direction::W);
        new_grid = tilt_grid(new_grid, Direction::S);
        new_grid = tilt_grid(new_grid, Direction::E);
    }
    let part2 = calculate_load(&new_grid);
    println!("Part 2: {}", part2);
}
