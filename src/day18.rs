// Thanks to those in day 10 discussions who talked about using Pick's theorem!
// Although the day 10 solution of scanning the grid still works, if you were to
// build up the similar "pipe" layouts there and track corners etc.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    N,
    S,
    E,
    W,
}

impl Direction {
    fn from_str(s: &str) -> Self {
        match s {
            "U" => Direction::N,
            "D" => Direction::S,
            "L" => Direction::W,
            "R" => Direction::E,
            _ => panic!("Invaldi direction"),
        }
    }
}

fn calc_area(data: impl Iterator<Item = (Direction, u64)>) -> u64 {
    let (coords, perimeter) = data.fold(
        (vec![(0, 0)], 0),
        |(mut coords, peri_sum), (dir, distance)| {
            let last = coords.last().unwrap();
            let next = match dir {
                Direction::N => (last.0, last.1 - distance),
                Direction::E => (last.0 + distance, last.1),
                Direction::S => (last.0, last.1 + distance),
                Direction::W => (last.0 - distance, last.1),
            };
            coords.push(next);
            (coords, peri_sum + distance)
        },
    );

    assert_eq!(coords.first(), coords.last());
    // Use the Shoelace formula as per https://en.wikipedia.org/wiki/Shoelace_formula to get total area
    // and then Pick's theorem https://en.wikipedia.org/wiki/Pick%27s_theorem to get the inner points
    let area = coords.windows(2).fold(0, |sum, pair| {
        let a = pair[0];
        let b = pair[1];
        let det = (a.0 * b.1) as i64 - (b.0 * a.1) as i64;
        // println!("({},{}) to ({},{}) = {det}", a.0, a.1, b.0, b.1);
        sum + det
    }) as u64
        / 2;
    // Because we are counting the boundary as having "area" as well, we need to add half the perimeter.
    return area + perimeter / 2 + 1;
}

pub fn run(input_path: String) {
    let input = std::fs::read_to_string(input_path).unwrap();

    let p1_data = input.lines().map(|l| {
        let mut words = l.split_whitespace();
        let dir = words.next().map(Direction::from_str).unwrap();
        let distance = words.next().unwrap().parse::<u64>().unwrap();
        (dir, distance)
    });

    let p2_data = input.lines().map(|l| {
        let mut words = l.split_whitespace();
        let color_word = words
            .nth(2)
            .unwrap()
            .trim_start_matches(&['(', '#'])
            .trim_end_matches(')');
        let distance = u64::from_str_radix(&color_word[0..5], 16).unwrap();
        let dir = match &color_word.chars().nth(5).unwrap() {
            '0' => Direction::E,
            '1' => Direction::S,
            '2' => Direction::W,
            '3' => Direction::N,
            _ => panic!("Invalid direction"),
        };
        (dir, distance)
    });

    let part1 = calc_area(p1_data);
    println!("Part 1: {}", part1);
    let part2 = calc_area(p2_data);
    println!("Part 2: {}", part2);
}
