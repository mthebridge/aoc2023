// That was fun.  A nice puzzle, and several ways to go at it.
//
// The good - I decided to go for the iterate-over-the-grid for the second part: and work out for each cell
// Whether it was inside or outside the loop by tracking how many times we'd crossed the path on that row.
// Release build runs in 50ms so happy on that front.
//
// On the other hand several hours wasted diue to errors...
// - Forgetting to handle the dangling section case at the end of a row (easily fixed).
// - Not proeprly thinking through my algorithm.
// - I forgot to handle the start cell at all at first. Then when I fixed that, I got it *wrong* - but only if the S was
// on the right edge of a horizontal segment, which none of the test examples are.
// In the end the liberal assertions saved me, so lesson is to put those in sooner than later!

// Simple wrapper around the grid, holding the start cell separately.
struct Grid {
    grid: Vec<Vec<char>>,
    start: (usize, usize),
}

impl Grid {
    fn get(&self, pos: (usize, usize)) -> char {
        let (x, y) = pos;
        self.grid[y][x]
    }
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

    // Calculate the connected positions to this element.
    // These are the two adjacent elements absed on the pipe shape.
    fn pipe_neighbours(&self, pos: (usize, usize)) -> impl Iterator<Item = (usize, usize)> + '_ {
        let (x, y) = pos;

