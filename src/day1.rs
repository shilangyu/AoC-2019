use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn part1() -> i32 {
	let reader = BufReader::new(File::open("./inputs/day1.txt").unwrap());
	let mut fuel = 0;

	for line in reader.lines() {
		let mut mass: i32 = line.unwrap().parse().expect("failed to parse into number");
		mass /= 3;
		fuel += mass - 2;
	}
	fuel
}

pub fn part2() -> i32 {
	let reader = BufReader::new(File::open("./inputs/day1.txt").unwrap());
	let mut fuel = 0;

	const THRESHOLD: i32 = 2 * 4;

	for line in reader.lines() {
		let mut mass: i32 = line.unwrap().parse().expect("failed to parse into number");

		while mass > THRESHOLD {
			mass /= 3;
			mass -= 2;
			fuel += mass;
		}
	}

	fuel
}
