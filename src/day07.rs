// Day 7.
// Remember to read the question!

use std::cmp::{Ord, Ordering, PartialOrd};

#[derive(Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq)]
enum Card {
    Joker,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

impl Card {
    fn from_char(c: &char) -> Self {
        match *c {
            '2' => Card::Two,
            '3' => Card::Three,
            '4' => Card::Four,
            '5' => Card::Five,
            '6' => Card::Six,
            '7' => Card::Seven,
            '8' => Card::Eight,
            '9' => Card::Nine,
            'T' => Card::Ten,
            'J' => Card::Jack,
            'Q' => Card::Queen,
            'K' => Card::King,
            'A' => Card::Ace,
            _ => panic!("invalid card"),
        }
    }
}

// We store the list of cards and types separately for
// part1 and part2.  This is so "J" can have a different value in each
// case.
#[derive(Debug, Clone, PartialEq, Eq)]
struct Hand {
    cards_pt1: Vec<Card>,
    cards_pt2: Vec<Card>,
    hand_type_pt1: HandType,
    hand_type_pt2: HandType,
    bid: usize,
}

#[derive(Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq)]
enum HandType {
    HighCard,
    Pair,
    TwoPair,
    ThreeKind,
    FullHouse,
    FourKind,
    FiveKind,
}

// Get a list of (card type, count) for a hand.
fn get_card_counts(cards: &[Card]) -> Vec<(Card, u8)> {
    cards.iter().fold(vec![], |mut set, this| {
        let cur_count = match set.iter().find(|(c, _)| c == this) {
            Some((_, count)) => count + 1,
            None => 1u8,
        };
        set.retain(|x| x.0 != *this);
        set.push((*this, cur_count));
        set
    })
}

impl Hand {
    // Parse the hand.
    fn from_str(line: &str) -> Self {
        let (card_str, bid_str) = line.split_once(' ').unwrap();
        let bid = bid_str.parse().unwrap();
        let cards_pt1 = card_str
            .chars()
            .map(|c| Card::from_char(&c))
            .collect::<Vec<_>>();
        let card_counts_pt1 = get_card_counts(&cards_pt1);

        // For part2, replace all the Jacks with Jokers.
        let mut cards_pt2 = cards_pt1.clone();
        for card in &mut cards_pt2 {
            if *card == Card::Jack {
                *card = Card::Joker
            }
        }
        let mut card_counts_pt2 = get_card_counts(&cards_pt2);

        // Determine rank.
        // Work out the most common card type in each hand.  Tie
        let mut top_card_count = card_counts_pt2.iter().map(|card| card.1).max().unwrap();
        let hand_type_pt1 = get_hand_type(&card_counts_pt1, top_card_count);
        // For part 2, replace any J instances with whatever there are most of.
        let hand_type_pt2 = if let Some(pos) = card_counts_pt2
            .iter()
            .position(|&(card, _)| card == Card::Joker)
        {
            let jokers = card_counts_pt2.remove(pos);
            if jokers.1 == 5 {
                // Special case the 5-Joker hand
                HandType::FiveKind
            } else {
                {
                    let (_, ref mut top_count) = card_counts_pt2
                        .iter_mut()
                        .max_by_key(|(_, count)| *count)
                        .unwrap();
                    *top_count += jokers.1;
                    top_card_count = *top_count;
                }
                get_hand_type(&card_counts_pt2, top_card_count)
            }
        } else {
            hand_type_pt1
        };

        Hand {
            cards_pt1,
            cards_pt2,
            bid,
            hand_type_pt1,
            hand_type_pt2,
        }
    }
}

fn get_hand_type(card_counts: &[(Card, u8)], top_card_count: u8) -> HandType {
    match card_counts.len() {
        1 => HandType::FiveKind,
        2 if top_card_count == 4 => HandType::FourKind,
        2 if top_card_count == 3 => HandType::FullHouse,
        3 if top_card_count == 3 => HandType::ThreeKind,
        3 if top_card_count == 2 => HandType::TwoPair,
        4 => HandType::Pair,
        5 => HandType::HighCard,
        n => panic!("Invalid hand {n}"),
    }
}

impl Hand {
    fn sort(&self, other: &Hand, part2: bool) -> Ordering {
        let (self_type, other_type) = if part2 {
            (self.hand_type_pt2, other.hand_type_pt2)
        } else {
            (self.hand_type_pt1, other.hand_type_pt1)
        };
        if self_type < other_type {
            Ordering::Less
        } else if self_type > other_type {
            Ordering::Greater
        } else {
            // Same rank.  Compare card values
            let (self_cards, other_cards) = if part2 {
                (&self.cards_pt2, &other.cards_pt2)
            } else {
                (&self.cards_pt1, &other.cards_pt1)
            };
            for (a, b) in self_cards.iter().zip(other_cards) {
                if a < b {
                    return Ordering::Less;
                } else if a > b {
                    return Ordering::Greater;
                } else { // Try next card }
                }
            }
            Ordering::Equal
        }
    }
}

fn solve(hands: &[Hand]) -> usize {
    hands
        .iter()
        .enumerate()
        .map(|(i, hand)| hand.bid * (i + 1))
        .sum::<usize>()
}

pub fn run(input_path: String) {
    let input = std::fs::read_to_string(input_path).unwrap();
    let mut hands = input.lines().map(Hand::from_str).collect::<Vec<_>>();
    let mut hands2 = hands.clone();
    // Now sort the hands.
    hands.sort_by(|left, right| left.sort(right, false));
    hands2.sort_by(|left, right| left.sort(right, true));

    let part1 = solve(&hands);
    let part2 = solve(&hands2);

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}
