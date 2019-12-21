use std::collections::HashMap;

pub fn part1() -> i32 {
	let data = std::fs::read_to_string("./inputs/day6.txt").unwrap();
	let data: Vec<Vec<_>> = data
		.trim()
		.split('\n')
		.map(|s| s.split(')').collect())
		.collect();
	let mut connections = HashMap::new();
	let mut orbits = 0;

	for line in data {
		connections.insert(line[1].to_string(), line[0].to_string());
	}

	for (key, val) in &connections {
		let mut curr_key = key.clone();
		let mut curr_val = val.clone();
		if curr_val == "COM" {
			orbits += 1;
		}
		while curr_val != "COM" {
			curr_val = connections[&curr_key[..]].clone();
			curr_key = curr_val.clone();
			orbits += 1;
		}
	}
	orbits
}

pub fn part2() -> usize {
	let data = std::fs::read_to_string("./inputs/day6.txt").unwrap();
	let data: Vec<Vec<_>> = data
		.trim()
		.split('\n')
		.map(|s| s.split(')').collect())
		.collect();
	let mut connections = HashMap::new();
	for line in data {
		connections.insert(line[1].to_string(), line[0].to_string());
	}
	let mut curr_key = "YOU".to_string();
	let mut curr_val = connections[&curr_key[..]].clone();
	let mut you_path = vec![curr_val.clone()];
	while curr_val != "COM" {
		curr_key = curr_val.clone();
		curr_val = connections[&curr_key[..]].clone();
		if curr_val == "SAN" {
			return you_path.len() + 1;
		}
		you_path.push(curr_val.clone());
	}
	let mut curr_key = "SAN".to_string();
	let mut curr_val = connections[&curr_key[..]].clone();
	if curr_val == "COM" {
		return you_path.len() - 1;
	}
	let mut distance = 0;
	while curr_val != "COM" {
		curr_key = curr_val.clone();
		curr_val = connections[&curr_key[..]].clone();
		distance += 1;
		if let Some(i) = you_path.iter().position(|s| s == &curr_val[..]) {
			return i + distance;
		}
	}
	panic!("could find path from YOU to SAN")
}
