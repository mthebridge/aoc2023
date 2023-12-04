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

#[derive(Debug, Clone)]
struct Card {
    id: u8,
    matches: u8,
}

impl Card {
    fn part1_score(&self) -> u32 {
        // score is 0 for no matches, 2^(n-1) for 1 or more.
        if self.matches == 0 {
            0
        } else {
            2u32.pow(self.matches as u32 - 1)
        }
    }
}

pub fn run(input_path: String) {
    let input = std::fs::read_to_string(input_path).unwrap();
    let num_games = input.lines().count();

    let cards = input.lines().enumerate().map(|(idx, line)| {
        let (_, numbers) = line.split_once(": ").unwrap();
        let mut numbersets = numbers.split(" | ");
        let winners = parse_set(numbersets.next().unwrap());
        let this_card = parse_set(numbersets.next().unwrap());
        let match_count = winners.intersection(&this_card).count() as u8;
        Card {
            id: idx as u8 + 1,
            matches: match_count,
        }
    });
    println!(
        "Part 1: {}",
        cards.clone().map(|card| card.part1_score()).sum::<u32>()
    );

    let mut card_list: Vec<Card> = cards.collect();
    card_list.sort_by_key(|c| c.id);
    let mut card_copies = vec![1; num_games];

    for card in &card_list {
        let this_copies = card_copies[card.id as usize - 1];
        for i in 0..card.matches {
            // Add a copy of the next card up.
            let next_id = (i + card.id) as usize;
            card_copies[next_id] += this_copies;
        }
    }

    println!("Part 2: {}", card_copies.iter().sum::<u32>());
}
