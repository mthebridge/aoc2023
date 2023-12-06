mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
// mod day08;
// mod day09;
// mod day10;
// mod day11;
// mod day12;
// mod day13;
// mod day14;
// mod day15
// mod day16;
// mod day17;
// mod day18;
// mod day19;
// mod day20;
// mod day21;
// mod day22;
// mod day23;
// mod day24;
// mod day25;

fn main() -> Result<(), String> {
    let day_num: u16 = std::env::args()
        .nth(1)
        .expect("Must pass day number as first argument")
        .parse::<u16>()
        .map_err(|_| "Must pass numeric argument".to_string())?;
    let input_path = std::env::args()
        .nth(2)
        .expect("Must pass input file as second argument");
    match day_num {
        1 => day01::run(input_path),
        2 => day02::run(input_path),
        3 => day03::run(input_path),
        4 => day04::run(input_path),
        5 => day05::run(input_path),
        6 => day06::run(input_path),
        7 => day07::run(input_path),
        // 8 => day08::run(input_path),
        // 9 => day09::run(input_path),
        // 10 => day10::run(input_path),
        // 11 => day11::run(input_path),
        // 12 => day12::run(input_path),
        // 13 => day13::run(input_path),
        // 14 => day14::run(input_path),
        // 15 => day15::run(input_path),
        // 16 => day16::run(input_path),
        // 17 => day17::run(input_path),
        // 18 => day18::run(input_path),
        // 19 => day19::run(input_path),
        // 20 => day10::run(input_path),
        // 21 => day21::run(input_path),
        // 22 => day22::run(input_path),
        // 23 => day23::run(input_path),
        // 24 => day24::run(input_path),
        // 25 => day25::run(input_path),
        _ => panic!("Day not implemented"),
    }

    Ok(())
}
