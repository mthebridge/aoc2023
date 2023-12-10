struct Grid {
    grid: Vec<Vec<char>>,
    start: (usize, usize),
}

impl Grid {
    fn from_str(input: &str) -> Self {
        let mut start = None;
        let grid = input
            .lines()
            .enumerate()
            .map(|(y, row)| {
                row.chars()
                    .enumerate()
                    .map(|(x, c)| {
                        if c == 'S' {
                            start = Some((x, y));
                        }
                        c
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        Grid {
            grid,
            start: start.unwrap(),
        }
    }

    fn check_bounds(&self, x: usize, y: usize) -> bool {
        let max_y = self.grid.len();
        let max_x = self.grid[0].len();
        x < max_x && y < max_y
    }

    fn pipe_neighbours(&self, pos: (usize, usize)) -> impl Iterator<Item = (usize, usize)> + '_ {
        let (x, y) = pos;
        let element = self.grid[y][x];
        // dbg!(&element);

        let candidates = match element {
            '|' => vec![(x, y.saturating_sub(1)), (x, y + 1)],
            '-' => vec![(x.saturating_sub(1), y), (x + 1, y)],
            'L' => vec![(x, y.saturating_sub(1)), (x + 1, y)],
            'J' => vec![(x, y.saturating_sub(1)), (x.saturating_sub(1), y)],
            '7' => vec![(x.saturating_sub(1), y), (x, y + 1)],
            'F' => vec![(x + 1, y), (x, y + 1)],
            '.' => vec![],
            'S' => {
                // Start requires us to work out which ones connect.
                [
                    (x.saturating_sub(1), y),
                    (x + 1, y),
                    (x, y.saturating_sub(1)),
                    (x, y + 1),
                ]
                .into_iter()
                .filter(|(nx, ny)| {
                    (*nx, *ny) != (x, y) && self.pipe_neighbours((*nx, *ny)).any(|n| n == (x, y))
                })
                .collect()
            }
            _ => panic!("Invalid character"),
        };
        candidates
            .into_iter()
            .filter(move |(nx, ny)| (*nx, *ny) != (x, y) && self.check_bounds(*nx, *ny))
    }
}

pub fn run(input_path: String) {
    let input = std::fs::read_to_string(input_path).unwrap();
    let grid = Grid::from_str(&input);

    // Start
    let mut this = grid.start;
    let mut path = vec![];
    loop {
        // dbg!(&this);
        path.push(this);
        match grid
            .pipe_neighbours(this)
            .filter(|n| !path.contains(n))
            .next()
        {
            Some(next) => this = next,
            None => break,
        }
    }

    println!("Part 1: {}", (path.len() + 1) / 2);
    let part2 = 0;

    println!("Part 2: {}", part2);
}
