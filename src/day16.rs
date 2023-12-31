// A lot of copy-pasting grids areound this year!
// Turns out you can just brute force aprt 2 relatively easily.
// Some tedium getting the orderings right in the directions, but
// otherwise felt quite like some earlier days.

use std::collections::HashSet;

// Mirror setup.
type Grid = Vec<Vec<char>>;

// Direction of beam travel.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    N,
    S,
    E,
    W,
}

// Get the next cell the beam should hit in a given direction,
// returning None if falling off the grid.
fn get_neighbour(
    x: usize,
    y: usize,
    dir: Direction,
    max_x: usize,
    max_y: usize,
) -> impl Iterator<Item = (Direction, (usize, usize))> {
    match dir {
        Direction::N if y > 0 => Some((dir, (x, y - 1))),
        Direction::W if x > 0 => Some((dir, (x - 1, y))),
        Direction::S if y < max_y - 1 => Some((dir, (x, y + 1))),
        Direction::E if x < max_x - 1 => Some((dir, (x + 1, y))),
        _ => None,
    }
    .into_iter()
}

// Get the cells a beam will hit next, and add them to the list.
fn extend_single_beam(
    grid: &Grid,
    beams: &mut Vec<(Direction, (usize, usize))>,
    entry_dir: Direction,
    cell: (usize, usize),
) {
    let (x, y) = cell;
    let this = grid[y][x];
    let max_x = grid[0].len();
    let max_y = grid.len();
    match this {
        // Keep going if empty or a parallel splitter
        '.' => beams.extend(get_neighbour(x, y, entry_dir, max_x, max_y)),
        '|' if entry_dir == Direction::N || entry_dir == Direction::S => {
            beams.extend(get_neighbour(x, y, entry_dir, max_x, max_y));
        }
        '-' if entry_dir == Direction::E || entry_dir == Direction::W => {
            beams.extend(get_neighbour(x, y, entry_dir, max_x, max_y))
        }
        // Bend
        '/' => {
            let new_dir = match entry_dir {
                Direction::N => Direction::E,
                Direction::E => Direction::N,
                Direction::S => Direction::W,
                Direction::W => Direction::S,
            };
            beams.extend(get_neighbour(x, y, new_dir, max_x, max_y))
        }
        '\\' => {
            let new_dir = match entry_dir {
                Direction::N => Direction::W,
                Direction::E => Direction::S,
                Direction::S => Direction::E,
                Direction::W => Direction::N,
            };
            beams.extend(get_neighbour(x, y, new_dir, max_x, max_y))
        }
        // Splits.
        '-' => beams.extend(
            get_neighbour(x, y, Direction::E, max_x, max_y).chain(get_neighbour(
                x,
                y,
                Direction::W,
                max_x,
                max_y,
            )),
        ),
        '|' => beams.extend(
            get_neighbour(x, y, Direction::N, max_x, max_y).chain(get_neighbour(
                x,
                y,
                Direction::S,
                max_x,
                max_y,
            )),
        ),
        _ => panic!("Invalid char"),
    }
}

// Fire the beam into the grid and follow the path.
// Keep track of visited cells and crucially also the direction - if we hit a path we've previously
// traversed then we don't need to keep going as it's already counted.
fn get_energize_count(grid: &Grid, start_dir: Direction, start_pos: (usize, usize)) -> usize {
    let mut beams = vec![(start_dir, start_pos)];
    let mut visited = HashSet::new();

    while let Some((dir, pos)) = beams.pop() {
        // Move on if we haven't arrived at this cell from this direction
        if visited.insert((dir, pos)) {
            extend_single_beam(grid, &mut beams, dir, pos)
        }
    }

    // We want to de-deuplicate on direction, so re-combine into a HashSet that drops the direction,
    // and then take its size to get the energized cell count.
    // (We could separately be building up the two hashSets, but perf is good enough).
    visited
        .iter()
        .map(|(_, pos)| pos)
        .collect::<HashSet<_>>()
        .len()
}

pub fn run(input_path: String) {
    let input = std::fs::read_to_string(input_path).unwrap();
    let grid: Grid = input.lines().map(|l| l.chars().collect()).collect();

    // Starting top-left, search the grid
    let part1 = get_energize_count(&grid, Direction::E, (0, 0));
    println!("Part 1: {}", part1);
    let max_x = grid[0].len();
    let max_y = grid.len();

    // Now try from every side square.
    // I can't see much clever we can do without brute-force beyond some cacheing of previously seen grid
    // states across runs - but a release build runs in 200ms so I'm not super fussed about optimizing.
    let part2 = (0..max_y)
        .flat_map(|y| {
            [
                get_energize_count(&grid, Direction::E, (0, y)),
                get_energize_count(&grid, Direction::W, (0, max_y - 1 - y)),
            ]
        })
        .chain((0..max_x).flat_map(|x| {
            [
                get_energize_count(&grid, Direction::S, (x, 0)),
                get_energize_count(&grid, Direction::N, (max_x - x - 1, 0)),
            ]
        }))
        .max()
        .unwrap();
    println!("Part 2: {}", part2);
}
