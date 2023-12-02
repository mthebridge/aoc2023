// Arguably easier than yesterday - only issues were not reading the question!
// Parsing is the only fiddly bit - I almost reached for regex, but the need for nesting across
// the various draws for each game felt fiddly, and I quite like the string-split-and-iterator
// approach when you now the input is well-formed and can just unwrap eveyrwhere.

// A set of numbers read from the bag.
#[derive(Debug, Clone, Eq, PartialEq, Default)]
struct ShowResult {
    blue: u16,
    red: u16,
    green: u16,
}

struct Game {
    number: usize,
    results: Vec<ShowResult>,
}

impl Game {
    fn from_string(index: usize, line: &str) -> Self {
        // Format is: Game [num]: [num] [color], ...;  [num] [color], ...; ...
        // We don't need to parse the game number - they are in ascending order
        let (_, results) = line.split_once(':').unwrap();
        let results = results
            .split(';')
            .map(|res_line| {
                let mut result: ShowResult = Default::default();
                for entry in res_line.split(',') {
                    // Should have something of the form {num} {color}
                    let entry = entry.trim();
                    let (num, color) = entry.split_once(' ').unwrap();
                    let value = num.parse::<u16>().unwrap();
                    match color {
                        "red" => result.red = value,
                        "blue" => result.blue = value,
                        "green" => result.green = value,
                        _ => panic!("Invalid color"),
                    }
                }
                result
            })
            .collect();

        Game {
            number: index,
            results,
        }
    }

    // Get the power value of a game.
    // The "power" is the product of the minumum possible cubes, which
    // is the *largest* of each of the red, blue and green seen across
    // each show in the game.
    fn get_power(&self) -> u32 {
        // Get the total for each game.
        let min_red = self
            .results
            .iter()
            .max_by_key(|&r| r.red)
            .map_or(0, |r| r.red) as u32;
        let min_blue = self
            .results
            .iter()
            .max_by_key(|&r| r.blue)
            .map_or(0, |r| r.blue) as u32;
        let min_green = self
            .results
            .iter()
            .max_by_key(|&r| r.green)
            .map_or(0, |r| r.green) as u32;

        min_red * min_blue * min_green
    }
}

pub fn run(input_path: String) {
    let input = std::fs::read_to_string(input_path).unwrap();

    let games = input
        .lines()
        .enumerate()
        // Use the array index + 1 as the game index, which is a valid assumption.
        .map(|(idx, line)| Game::from_string(idx + 1, line));

    // Count games where *every* result in the game has at most 12 red, 13 green and 14 blue
    let part1 = games
        .clone() // Needed to reuse for part2.
        .filter(|game| {
            game.results
                .iter()
                .all(|res| res.red <= 12 && res.green <= 13 && res.blue <= 14)
        })
        .map(|game| game.number)
        .sum::<usize>();

    // Sum the powers.
    let part2 = games.map(|g| g.get_power()).sum::<u32>();

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}
