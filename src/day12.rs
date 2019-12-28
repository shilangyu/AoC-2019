#[derive(Debug, Clone)]
struct Moon {
	id: usize,
	pos: Pos,
	vel: Pos,
}

impl Moon {
	fn new(pos: Pos, id: usize) -> Self {
		Self {
			id,
			pos,
			vel: Pos::default(),
		}
	}

	fn calc_vel(&mut self, others: Vec<Self>) {
		let mut vel = Pos::default();
		let extra = |s, o| {
			if s > o {
				-1
			} else if s == o {
				0
			} else {
				1
			}
		};
		for other in others {
			vel.x += extra(self.pos.x, other.pos.x);
			vel.y += extra(self.pos.y, other.pos.y);
			vel.z += extra(self.pos.z, other.pos.z);
		}
		self.vel = self.vel + vel;
	}

	fn kin(&self) -> i32 {
		self.vel.x.abs() + self.vel.y.abs() + self.vel.z.abs()
	}

	fn pot(&self) -> i32 {
		self.pos.x.abs() + self.pos.y.abs() + self.pos.z.abs()
	}

	fn energy(&self) -> i32 {
		self.pot() * self.kin()
	}
}

#[derive(Default, Debug, Clone, Copy)]
struct Pos {
	x: i32,
	y: i32,
	z: i32,
}

impl std::ops::Add for Pos {
	type Output = Self;

	fn add(self, other: Self) -> Self {
		Self {
			x: self.x + other.x,
			y: self.y + other.y,
			z: self.z + other.z,
		}
	}
}

impl std::ops::Neg for Pos {
	type Output = Self;

	fn neg(self) -> Self {
		Self {
			x: -self.x,
			y: -self.y,
			z: -self.z,
		}
	}
}

impl std::ops::Sub for Pos {
	type Output = Self;

	fn sub(self, other: Self) -> Self {
		Self {
			x: self.x - other.x,
			y: self.y - other.y,
			z: self.z - other.z,
		}
	}
}

pub fn part1() -> i32 {
	let data = std::fs::read_to_string("./inputs/day12.txt").unwrap();
	let mut moons: Vec<Moon> = data
		.split('\n')
		.filter(|s| s.len() > 0)
		.enumerate()
		.map(
			|(i, s)| match scan_fmt!(s, "<x={d}, y={d}, z={d}", i32, i32, i32) {
				Ok((x, y, z)) => Moon::new(Pos { x, y, z }, i),
				Err(err) => panic!("failed to parse file {}", err),
			},
		)
		.collect();

	const STEPS: usize = 1000;

	for _ in 0..STEPS {
		let moons_copy = moons.clone();
		for moon in &mut moons {
			moon.calc_vel(moons_copy.clone());
		}
		for moon in &mut moons {
			moon.pos = moon.vel + moon.pos;
		}
	}

	let mut total_energy = 0;
	for moon in &moons {
		total_energy += moon.energy();
	}

	total_energy
}

pub fn part2() -> i32 {
	// let data = std::fs::read_to_string("./inputs/day12.txt").unwrap();
	// let mut moons: Vec<Moon> = data
	// 	.split('\n')
	// 	.filter(|s| s.len() > 0)
	// 	.enumerate()
	// 	.map(
	// 		|(i, s)| match scan_fmt!(s, "<x={d}, y={d}, z={d}", i32, i32, i32) {
	// 			Ok((x, y, z)) => Moon::new(Pos { x, y, z }, i),
	// 			Err(err) => panic!("failed to parse file {}", err),
	// 		},
	// 	)
	// 	.collect();

	// const STEPS: usize = 1000;

	// for _ in 0..STEPS {
	// 	let moons_copy = moons.clone();
	// 	for moon in &mut moons {
	// 		moon.calc_vel(moons_copy.clone());
	// 	}
	// 	for moon in &mut moons {
	// 		moon.pos = moon.vel + moon.pos;
	// 	}
	// }

	// let mut total_energy = 0;
	// for moon in &moons {
	// 	total_energy += moon.energy();
	// }

	// total_energy
	0
}
