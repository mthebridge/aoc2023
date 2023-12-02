use std::collections::HashSet;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
struct NumberEntry {
    value: u32,
    col_start: usize,
    col_end: usize,
    row: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Coordinate {
    col: usize,
    row: usize,
}

impl NumberEntry {
    fn get_neighbor_coords(&self) -> impl Iterator<Item = Coordinate> {
        // Row before.
        std::iter::repeat(self.row - 1)
            .zip((self.col_start - 1)..=(self.col_end + 1))
            .chain(
                // Row after
                std::iter::repeat(self.row + 1)
                    .zip((self.col_start - 1)..=(self.col_end + 1))
                    .chain(
                        // Left element
                        std::iter::once((self.row, self.col_start - 1))
                            .chain(std::iter::once((self.row, self.col_end + 1))),
                    ),
            )
            .map(|(row, col)| Coordinate { col, row })
    }
}

pub fn run(input_path: String) {
    let input = std::fs::read_to_string(input_path).unwrap();
    let mut symbols = HashSet::new();
    let mut asterisks = HashSet::new();
    let mut numbers = HashSet::new();
    let mut current_number = None;

    // Build up a list of:
    for (row, line) in input.lines().enumerate() {
        for (col, c) in line.char_indices() {
            match c {
                c if c.is_ascii_digit() => match current_number.as_mut() {
                    None => {
                        current_number = Some(NumberEntry {
                            value: c.to_digit(10).unwrap(),
                            row: row + 1,
                            col_start: col + 1,
                            col_end: col + 1,
                        })
                    }
                    Some(number) => {
                        number.value *= 10;
                        number.value += c.to_digit(10).unwrap();
                        number.col_end += 1
                    }
                },
                c => {
                    // Handle end of number
                    if let Some(n) = current_number.take() {
                        numbers.insert(n);
                    }
                    if c == '*' {
                        asterisks.insert(Coordinate {
                            col: col + 1,
                            row: row + 1,
                        });
                    }
                    if c != '.' {
                        // Symbol
                        symbols.insert(Coordinate {
                            col: col + 1,
                            row: row + 1,
                        });
                    }
                }
            }
        }
        // If the row finished with a number, save it.
        if let Some(n) = current_number.take() {
            numbers.insert(n);
        }
    }

    // For part 1: Find all the numbers such that they have a symbol in their neighbours.
    let part_numbers = numbers
        .iter()
        .filter(|&number| {
            number
                .get_neighbor_coords()
                .any(|neighbor| symbols.contains(&neighbor))
        })
        .map(|number| number.value);

    println!("Part 1: {}", part_numbers.sum::<u32>());

    // Find the gears.  These are asterisks who have exactly two neighbours.
    let gear_ratios = asterisks.iter().map(|this| {
        // Look for neighbouring numbers.
        let neighbors = numbers
            .iter()
            .filter(|n| n.get_neighbor_coords().any(|c| c == *this));
        if neighbors.clone().count() == 2 {
            neighbors.map(|neigh| neigh.value as u64).product::<u64>()
        } else {
            0
        }
    });
    println!("Part 2: {}", gear_ratios.sum::<u64>());
}
