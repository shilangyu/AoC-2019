mod day1;
mod day10;
mod day12;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day8;

#[macro_use]
extern crate scan_fmt;

fn main() {
    println!(
        "day 1:
    part 1: {}
    part 2: {}",
        day1::part1(),
        day1::part2()
    );
    println!(
        "day 2:
    part 1: {}
    part 2: {}",
        day2::part1(),
        day2::part2()
    );
    println!(
        "day 3:
    part 1: {}
    part 2: {}",
        day3::part1(),
        day3::part2()
    );
    println!(
        "day 4:
    part 1: {}
    part 2: {}",
        day4::part1(),
        day4::part2()
    );
    println!(
        "day 5:
    part 1: {}
    part 2: {}",
        day5::part1(),
        day5::part2()
    );
    println!(
        "day 6:
    part 1: {}
    part 2: {}",
        day6::part1(),
        day6::part2()
    );
    println!(
        "day 8:
    part 1: {}
    part 2: {}",
        day8::part1(),
        day8::part2()
    );
    println!(
        "day 10:
    part 1: {}
    part 2: {}",
        day10::part1(),
        day10::part2()
    );
    println!(
        "day 12:
    part 1: {}
    part 2: {}",
        day12::part1(),
        day12::part2()
    );
}
