// Part 2 took me longer than it should have done - mainly because I kept trying to
// work out how to mutate the list of Cards as I iterated to update the copy count within them,
// and fighting the borrow checker.
// In the end, switching to tracking copies separately solved that.
// Otherwise, not a lot of interest - a relatively straightforward day I think.

use std::collections::HashSet;

// Parse the whitespace-separated set of numbers.
fn parse_set(line: &str) -> HashSet<u8> {
    line.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

#[derive(Debug, Clone)]
struct Card {
    id: u8,
    matches: u8,
}

impl Card {
    // Calculate the score, which is 0 for no matches, 2^(n-1) for 1 or more.
    // (Don't think there's a single mathematical function to get that...)
    fn part1_score(&self) -> u32 {
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
        // Parsing.
        let (_, numbers) = line.split_once(": ").unwrap();
        let mut numbersets = numbers.split(" | ");
        let winners = parse_set(numbersets.next().unwrap());
        let this_card = parse_set(numbersets.next().unwrap());
        let match_count = winners.intersection(&this_card).count() as u8;
        Card {
            // As in other days, rely on the Card number being line number + 1.
            id: idx as u8 + 1,
            matches: match_count,
        }
    });

    // Part1 - just sum the scores of each card.
    println!(
        "Part 1: {}",
        cards.clone().map(|card| card.part1_score()).sum::<u32>()
    );

    // For part 2, keep a count of copies of each card.
    // Cards are 1-indexed and this list is 0-indexed!
    // We start with 1 copy of each card.
    let mut card_copies = vec![1; num_games];
    for card in cards {
        let this_copies = card_copies[card.id as usize - 1];
        for i in 1..=card.matches {
            // For each copy of *this* card, add a copy to the next card up.
            let next_id = (i + card.id) as usize;
            card_copies[next_id - 1] += this_copies;
        }
    }

    println!("Part 2: {}", card_copies.iter().sum::<u32>());
}
