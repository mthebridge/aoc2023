// Day 7.
// Remember to read the question!

use std::cmp::{Ord, Ordering, PartialOrd};

#[derive(Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq)]
enum Card {
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

#[derive(Debug, Clone, PartialEq, Eq)]
struct Hand {
    cards: Vec<Card>,
    hand_type: HandType,
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

impl Hand {
    fn from_str(line: &str) -> Self {
        let (card_str, bid_str) = line.split_once(' ').unwrap();
        let bid = bid_str.parse().unwrap();
        let cards = card_str
            .chars()
            .map(|c| Card::from_char(&c))
            .collect::<Vec<_>>();
        let card_counts = cards.iter().fold(vec![], |mut set, this| {
            let cur_count = match set.iter().find(|(c, _)| *c == this) {
                Some((_, count)) => count + 1,
                None => 1,
            };
            set.retain(|x| x.0 != this);
            set.push((this, cur_count));
            set
        });

        // Determine rank. How many different types of card do we have?
        let top_card_count = card_counts.iter().map(|card| card.1).max().unwrap();
        let hand_type = match card_counts.len() {
            1 => HandType::FiveKind,
            2 if top_card_count == 4 => HandType::FourKind,
            2 if top_card_count == 3 => HandType::FullHouse,
            3 if top_card_count == 3 => HandType::ThreeKind,
            3 if top_card_count == 2 => HandType::TwoPair,
            4 => HandType::Pair,
            5 => HandType::HighCard,
            _ => panic!("Invalid hand"),
        };

        Hand {
            cards,
            bid,
            hand_type,
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        // dbg!(&self, &other);
        if self.hand_type < other.hand_type {
            Some(Ordering::Less)
        } else if self.hand_type > other.hand_type {
            Some(Ordering::Greater)
        } else {
            // Same rank.  Compare card values
            for (a, b) in self.cards.iter().zip(&other.cards) {
                if a < b {
                    return Some(Ordering::Less);
                } else if a > b {
                    return Some(Ordering::Greater);
                } else { // Try next card }
                }
            }
            Some(Ordering::Equal)
        }
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

pub fn run(input_path: String) {
    let input = std::fs::read_to_string(input_path).unwrap();
    let mut hands = input.lines().map(Hand::from_str).collect::<Vec<_>>();

    // Now sort the hands.
    hands.sort();

    let part1 = hands
        .iter()
        .enumerate()
        .map(|(i, hand)| hand.bid * (i + 1))
        .sum::<usize>();
    let part2 = 0;

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}
