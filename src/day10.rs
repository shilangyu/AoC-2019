fn simplify(ab: (i32, i32)) -> (i32, i32) {
	let mut cp_a = ab.0;
	let mut cp_b = ab.1;
	while cp_a != 0 {
		let rem = cp_b % cp_a;
		cp_b = cp_a;
		cp_a = rem;
	}
	let gcd = if cp_b < 0 { cp_b * -1 } else { cp_b };

	return (ab.0 / gcd, ab.1 / gcd);
}

fn angle(pos: (i32, i32)) -> f64 {
	let dot: f64 = (pos.0 * 0 + pos.1 * -1) as f64;
	let det: f64 = (pos.0 * -1 - pos.1 * 0) as f64;
	let res = det.atan2(dot);
	if pos.0 == 0 && pos.1 < 0 {
		return 0.0;
	}
	if res < 0.0 {
		return res * -1.0;
	}
	(res - std::f64::consts::PI * 2.0) * -1.0
}

pub fn part1() -> usize {
	let data = std::fs::read_to_string("./inputs/day10.txt").unwrap();
	let data: Vec<Vec<_>> = data
		.trim()
		.split("\n")
		.map(|s| s.chars().map(|c| c == '#').collect())
		.collect();

	let mut best = 0;

	for i in 0..data.len() {
		for j in 0..data[i].len() {
			if !data[i][j] {
				continue;
			}
			let mut counter = 0;
			for x in 0..data.len() {
				for y in 0..data[i].len() {
					let vision = ((x as i32 - j as i32), (y as i32 - i as i32));
					if vision == (0, 0) {
						continue;
					}
					if simplify(vision) == vision {
						let mut iter = 1;
						loop {
							match data.get((i as i32 + vision.1 * iter) as usize) {
								Some(v) => match v.get((j as i32 + vision.0 * iter) as usize) {
									Some(b) => {
										if *b {
											counter += 1;
											break;
										}
										iter += 1;
									}
									None => break,
								},
								None => break,
							}
						}
					}
				}
			}
			best = std::cmp::max(counter, best);
		}
	}

	best
}

pub fn part2() -> i32 {
	let data = std::fs::read_to_string("./inputs/day10.txt").unwrap();
	let data: Vec<Vec<_>> = data
		.trim()
		.split("\n")
		.map(|s| s.chars().map(|c| c == '#').collect())
		.collect();

	let mut best: Vec<(i32, i32)> = vec![];
	let mut best_pos: (i32, i32) = (-1, -1);

	for i in 0..data.len() {
		for j in 0..data[i].len() {
			if !data[i][j] {
				continue;
			}
			let mut curr: Vec<(i32, i32)> = vec![];
			for x in 0..data.len() {
				for y in 0..data[i].len() {
					let vision = ((x as i32 - j as i32), (y as i32 - i as i32));
					if vision == (0, 0) {
						continue;
					}
					if simplify(vision) == vision {
						let mut iter = 1;
						loop {
							match data.get((i as i32 + vision.1 * iter) as usize) {
								Some(v) => match v.get((j as i32 + vision.0 * iter) as usize) {
									Some(b) => {
										if *b {
											curr.push((vision.0 * iter, vision.1 * iter));
											break;
										}
										iter += 1;
									}
									None => break,
								},
								None => break,
							}
						}
					}
				}
			}
			if curr.len() > best.len() {
				best = curr;
				best_pos = (j as i32, i as i32);
			}
		}
	}

	best.sort_by(|a, b| angle(*a).partial_cmp(&angle(*b)).unwrap());

	if best.len() < 200 {
		panic!("whoops! didnt take that into account");
	}
	(best[199].0 + best_pos.0) * 100 + (best[199].1 + best_pos.1)
}
