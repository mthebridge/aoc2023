// Template.
// Copy to daynum.rs, and uncomment relevant lins in main to add

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

pub fn run(input_path: String) {
    let input = std::fs::read_to_string(input_path).unwrap();
    let (dir_input, map_input) = input.split_once("\n\n").unwrap();
    let dirs = dir_input.chars().map(Dir::from_char);

    let map_pattern = regex::Regex::new(r#"([A-Z]+) = \(([A-Z]+), ([A-Z]+)\)"#).unwrap();
    let nodes = map_input
        .lines()
        .map(|l| {
            // dbg!(&map_pattern, &l);
            let matches = map_pattern.captures(l).unwrap();
            let src = matches.get(1).unwrap().as_str();
            let left = matches.get(2).unwrap().as_str();
            let right = matches.get(3).unwrap().as_str();
            (src.to_string(), (left.to_string(), right.to_string()))
        })
        .collect::<HashMap<_, _>>();

    // dbg!(&nodes);
    let mut start = "AAA";
    let mut part1_steps = 0;
    for dir in dirs.cycle() {
        part1_steps += 1;
        let entry = nodes.get(start).unwrap();
        let next = match dir {
            Dir::Left => &entry.0,
            Dir::Right => &entry.1,
        };
        if next == "ZZZ" {
            break;
        } else {
            start = next
        }
    }

    let part2 = 0;

    println!("Part 1: {}", part1_steps);
    println!("Part 2: {}", part2);
}
