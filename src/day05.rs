// Ooof.
// The classic nice easy part 1, and then a part 2 that needs optimization.
// Part 2 runs in about 3 minutes which is really too slow.

#[derive(Debug, Clone, Eq, PartialEq)]
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

    fn apply(&self, input: u64) -> u64 {
        input - self.src_range.0 + self.dest_range.0
    }
}

fn is_in_range(n: u64, range: (u64, u64)) -> bool {
    n > range.0 && n < range.1
}

// Apply the mapping, to get all the ranges that this range maps to in the destination.
// If the input is within src..src+size, adjust by (dst_start - src_start).
// Otherwise return the input.
fn apply_mapping_range(
    input_ranges: impl IntoIterator<Item = (u64, u64)>,
    mapping: &[MapBucket],
) -> Vec<(u64, u64)> {
    input_ranges.into_iter().flat_map(|(start, end)| {
        let start_bucket = mapping
            .iter()
            // Get the correct buckets.
            .find(|bucket| is_in_range(start, bucket.src_range));

        let end_bucket = mapping
            .iter()
            // Get the correct buckets.
            .find(|bucket| is_in_range(end, bucket.src_range));

        // Now work out the ranges.
        match (start_bucket, end_bucket) {
            // No buckets match, so just return the input range.
            (None, None) => vec![(start, end)],
            // No bucket for A, bucket for B.  Return the overlap with the b range
            (None, Some(b)) => {
                let second_range = (b.dest_range.0, b.apply(end));
                let first_range = (start, start + second_range.1 - second_range.0);
                vec![first_range, second_range]
            },
            // Bucket for A, no bucket for B.
            (Some(a), None) => {
                let first_range = (a.apply(start), a.dest_range.1, );
                let second_range = (end - (first_range.1 - first_range.0), end);
                vec![first_range, second_range]
            }
            // Two buckets.
            (Some(a), Some(b))  => {
                if a == b {vec![(a.apply(start), b.apply(end))] }
                else {
                    let first_range = (a.apply(start), a.dest_range.1);
                    let third_range = (b.dest_range.0, b.apply(end));
                    let mid_range = (start + (first_range.1 - first_range.0), end - (third_range.1 - third_range.0));
                    vec![first_range, mid_range, third_range]
                }
            },
        }
    }).collect()
}
const DOUBLE_BLANK_LINE: &str = "\n\n";

// For each seed, work out its final location, and then get the minimum of those.
fn solve(seeds: impl Iterator<Item = (u64, u64)>, mappings: &[Vec<MapBucket>]) -> u64 {
    seeds
        .flat_map(|seed_range| {
            // Run the mappings in order.
            mappings.iter().fold(vec![seed_range], |input, mapping| {
                apply_mapping_range(input, &mapping)
            })
        })
        .map(|(a, b)| { println!("Range: {a}-{b}"); a})
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

    // dbg!(&mappings);
    let part1 = solve(seeds.clone().into_iter().map(|s| (s, s)), &mappings);
    println!("Part 1: {}", part1);

    let mut seed_ranges = seeds.chunks(2).map(|chunk| (chunk[0], chunk[0] + chunk[1])).collect::<Vec<_>>();
    seed_ranges.sort_by_key(|range| range.0);
    let part2 = solve(seed_ranges.into_iter(), &mappings);
    println!("Part 2: {}", part2);
}
