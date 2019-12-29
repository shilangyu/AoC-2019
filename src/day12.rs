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

fn lcm(a: i128, b: i128) -> i128 {
	(a * b) / crate::day10::gcd(a, b)
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

pub fn part2() -> i128 {
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

	let initial = moons.clone();
	let mut cycle = (0, 0, 0);

	let mut i = 1;
	loop {
		i += 1;

		let moons_copy = moons.clone();
		for moon in &mut moons {
			moon.calc_vel(moons_copy.clone());
		}
		for moon in &mut moons {
			moon.pos = moon.vel + moon.pos;
		}

		if cycle.0 == 0
			&& initial
				.iter()
				.enumerate()
				.all(|(i, m)| m.pos.x == moons[i].pos.x && m.vel.x == m.vel.x)
		{
			cycle.0 = i;
		}
		if cycle.1 == 0
			&& initial
				.iter()
				.enumerate()
				.all(|(i, m)| m.pos.y == moons[i].pos.y && m.vel.y == m.vel.y)
		{
			cycle.1 = i;
		}
		if cycle.2 == 0
			&& initial
				.iter()
				.enumerate()
				.all(|(i, m)| m.pos.z == moons[i].pos.z && m.vel.z == m.vel.z)
		{
			cycle.2 = i;
		}
		if cycle.0 * cycle.1 * cycle.2 != 0 {
			break;
		}
	}

	lcm(lcm(cycle.0, cycle.1), cycle.2)
}
