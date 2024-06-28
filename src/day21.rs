// More pathfinding.

use std::collections::{HashMap, VecDeque};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Space {
    Open,
    Rock,
}

// Simple wrapper around the grid, holding the start cell separately.
#[derive(Debug, Clone, PartialEq, Eq)]
struct Grid {
    grid: Vec<Vec<Space>>,
    start: (i64, i64),
}

fn generate_distances(grid: &Grid) -> HashMap<(i64, i64), u64> {
    let max_y = grid.grid.len();
    let max_x = grid.grid[0].len();
    assert_eq!(max_x, max_y); // By inspection of input.

    // Start at "start"
    let mut visited = HashMap::new();
    let mut queue = VecDeque::new();

    queue.push_back((0, grid.start));
    visited.insert(grid.start, 0);
    while let Some((steps, (x, y))) = queue.pop_front() {
        // Add neighbours
        let neighs = [((x - 1, y)), ((x, y - 1)), ((x + 1, y)), ((x, y + 1))];
        for (nx, ny) in neighs {
            let nx = (nx as usize % max_x) as i64;
            let ny = (ny as usize % max_y) as i64;
            if !visited.contains_key(&(nx, ny))
                && grid.grid[ny as usize][nx as usize] == Space::Open
            {
                visited.insert((nx, ny), steps + 1);
                queue.push_back((steps + 1, (nx, ny)));
            }
        }
    }
    visited
}

pub fn run(input_path: String) {
    let input = std::fs::read_to_string(input_path).unwrap();
    let mut start = None;
    let grid = input
        .lines()
        .enumerate()
        .map(|(y, l)| {
            l.chars()
                .enumerate()
                .map(|(x, c)| {
                    if c == 'S' {
                        start = Some((x as i64, y as i64))
                    }
                    match c {
                        '#' => Space::Rock,
                        '.' | 'S' => Space::Open,
                        _ => panic!("Bad input"),
                    }
                })
                .collect()
        })
        .collect();
    let grid = Grid {
        grid,
        start: start.unwrap(),
    };

    // Observe that:
    // - if we can get to a position in N steps we can get there in every other multiple of 2 below N.
    // - the provided map has no rocks on the outer boundary.
    //
    // So we can start by breadth-first searching every position on the grid and storing its
    // distance from the start.  If that disatnce is at most 64 and of even parity, we can reach it.
    let distances = generate_distances(&grid);
    let part1 = distances
        .values()
        .filter(|&&v| v <= 64 && v % 2 == 0)
        .count();
    println!("Part 1: {}", part1);

    // For part2, we can't keep iterating.
    // Note that because the grid has open edges, if we need to cross more than 1 grid instance, we can always do so
    // in the Manhattan distance.
    // So, we can reach:
    // - Anywhere in the existing grid, multiplied by that is (2N - 1) * N, where N is (max_steps / grid size)
    // - Then for remaining spaces, we can reach anywhere that is
    let maxsteps_p2 = 26501365u64;
    let grid_repeats = maxsteps_p2 / grid.grid.len() as u64;
    let grid_max_size = 2 * grid_repeats - 1;
    let local_reachable = distances.values().filter(|&&v| v <= maxsteps_p2 && v % 2 == 1).count();
    dbg!(local_reachable, grid_max_size);
    let part2 = local_reachable as u64 * grid_max_size * grid_max_size; //max_positions_for_stepcount(&grid, 26501365);
    // Now need to add on all those reachable spots on the most distant grid.

    println!("Part 2: {}", part2);
}
