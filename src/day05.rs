// Template.
// Copy to daynum.rs, and uncomment relevant lins in main to add

use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone)]
struct MapBucket {
    src_start: u64,
    dest_start: u64,
    size: u64,
}

impl MapBucket {
    fn from_str(input: &str) -> Self {
        let mut words = input.split(' ');
        let dest_start = words.next().unwrap().parse().unwrap();
        let src_start = words.next().unwrap().parse().unwrap();
        let size = words.next().unwrap().parse().unwrap();
        MapBucket {
            src_start,
            dest_start,
            size,
        }
    }
}

fn apply_mapping(input: u64, mapping: &[MapBucket]) -> u64 {
    mapping
        .iter()
        // Get the correct bucket.  Will only be one.
        .filter(|bucket| input >= bucket.src_start && input < (bucket.src_start + bucket.size))
        .next()
        .map_or(input, |bucket| input - bucket.src_start + bucket.dest_start)
}

const DOUBLE_BLANK_LINE: &str = "\n\n";

fn find_answer(seeds: impl Iterator<Item = u64>, mappings: &Vec<Vec<MapBucket>>) -> u64 {
    let mut cache = HashMap::new();
    seeds
        .map(|seed| match cache.get(&seed) {
            Some(x) => *x,
            None => {
                let result = mappings.iter().fold(seed, |input, mapping| {
                    apply_mapping(input, mapping.as_slice())
                });
                cache.insert(seed, result);
                result
            }
        })
        .min()
        .unwrap()
}

pub fn run(input_path: String) {
    let input = std::fs::read_to_string(input_path).unwrap();

    let (seed_str, map_str) = input.split_once(DOUBLE_BLANK_LINE).unwrap();
    let (_, seed_part) = seed_str.split_once(": ").unwrap();
    let seeds = seed_part
        .trim()
        .split(' ')
        .map(|s| s.parse::<u64>().unwrap())
        .collect::<Vec<_>>();

    let mappings = map_str
        .split(DOUBLE_BLANK_LINE)
        .map(|section| {
            section
                .lines()
                // Skip the header
                .skip(1)
                .map(MapBucket::from_str)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let part1 = find_answer(seeds.clone().into_iter(), &mappings);
    println!("Part 1: {}", part1);

    let new_seeds = seeds
        .chunks(2)
        .map(|chunk| chunk[0]..chunk[0] + chunk[1])
        .flatten();
    let part2 = find_answer(new_seeds, &mappings);
    println!("Part 2: {}", part2);
}
