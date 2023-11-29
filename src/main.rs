mod day01;
// mod day02;
// mod day03;
// mod day04;
// mod day05;
// mod day06;
// mod day07;
// mod day08;
// mod day09;
// mod day10;
// mod day11;
// mod day12;
// mod day13;
// mod day14;
// mod day15;

fn main() -> Result<(), String> {
    let day_num: u16 = std::env::args()
        .nth(1)
        .expect("No argument provided")
        .parse::<u16>()
        .map_err(|_| "Must pass numeric argument".to_string())?;
    match day_num {
        1 => day01::run(),
        // 2 => day02::run(),
        // 3 => day03::run(),
        // 4 => day04::run(),
        // 5 => day05::run(),
        // 6 => day06::run(),
        // 7 => day07::run(),
        // 8 => day08::run(),
        // 9 => day09::run(),
        // 10 => day10::run(),
        // 11 => day11::run(),
        // 12 => day12::run(),
        // 13 => day13::run(),
        // 14 => day14::run(),
        // 15 => day15::run(),
        _ => panic!("Day not implemented"),
    }

    Ok(())
}
