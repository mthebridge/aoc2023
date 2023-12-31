// Ooof.
// The classic nice easy part 1, and then a part 2 that needs optimization.
// Originally solved with brute-force (see commit history) - runs in about 3 minutes which is really too slow.
// Took ages to get this right because I had a mental block about how overlapping works, but in the end
// I quite like this.

#[derive(Debug, Clone, Eq, PartialEq)]
struct MapBucket {
    src_range: (u64, u64),
    dest_range: (u64, u64),
}

const DOUBLE_BLANK_LINE: &str = "\n\n";

// Parsing.
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
    n >= range.0 && n < range.1
}

// Apply the mapping, to get all the ranges that this range maps to in the destination.
fn apply_mapping_range(
    input_ranges: impl IntoIterator<Item = (u64, u64)>,
    mapping: &[MapBucket],
) -> Vec<(u64, u64)> {
    input_ranges
        .into_iter()
        .flat_map(|(start, end)| {
            assert!(start <= end);
            // Find all the mappings that overlap this range.
            let overlap_buckets = mapping
                .iter()
                // Any overlap if the end is after the mapping start and the start before the mapping end.
                // Had to draw this out to convince myself!
                .filter(|bucket| end >= bucket.src_range.0 && start < bucket.src_range.1)
                .collect::<Vec<_>>();

            // No overlap so just return the input.
            if overlap_buckets.is_empty() {
                return vec![(start, end)];
            }

            let mut ranges = vec![];
            let mut prev_bucket: Option<&MapBucket> = None;

            // Add any part of the range before and after the overlapping buckets.
            if let Some(b) = overlap_buckets.first() {
                if start < b.src_range.0 {
                    ranges.push((start, b.src_range.0))
                }
            }
            if let Some(b) = overlap_buckets.last() {
                if end > b.src_range.1 {
                    ranges.push((b.src_range.1, end))
                }
            }
            for bucket in &overlap_buckets {
                if let Some(prev) = prev_bucket.take() {
                    // Push the source gap between buckets, if there is a gap
                    if prev.src_range.1 < bucket.src_range.0 {
                        ranges.push((prev.src_range.1, bucket.src_range.0))
                    }
                }
                prev_bucket = Some(bucket);
                match (
                    is_in_range(start, bucket.src_range),
                    is_in_range(end, bucket.src_range),
                ) {
                    // All in one bucket.  Only include the mapped range.
                    (true, true) => ranges.push((bucket.apply(start), bucket.apply(end))),
                    // Must be first bucket.  Just push the appropriate mapped section.
                    (true, false) => ranges.push((bucket.apply(start), bucket.dest_range.1)),
                    // Must be last bucket.  Just push the appropriate mapped section.
                    (false, true) => ranges.push((bucket.dest_range.0, bucket.apply(end))),
                    // Neither the start nor the end are in this bucket but it overlaps.
                    // Push the whole destination range
                    (false, false) => ranges.push(bucket.dest_range),
                };
            }
            ranges
        })
        .collect()
}

// For each seed, work out its final location, and then get the minimum of those.
fn solve(seeds: impl Iterator<Item = (u64, u64)>, mappings: &[Vec<MapBucket>]) -> u64 {
    let final_ranges = seeds
        .flat_map(|seed_range| {
            // Run the mappings in order.
            mappings.iter().fold(vec![seed_range], |input, mapping| {
                apply_mapping_range(input, mapping)
            })
        })
        .collect::<Vec<_>>();
    final_ranges.into_iter().map(|(a, _)| a).min().unwrap()
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

    // Treat each seed a a single-element range, to reuse the part 2 code.
    let part1 = solve(seeds.clone().into_iter().map(|s| (s, s)), &mappings);
    println!("Part 1: {}", part1);

    let seed_ranges = seeds.chunks(2).map(|chunk| (chunk[0], chunk[0] + chunk[1]));
    let part2 = solve(seed_ranges, &mappings);
    println!("Part 2: {}", part2);
}
