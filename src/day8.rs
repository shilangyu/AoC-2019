pub fn part1() -> usize {
	let data = std::fs::read_to_string("./inputs/day8.txt").unwrap();
	let data: Vec<_> = data
		.trim()
		.chars()
		.map(|c| c.to_digit(10).unwrap())
		.collect();
	const WIDTH: usize = 25;
	const HEIGHT: usize = 6;

	let mut layers: Vec<Vec<Vec<usize>>> = vec![vec![vec![]]];
	let mut zeros = vec![0];
	let mut ones = vec![0];
	let mut twos = vec![0];

	for c in data {
		let curr = layers.last_mut().unwrap();

		if curr.last().unwrap().len() == WIDTH {
			if curr.len() == HEIGHT {
				drop(curr);
				layers.push(vec![vec![]]);
				zeros.push(0);
				ones.push(0);
				twos.push(0);
			} else {
				curr.push(vec![]);
			}
		}

		let index = zeros.len() - 1;
		match c {
			0 => {
				zeros[index] += 1;
			}
			1 => {
				ones[index] += 1;
			}
			2 => {
				twos[index] += 1;
			}
			_ => (),
		}

		layers
			.last_mut()
			.unwrap()
			.last_mut()
			.unwrap()
			.push(c as usize);
	}

	let least_index = zeros
		.iter()
		.position(|c| c == zeros.iter().min().unwrap())
		.unwrap();
	ones[least_index] * twos[least_index]
}

pub fn part2() -> String {
	let data = std::fs::read_to_string("./inputs/day8.txt").unwrap();
	let data: Vec<_> = data
		.trim()
		.chars()
		.map(|c| c.to_digit(10).unwrap())
		.collect();
	const WIDTH: usize = 25;
	const HEIGHT: usize = 6;

	let mut layers: Vec<Vec<Vec<usize>>> = vec![vec![vec![]]];

	for c in data {
		let curr = layers.last_mut().unwrap();

		if curr.last().unwrap().len() == WIDTH {
			if curr.len() == HEIGHT {
				drop(curr);
				layers.push(vec![vec![]]);
			} else {
				curr.push(vec![]);
			}
		}

		layers
			.last_mut()
			.unwrap()
			.last_mut()
			.unwrap()
			.push(c as usize);
	}

	let mut image = String::from("\n");
	for i in 0..HEIGHT {
		for j in 0..WIDTH {
			for layer in &layers {
				if layer[i][j] != 2 {
					if layer[i][j] == 1 {
						image.push('█');
					} else {
						image.push(' ');
					}
					break;
				}
			}
		}
		image.push_str("\n");
	}

	image
}
