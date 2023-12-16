// Build-your-own HashMap. Nice and easy today, very much a reading comprehension task.
// Only issues were not reading the instructions properly.

#[derive(Debug, Clone, PartialEq, Eq)]
struct Lens<'a> {
    label: &'a str,
    focal_length: u8,
}

#[derive(Debug, Clone, Default)]
struct LensBox<'a> {
    lenses: Vec<Lens<'a>>,
}

fn hash_string(input: &str) -> usize {
    input
        .chars()
        .fold(0, |total, c| (17 * (total + c as usize)) % 256)
}

pub fn run(input_path: String) {
    let input = std::fs::read_to_string(input_path).unwrap();

    let part1 = input.trim().split(',').map(hash_string).sum::<usize>();

    println!("Part 1: {}", part1);

    // Follow the instructions...
    let mut boxes: Vec<LensBox> = vec![Default::default(); 256];
    for instr in input.trim().split(',') {
        if instr.contains('=') {
            // Insert lens into box.
            let (label, length) = instr.split_once('=').unwrap();
            let focal_length = length.parse().unwrap();
            let this_box = &mut boxes[hash_string(label)];
            // If already in the box just change the length.
            if let Some(lens) = this_box.lenses.iter_mut().find(|l| l.label == label) {
                lens.focal_length = focal_length
            } else {
                // Add to box.
                this_box.lenses.push(Lens {
                    label,
                    focal_length,
                });
            }
        } else {
            // Remove from box if present.
            debug_assert!(instr.ends_with('-'));
            let label = instr.trim_end_matches('-');
            let this_box = &mut boxes[hash_string(label)];
            if let Some(lens_idx) = this_box.lenses.iter().position(|l| l.label == label) {
                this_box.lenses.remove(lens_idx);
            }
        }
    }
    let part2 = boxes
        .iter()
        .enumerate()
        .flat_map(|(box_id, boxx)| {
            boxx.lenses.iter().enumerate().map(move |(lens_id, lens)| {
                (1 + box_id) * (1 + lens_id) * lens.focal_length as usize
            })
        })
        .sum::<usize>();
    println!("Part 2: {}", part2);
}
