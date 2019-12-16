use std::fs;

pub fn part1() -> usize {
	let mut data: Vec<usize> = fs::read_to_string("./inputs/day2.txt")
		.unwrap()
		.trim()
		.split(',')
		.map(|s| s.parse().expect("failed to parse file"))
		.collect();

	data[1] = 12;
	data[2] = 2;
	let mut i = 0;
	loop {
		match data[i] {
			1 => {
				let index = data[i + 3];
				data[index] = data[data[i + 1]] + data[data[i + 2]];
			}
			2 => {
				let index = data[i + 3];
				data[index] = data[data[i + 1]] * data[data[i + 2]]
			}
			99 => break,
			_ => panic!("unknown command"),
		}
		i += 4;
	}
	data[0]
}

pub fn part2() -> usize {
	let initial_data: Vec<usize> = fs::read_to_string("./inputs/day2.txt")
		.unwrap()
		.trim()
		.split(',')
		.map(|s| s.parse().expect("failed to parse file"))
		.collect();
	const GOAL: usize = 19690720;

	let mut noun = 0;
	let mut verb = 0;
	for i in 0..=100 {
		for j in 0..=100 {
			let mut data = initial_data.clone();
			noun = i;
			verb = j;
			data[1] = noun;
			data[2] = verb;

			let mut i = 0;
			loop {
				match data[i] {
					1 => {
						let index = data[i + 3];
						data[index] = data[data[i + 1]] + data[data[i + 2]];
					}
					2 => {
						let index = data[i + 3];
						data[index] = data[data[i + 1]] * data[data[i + 2]]
					}
					99 => break,
					_ => panic!("unknown command"),
				}
				i += 4;
			}

			if data[0] == GOAL {
				return 100 * noun + verb;
			}
		}
	}
	100 * noun + verb
}
