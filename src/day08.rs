// Another "optimize perf for part 2".  Part 1 took me 10 minutes; part 2
// over an hour...
// There's a lot of assumptions here, namely that the answer is
// a multiple of the number of steps needed to reach a target for each ghost, and that
// they all cycle nicely, which I only added proof for after the fact -
// more details in the inline comments.
// Thanks to my colleagues who shared insights that helped me find this solution and get
// over the line!

use std::collections::HashMap;

#[derive(Clone, Debug, Copy)]
enum Dir {
    Left,
    Right,
}

impl Dir {
    fn from_char(c: char) -> Self {
        match c {
            'L' => Dir::Left,
            'R' => Dir::Right,
            _ => panic!("Invalid char"),
        }
    }
}

fn solve<I: IntoIterator<Item = Dir> + Clone>(
    dirs: I,
    nodes: &HashMap<String, (String, String)>,
    part2: bool,
) -> u64
where
    <I as IntoIterator>::IntoIter: Clone,
{
    // Note down the length of the direction cycle.
    let dir_count = dirs.clone().into_iter().count() as u64;
    // Part 1 wants route from AAA -> ZZZ
    // Part 2 wants routes from all ??A to any ??Z
    let (start_nodes, targets) = if part2 {
        (
            nodes
                .keys()
                .filter(|k| k.ends_with('A'))
                .map(|k| k.as_str())
                .collect::<Vec<_>>(),
            nodes
                .keys()
                .filter(|k| k.ends_with('Z'))
                .map(|k| k.as_str())
                .collect::<Vec<_>>(),
        )
    } else {
        (vec!["AAA"], vec!["ZZZ"])
    };

    // For each source, count the steps to reach a target.
    // We want the minimum steps for all the targets to be reached at once.
    // It turns out with our inputs this is the lowest common multiple of
    // the individual steps, and the input direction cycle length.
    // This only works as the next step after each target matches the step from the source.
    // The code asserts this is the case by looping through again to reach the next target.
    //
    // If this weren't true, we'd need to keep track of how long it takes to get back
    // to another target, and the position modulo length of direction lists, and do some
    // modular arithmetic fun...
    let steps = start_nodes.iter().map(|&start| {
        let mut steps = 0u64;
        let mut first_target = None;
        let mut current = start;
        for dir in dirs.clone().into_iter().cycle() {
            steps += 1;

            let entry = nodes.get(current).unwrap();
            let next = match dir {
                Dir::Left => entry.0.as_str(),
                Dir::Right => entry.1.as_str(),
            };

            if let Some(t) = targets.iter().find(|&&target| target == next) {
                if let Some((loc, count)) = first_target {
                    // Reached a target again - check it's the same one so the cycle
                    // assumptions hold.
                    assert_eq!(loc, t);
                    assert_eq!(steps, count);
                    break;
                } else {
                    // Reached the target.  Save off the name and count, and keep going.
                    first_target = Some((t, steps));
                    steps = 0;
                }
            }

            current = next;
        }
        steps
    });

    // Using the `num` crate as I can't be bothered to implement Euclid's algorithm myself.
    steps.fold(dir_count, |total, to_target| {
        num::integer::lcm(total, to_target)
    })
}

pub fn run(input_path: String) {
    let input = std::fs::read_to_string(input_path).unwrap();
    let (dir_input, map_input) = input.split_once("\n\n").unwrap();
    let dirs = dir_input.chars().map(Dir::from_char);

    // Regex cos life's too short.
    let map_pattern = regex::Regex::new(r#"([A-Z1-9]+) = \(([A-Z1-9]+), ([A-Z1-9]+)\)"#).unwrap();
    let nodes = map_input
        .lines()
        .map(|l| {
            let matches = map_pattern.captures(l).unwrap();
            let src = matches.get(1).unwrap().as_str();
            let left = matches.get(2).unwrap().as_str();
            let right = matches.get(3).unwrap().as_str();
            (src.to_string(), (left.to_string(), right.to_string()))
        })
        .collect::<HashMap<_, _>>();

    let part1 = solve(dirs.clone(), &nodes, false);
    println!("Part 1: {}", part1);

    let part2 = solve(dirs, &nodes, true);
    println!("Part 2: {}", part2);
}
