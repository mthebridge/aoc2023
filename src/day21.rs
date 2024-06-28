// More pathfinding.

use std::collections::{HashSet, VecDeque};

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

fn max_positions_for_stepcount(grid: &Grid, max_steps: u64) -> u64 {
    let max_y = grid.grid.len();
    let max_x = grid.grid[0].len();

    // Start at "start"
    let mut visited = HashSet::new();
    let mut ans = 0;
    let mut queue = VecDeque::new();

    queue.push_back((0, grid.start));
    visited.insert(grid.start);
    while let Some((steps, (x, y))) = queue.pop_front() {
        if steps > max_steps {
            break;
        }
        if steps % 2 == 0 {
            // println!("Can reach ({x}, {y})");
            ans += 1;
        }

        if steps % 1000 == 0 {
            dbg!(steps);
            dbg!(queue.len());
        }
        // Add neighbours
        let neighs = [((x - 1, y)), ((x, y - 1)), ((x + 1, y)), ((x, y + 1))];
        for (nx, ny) in neighs {
            if !visited.contains(&(nx, ny))
                && grid.grid[(ny as usize) % max_y][(nx as usize) % max_x] == Space::Open
            {
                visited.insert((nx, ny));
                queue.push_back((steps + 1, (nx, ny)));
            }
        }
    }
    ans
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

    let part1 = max_positions_for_stepcount(&grid, 64);
    println!("Part 1: {}", part1);

    let part2 = max_positions_for_stepcount(&grid, 26501365);

    println!("Part 2: {}", part2);
}