        // Use of saturating substraction avoids wrapping errors (although means we have to filter out
        // pos again later).
        let candidates = match self.get((x, y)) {
            '|' => vec![(x, y.saturating_sub(1)), (x, y + 1)],
            '-' => vec![(x.saturating_sub(1), y), (x + 1, y)],
            'L' => vec![(x, y.saturating_sub(1)), (x + 1, y)],
            'J' => vec![(x, y.saturating_sub(1)), (x.saturating_sub(1), y)],
            '7' => vec![(x.saturating_sub(1), y), (x, y + 1)],
            'F' => vec![(x + 1, y), (x, y + 1)],
            '.' => vec![],
            'S' => {
                // We don't know the shape.  Find the neighbours by checking all 4 orthogonal
                // neighbours and seeing if *this* is a neighbour of the candidate.
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
            // If the saturating substraction returned ourselves, don't return that
            // or we'll recurse forever (not that I made that mistake, cough cough...)
            .filter(move |(nx, ny)| (*nx, *ny) != (x, y))
    }

    // Check if two positions are directly connected neighbours.
    fn is_pipe_neighbour(&self, first: (usize, usize), other: (usize, usize)) -> bool {
        self.pipe_neighbours(first).any(|n| n == other)
    }

    // Does the given horizontal section of pipe cross the outside/inside boundary?
    // Assumes caller has already checked that the provided section is a continuous
    // section of the path.
    fn is_crossing_boundary(&self, y: usize, start_x: usize, end_x: usize) -> bool {
        let (start_val, end_val) = (self.get((start_x, y)), self.get((end_x, y)));
        // This section is a crossing if the start and end have different vertical connections
        // This is a bit more tedious in the case of the S.
        let real_start = if start_val == 'S' {
            if self.is_pipe_neighbour((start_x, y), (start_x, y + 1)) {
                'F'
            } else {
                'L'
            }
        } else {
            start_val
        };
        let real_end = if end_val == 'S' {
            if self.is_pipe_neighbour((end_x, y), (end_x, y + 1)) {
                '7'
            } else {
                'J'
            }
        } else {
            end_val
        };

        // Check assumptions about crossing points.
        debug_assert!(real_start == 'F' || real_start == 'L');
        debug_assert!(real_end == 'J' || real_end == '7');
        for i in start_x + 1..end_x {
            debug_assert_eq!(self.get((i, y)), '-');
        }
        (real_start == 'F' && real_end == 'J') || (real_start == 'L' && real_end == '7')
    }

    // Calculate the points in the path that are on a given row.
    // Returns the sorted list of x-coordinates.
    fn get_crossing_points<'a>(
        &'a self,
        y: usize,
        path: impl Iterator<Item = &'a (usize, usize)>,
    ) -> Vec<usize> {
        let mut cross_points = path
            .filter_map(|&pos| {
                // Only include points in the row.
                if pos.1 == y {
                    Some(pos.0)
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();
        cross_points.sort();
        cross_points
    }

    fn get_row_pipe_sections(&self, y: usize, cross_points: &[usize]) -> Vec<PipeRow> {
        let mut section_start = None;
        let mut pipe_sections = vec![];
        for pair in cross_points.windows(2) {
            // Consider every overlapping pair of points in the loop.
            let (this_point, next_point) = (pair[0], pair[1]);
            if self.is_pipe_neighbour((next_point, y), (this_point, y)) {
                // This point is part of the same section of pipe as the previous one.
                // If not already in a section, store the previous as the start marker.
                let _ = section_start.get_or_insert(this_point);
            } else {
                // Completed a section.
                pipe_sections.push(PipeRow::build(self, y, section_start, this_point));
                section_start = None
            }
        }
        // Handle any final section completed by the final point.
        if let Some(end) = cross_points.last() {
            pipe_sections.push(PipeRow::build(self, y, section_start, *end));
        }

        pipe_sections
    }
}

// A continuous row of pipe within the grid.
#[derive(Debug, Clone)]
struct PipeRow {
    start: usize,
    end: usize,
    is_crossing: bool,
}

impl PipeRow {
    fn build(grid: &Grid, y: usize, start: Option<usize>, end: usize) -> Self {
        if let Some(start) = start {
            // End of a section - add it to the list.
            PipeRow {
                start,
                end,
                is_crossing: grid.is_crossing_boundary(y, start, end),
            }
        } else {
            // Not neighbours, and not in a section.  Must be a vertical boundary.
            // Add a single-element section
            debug_assert_eq!(grid.get((end, y)), '|');
            PipeRow {
                start: end,
                end,
                // A vertical line must by definition cross from inside to outside.
                is_crossing: true,
            }
        }
    }

    // Is the row beyond this section inside or outside the loop?
    fn is_inside_after(&self, was_inside: bool) -> bool {
        // We're inside the loop if:
        // - we already were and the section doesn't cross
        // - we previously weren't, but the section does cross.
        (self.is_crossing && !was_inside) || (!self.is_crossing && was_inside)
    }
}

// Work out how many points are inside the loop for a single row.
fn get_inside_point_count<'a>(
    grid: &'a Grid,
    y: usize,
    path: impl Iterator<Item = &'a (usize, usize)>,
) -> usize {
    // Work out the x-coordinates where the row intersects the loop.
    let cross_points = grid.get_crossing_points(y, path);

    // Now split these points up into segments of connected pipe.
    let pipe_sections = grid.get_row_pipe_sections(y, &cross_points);

    // Then we need to work out whether the spaces "between" each chunk are inside or outside.
    // We can do this by looping over the pairs of *sections*, and adding the gaps between them, *if* we
    // are on the "inside".  We can determine if we are on the inside because this only changes
    // over a crossing point.
    //
    let (inside, inside_count) =
        pipe_sections
            .windows(2)
            .fold((false, 0), |(is_inside, total), pair| {
                let (prev_section, this_section) = (&pair[0], &pair[1]);
                let now_inside = prev_section.is_inside_after(is_inside);
                let new_total = if now_inside {
                    // Remember to subtract 1 since we want the number of gaps between the pipes
                    // (eg adjacent numbers should give zero).
                    let gap = this_section.start - prev_section.end - 1;
                    total + gap
                } else {
                    total
                };
                (now_inside, new_total)
            });
    // Sense check: we should end up outside the loop!
    debug_assert!(!pipe_sections
        .last()
        .map(|section| section.is_inside_after(inside))
        .unwrap_or(inside));
    inside_count
}

pub fn run(input_path: String) {
    let input = std::fs::read_to_string(input_path).unwrap();
    let grid = Grid::from_str(&input);

    // Keep track of the path around the loop.
    let mut this = grid.start;
    let mut path = vec![];
    loop {
        path.push(this);
        // Get the neighbour we haven't seen - can only be at most one.
        match grid.pipe_neighbours(this).find(|n| !path.contains(n)) {
            // Next - go round the loop again
            Some(next) => this = next,
            // Back at the start
            None => break,
        }
    }

    // The furthest point is half the loop length.  Add 1 to allow for odd numbers.
    println!("Part 1: {}", (path.len() + 1) / 2);

    // For part2 we need to find howe many points are *inside* the loop.
    // We can consider the grid row by row.
    let part2: usize = (0..grid.grid.len())
        .map(|y| get_inside_point_count(&grid, y, path.iter()))
        .sum();

    println!("Part 2: {}", part2);
}
