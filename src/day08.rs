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

fn solve<I: IntoIterator<Item = Dir> + Clone>(dirs: I, nodes: &HashMap<String, (String, String)>, part2: bool) -> u32
    where <I as IntoIterator>::IntoIter: Clone {
    let mut steps = 0;
    let mut current_nodes = if part2 {
        nodes.keys().filter(|k| k.ends_with('A')).map(|k| k.as_str()).collect::<Vec<_>>()
    } else {
        vec!["AAA"]
    };
    let targets = if part2 {
        nodes.keys().filter(|k| k.ends_with('Z')).map(|k| k.as_str()).collect::<Vec<_>>()
    } else {
        vec!["ZZZ"]
    };
    for dir in dirs.into_iter().cycle() {
        steps += 1;
        let mut nexts  = current_nodes.into_iter().map(|current| {
            let entry = nodes.get(current).unwrap();
            match dir {
                Dir::Left => entry.0.as_str(),
                Dir::Right => entry.1.as_str(),
            }
        });
        if nexts.all(|next| targets.iter().find(|&&target| target == next).is_some()) {
            break;
        } else {
            current_nodes = nexts.collect()
        }
    }

    steps
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
    let part1 = solve(dirs.clone(), &nodes, false);
    let part2 = solve(dirs, &nodes, true);

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}
