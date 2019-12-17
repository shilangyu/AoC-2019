use std::cmp;
use std::fs;
use std::ops;

#[derive(Clone, Debug)]
struct Point {
	x: i32,
	y: i32,
}

impl Point {
	fn from_dir(dir: &str) -> Point {
		let mut p = Point { x: 0, y: 0 };
		let how_much: i32 = dir[1..].parse().expect("couldnt parse a command");
		match dir.chars().nth(0) {
			Some('U') => p.y += how_much,
			Some('D') => p.y -= how_much,
			Some('R') => p.x += how_much,
			Some('L') => p.x -= how_much,
			_ => panic!("unknown direction"),
		}
		p
	}

	fn manhattan_distance(&self) -> i32 {
		self.x.abs() + self.y.abs()
	}

	fn distance(&self, to: &Point) -> i32 {
		(self.x - to.x).abs() + (self.y - to.y).abs()
	}
}

impl ops::Add for Point {
	type Output = Point;
	fn add(self, other: Point) -> Point {
		Point {
			x: self.x + other.x,
			y: self.y + other.y,
		}
	}
}

struct Line(Point, Point);

impl Line {
	fn intersects(&self, with: &Line) -> Option<Point> {
		if with.0.x == with.1.x && self.1.y == self.0.y {
			if self.0.y < cmp::max(with.0.y, with.1.y)
				&& self.0.y > cmp::min(with.0.y, with.1.y)
				&& cmp::max(self.0.x, self.1.x) > with.0.x
				&& cmp::min(self.0.x, self.1.x) < with.0.x
			{
				return Some(Point {
					x: with.0.x,
					y: self.0.y,
				});
			}
		} else if with.0.y == with.1.y && self.1.x == self.0.x {
			if self.0.x < cmp::max(with.0.x, with.1.x)
				&& self.0.x > cmp::min(with.0.x, with.1.x)
				&& cmp::max(self.0.y, self.1.y) > with.0.y
				&& cmp::min(self.0.y, self.1.y) < with.0.y
			{
				return Some(Point {
					x: self.0.x,
					y: with.0.y,
				});
			}
		}
		None
	}

	fn length(&self) -> i32 {
		self.0.distance(&self.1)
	}
}

pub fn part1() -> i32 {
	let data = fs::read_to_string("./inputs/day3.txt").unwrap();
	let data: Vec<Vec<&str>> = data
		.trim()
		.split('\n')
		.map(|s| s.split(',').collect())
		.collect();

	let mut paths: Vec<Line> = vec![];

	let mut curr_pos = Point { x: 0, y: 0 };
	for dir in &data[0] {
		let prev_pos = curr_pos.clone();
		curr_pos = curr_pos + Point::from_dir(dir);
		paths.push(Line(prev_pos.clone(), curr_pos.clone()))
	}

	let mut cross: Vec<Point> = vec![];
	curr_pos = Point { x: 0, y: 0 };
	for dir in &data[1] {
		let prev_pos = curr_pos.clone();
		curr_pos = curr_pos + Point::from_dir(dir);
		for path in &paths {
			match path.intersects(&Line(prev_pos.clone(), curr_pos.clone())) {
				Some(p) => cross.push(p),
				None => continue,
			}
		}
	}

	let mut best = cross[0].manhattan_distance();
	for p in &cross {
		if p.manhattan_distance() < best {
			best = p.manhattan_distance();
		}
	}

	best
}

pub fn part2() -> i32 {
	let data = fs::read_to_string("./inputs/day3.txt").unwrap();
	let data: Vec<Vec<&str>> = data
		.trim()
		.split('\n')
		.map(|s| s.split(',').collect())
		.collect();

	let mut paths: Vec<Line> = vec![];

	let mut curr_pos = Point { x: 0, y: 0 };
	for dir in &data[0] {
		let prev_pos = curr_pos.clone();
		curr_pos = curr_pos + Point::from_dir(dir);
		paths.push(Line(prev_pos.clone(), curr_pos.clone()))
	}

	curr_pos = Point { x: 0, y: 0 };
	let mut best = -1;
	let mut sum_b = 0;
	for dir in &data[1] {
		let mut sum_a = 0;
		let prev_pos = curr_pos.clone();
		curr_pos = curr_pos + Point::from_dir(dir);
		let curr_line = Line(prev_pos.clone(), curr_pos.clone());
		sum_b += curr_line.length();
		for path in &paths {
			sum_a += path.length();
			match path.intersects(&curr_line) {
				Some(p) => {
					let curr = sum_a + sum_b - path.1.distance(&p) - curr_line.1.distance(&p);
					if best == -1 || best > curr {
						best = curr
					}
				}
				None => continue,
			}
		}
	}

	best
}
