pub fn part1() -> i32 {
	let range = 125730..=579381;
	let range: Vec<String> = range
		.map(|num| num.to_string().chars().rev().collect())
		.collect();

	let mut amount = 0;
	for num in range {
		let mut prev = num.chars().nth(0).unwrap().to_digit(10).unwrap();
		let mut inc = true;
		let mut dbl = false;
		for dig in num[1..].chars() {
			let curr = dig.to_digit(10).unwrap();
			if prev < curr {
				inc = false;
				break;
			}
			if prev == curr {
				dbl = true;
			}

			prev = curr;
		}

		if inc && dbl {
			amount += 1;
		}
	}

	amount
}

pub fn part2() -> i32 {
	let range = 125730..=579381;
	let range: Vec<String> = range
		.map(|num| num.to_string().chars().rev().collect())
		.collect();

	let mut amount = 0;
	for num in range {
		let mut prev = num.chars().nth(0).unwrap().to_digit(10).unwrap();
		let mut inc = true;
		let mut dbl = false;
		let mut curr_streak = 1;
		for dig in num[1..].chars() {
			let curr = dig.to_digit(10).unwrap();
			if prev < curr {
				inc = false;
				break;
			}
			if prev == curr {
				curr_streak += 1;
			} else {
				if curr_streak == 2 {
					dbl = true;
				}
				curr_streak = 1;
			}

			prev = curr;
		}

		if curr_streak == 2 {
			dbl = true;
		}

		if inc && dbl {
			amount += 1;
		}
	}

	amount
}
