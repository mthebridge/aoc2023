// Felt straightforward.  Parse out the number and symbol positions
// into HashSets for fast lookup: then we can simply check neighbours of the numbers
// as needed.  Part 2 wasn't that much extra work pleasingly.
// The use of  1-indexed coordinates allows including the row/column "outside" the
// grid as neighbours and not having to edge case or worry about bounds checks.
// Since all we do with neighbour coord values is check if they are in the symbols list
// this as fine.
// Some small things I missed first time:
// - Remembering to handle numbers the finish on the end of a row
// - Using `..=` for inclusive ranges!
//
// There's again plenty of perf improvements at a complexity cost - for example:
// - We could be using a HashMap for the number entries keyed off their position,
// so we can avoid checking numbers miles away for neighbouring gears.
// - We could keep track of neighbours/gears as we parse the input, rather than
// calculating them later.
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
    // Get all "neighbours" of a number.
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
    // Position of all symbols for part 1.
    let mut symbols = HashSet::new();
    // Position of all asterisks for part 2 - we'll work out which are
    // actually gears later.
    let mut asterisks = HashSet::new();
    // Positions and values of all numbers.
    let mut numbers = HashSet::new();
    // We'll be parsing characer-by-charcter, so need to keep track of
    // where we are in the current number.
    let mut current_number = None;

    // Parse the input.
    for (row, line) in input.lines().enumerate() {
        for (col, c) in line.char_indices() {
            match c {
                c if c.is_ascii_digit() => match current_number.as_mut() {
                    None => {
                        // Start of a number.
                        // Add 1 to indicies because we want 1-indexing.
                        current_number = Some(NumberEntry {
                            value: c.to_digit(10).unwrap(),
                            row: row + 1,
                            col_start: col + 1,
                            col_end: col + 1,
                        })
                    }
                    Some(number) => {
                        // Another digit of the number.  Modify accordingly.
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
                        // Potential gear. Add 1 to indices because we want 1-indexing.
                        asterisks.insert(Coordinate {
                            col: col + 1,
                            row: row + 1,
                        });
                    }
                    if c != '.' {
                        // Symbol. Add 1 to indices because we want 1-indexing.
                        symbols.insert(Coordinate {
                            col: col + 1,
                            row: row + 1,
                        });
                    }
                }
            }
        }
        // If the row finished with a number, save it off now!
        if let Some(n) = current_number.take() {
            numbers.insert(n);
        }
    }

    // For part 1: Find all the numbers that have at least 1 symbol in their neighbours.
    let part_numbers = numbers
        .iter()
        .filter(|&number| {
            number
                .get_neighbor_coords()
                .any(|neighbor| symbols.contains(&neighbor))
        })
        .map(|number| number.value);

    println!("Part 1: {}", part_numbers.sum::<u32>());

    // Find the gears.  These are asterisks who have exactly two neighbouring numbers.
    let gear_ratios = asterisks.iter().filter_map(|this| {
        // Look for neighbouring numbers to work out if this is a gear, and if so what
        // the ratio is.
        // Note that we are getting each number's neighbours and seeing  if they
        // contain our gear, rather than working out neighbours of the gear and
        // seeing which numbers cross them.  This is only because we already had the
        // first method written for part1.
        let neighbors = numbers
            .iter()
            .filter(|n| n.get_neighbor_coords().any(|c| c == *this));
        if neighbors.clone().count() == 2 {
            // This is a gear - asterisk with 2 numeric neighbours - so multiply the
            // values to get the ratio
            Some(neighbors.map(|neigh| neigh.value as u64).product::<u64>())
        } else {
            // Not a gear.
            None
        }
    });
    println!("Part 2: {}", gear_ratios.sum::<u64>());
}
