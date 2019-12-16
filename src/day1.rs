pub fn day1() -> f32 {
	use std::io::BufRead;
	let mut fuel = 0.0;

	for line in std::io::stdin().lock().lines() {
		let mut curr: f32 = line.unwrap().parse().expect("failed to parse into number");
		curr /= 3.0;
		fuel += curr.floor() - 2.0;
	}

	fuel
}
