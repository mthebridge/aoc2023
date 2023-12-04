// Template.
// Copy to daynum.rs, and uncomment relevant lins in main to add

use std::collections::HashSet;

type NumberSet = HashSet<u8>;

fn parse_set(line: &str) -> NumberSet {
    line.trim()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

pub fn run(input_path: String) {
    let input = std::fs::read_to_string(input_path).unwrap();

    let match_scores = input.lines().map(|line| {
        let (game, numbers) = line.split_once(": ").unwrap();
        let _game_num = game
            .split_once(' ')
            .unwrap()
            .1
            .trim()
            .parse::<u8>()
            .unwrap();
        let mut numbersets = numbers.split(" | ");
        let winners = parse_set(numbersets.next().unwrap());
        let this_card = parse_set(numbersets.next().unwrap());
        let match_count = winners.intersection(&this_card).count() as u32;
        // score is 0 for no matches, 2^(n-1) for 1 or more.
        if match_count == 0 {
            0
        } else {
            2u32.pow(match_count - 1)
        }
    });
    println!("Part 1: {}", match_scores.sum::<u32>());
    let part2 = 0;
    println!("Part 2: {}", part2);
}
