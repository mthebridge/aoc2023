// Template.
// Copy to daynum.rs, and uncomment relevant lins in main to add

use std::collections::{BinaryHeap, HashMap};

type Grid = Vec<Vec<u8>>;

// Direction of travel.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    N,
    S,
    E,
    W,
}

impl std::fmt::Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Direction::E => write!(f, "E"),
            Direction::N => write!(f, "N"),
            Direction::S => write!(f, "S"),
            Direction::W => write!(f, "W"),
        }
    }
}

const ALL_DIRS: &'static [Direction] = &[Direction::N, Direction::E, Direction::S, Direction::W];

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct NodeEntry {
    x: usize,
    y: usize,
    last_entered_dir: Direction,
    straight_count: u8,
}

impl std::fmt::Display for NodeEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "({}, {}) - {}{}",
            self.x, self.y, self.straight_count, self.last_entered_dir
        )
    }
}

impl NodeEntry {
    fn get_neighbours(&self, max_x: usize, max_y: usize) -> impl Iterator<Item = Self> + '_ {
        ALL_DIRS.iter().filter_map(move |dir| {
            (match dir {
                Direction::N if self.y > 0 => Some((self.x, self.y - 1)),
                Direction::E if self.x < max_x - 1 => Some((self.x + 1, self.y)),
                Direction::S if self.y < max_y - 1 => Some((self.x, self.y + 1)),
                Direction::W if self.x > 0 => Some((self.x - 1, self.y)),
                _ => None,
            })
            .and_then(|next| {
                if *dir == self.last_entered_dir && self.straight_count >= 3 {
                    None
                } else {
                    let straight_count = if *dir == self.last_entered_dir {
                        self.straight_count + 1
                    } else {
                        1
                    };
                    Some(NodeEntry {
                        x: next.0,
                        y: next.1,
                        last_entered_dir: *dir,
                        straight_count,
                    })
                }
            })
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct QueueEntry(u64, NodeEntry);

// Implement reverse ordering for a max-heap.
impl PartialOrd for QueueEntry {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        other.0.partial_cmp(&self.0)
    }
}

impl Ord for QueueEntry {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.0.cmp(&self.0)
    }
}

// const MAX_GRID_VALUE: u64 = 9;
// fn worst_case(node: &NodeEntry, target: &(usize, usize)) -> u64 {

//     //MAX_GRID_VALUE * (target.0.abs_diff(node.x) + target.1.abs_diff(node.y)) as u64
//     0
// }

pub fn run(input_path: String) {
    let input = std::fs::read_to_string(input_path).unwrap();
    let grid: Grid = input
        .lines()
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap() as u8).collect())
        .collect();

    // Run shortest path from (0, 0) to (max_x, max_y).
    // Classic Dijkstra, with the caveat that you can't go more than 3 steps in the same direction.
    // So need to store the recent direction + num steps in that direction.
    let max_y = grid.len();
    let max_x = grid[0].len();
    let mut distances = HashMap::new();
    // Our initial unvisited nodes is everything.
    let mut candidates = BinaryHeap::new();
    let start = NodeEntry {
        x: 0,
        y: 0,
        last_entered_dir: Direction::E,
        straight_count: 1,
    };
    let target = (max_x - 1, max_y - 1);
    candidates.push(QueueEntry(0, start.clone()));
    distances.insert(start.clone(), 0);

    let mut best = std::u64::MAX;

    // let mut i = 0;

    while let Some(QueueEntry(_, current)) = candidates.pop() {
        let cur_d = *distances.get(&current).unwrap();
        println!("Try {current} ({cur_d})");

        // Check the current distance
        if (current.x, current.y) == target {
            // We're at the target.  Cannot possibly get any better to keep going
            // from here.
            best = best.min(cur_d);
            continue;
        }

        for n in current.get_neighbours(max_x, max_y) {
            let entry = distances.entry(n.clone()).or_insert(std::u64::MAX);
            let new_dist = cur_d + grid[n.y][n.x] as u64;
            if new_dist < *entry {
                *entry = new_dist;
                candidates.push(QueueEntry(0, n));
            }
        }
    }

    let part1 = best;
    println!("Part 1: {}", part1);

    let part2 = 0;

    println!("Part 2: {}", part2);
}

