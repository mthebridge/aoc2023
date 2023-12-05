// Ooof.
// The classic nice easy part 1, and then a part 2 that needs optimization.
// Part 2 runs in about 3 minutes which is really too slow.

#[derive(Debug, Clone)]
struct MapBucket {
    src_range: (u64, u64),
    dest_range: (u64, u64),
}

impl MapBucket {
    fn from_str(input: &str) -> Self {
        let mut words = input.split(' ');
        let dest_start = words.next().unwrap().parse().unwrap();
        let src_start = words.next().unwrap().parse().unwrap();
        let size: u64 = words.next().unwrap().parse().unwrap();
        MapBucket {
            src_range: (src_start, src_start + size),
            dest_range: (dest_start, dest_start + size),
        }
    }
}

// Apply the mapping.  If the input is within src..src+size, adjust by (dst_start - src_start).
// Otherwise return the input.
fn apply_mapping(input: u64, mapping: &[MapBucket]) -> u64 {
    mapping
        .iter()
        // Get the correct bucket.
        .find(|bucket| input >= bucket.src_range.0 && input < bucket.src_range.1)
        .map_or(input, |bucket| {
            input - bucket.src_range.0 + bucket.dest_range.0
        })
}

const DOUBLE_BLANK_LINE: &str = "\n\n";

// For each seed, work out its final location, and then get the minimum of those.
fn solve(seeds: impl Iterator<Item = u64>, mappings: &[Vec<MapBucket>]) -> u64 {
    seeds
        .map(|seed| {
            // Run the mappings in order.
            mappings.iter().fold(seed, |input, mapping| {
                apply_mapping(input, mapping.as_slice())
            })
        })
        .min()
        .unwrap()
}

pub fn run(input_path: String) {
    let input = std::fs::read_to_string(input_path).unwrap();

    let (seed_str, map_str) = input.split_once(DOUBLE_BLANK_LINE).unwrap();
    let (_, seed_part) = seed_str.split_once(": ").unwrap();
    // Parse the initial seed numbers.
    let seeds = seed_part
        .trim()
        .split(' ')
        .map(|s| s.parse::<u64>().unwrap())
        .collect::<Vec<_>>();

    let mappings = map_str
        .split(DOUBLE_BLANK_LINE)
        .map(|section| {
            // Map each mapping table
            let mut table = section
                .lines()
                // Skip the header, it doesn't contain anything interesting.
                .skip(1)
                // Each line is a single mapping rule.
                .map(MapBucket::from_str)
                .collect::<Vec<_>>();
            table.sort_by_key(|map| map.src_range.0);
            table
        })
        .collect::<Vec<_>>();

    let part1 = solve(seeds.clone().into_iter(), &mappings);
    println!("Part 1: {}", part1);

    let new_seeds = seeds
        .chunks(2)
        .flat_map(|chunk| chunk[0]..chunk[0] + chunk[1]);
    // This is slow...
    let part2 = solve(new_seeds, &mappings);
    println!("Part 2: {}", part2);
}
