use std::fs;

fn get_modes(s: Vec<char>) -> (u32, u32, u32) {
	let mut modes: (u32, u32, u32) = (0, 0, 0);
	if s.len() == 3 {
		modes.0 = 1;
	} else if s.len() == 4 {
		modes.0 = s[1].to_digit(10).unwrap();
		modes.1 = 1;
	} else if s.len() == 5 {
		modes.0 = s[2].to_digit(10).unwrap();
		modes.1 = s[1].to_digit(10).unwrap();
		modes.2 = 1;
	}
	modes
}

pub fn part1() -> i32 {
	let mut cmds: Vec<i32> = fs::read_to_string("./inputs/day5.txt")
		.unwrap()
		.trim()
		.split(',')
		.map(|s| s.parse().expect("failed to parse file"))
		.collect();

	let mut i = 0;
	loop {
		let opcode: Vec<char> = cmds[i].to_string().chars().collect();

		match opcode[opcode.len() - 1] {
			'1' => {
				let modes = get_modes(opcode);
				let mut args: (i32, i32) = (0, 0);

				args.0 = if modes.0 == 0 {
					cmds[cmds[i + 1] as usize]
				} else {
					cmds[i + 1]
				};

				args.1 = if modes.1 == 0 {
					cmds[cmds[i + 2] as usize]
				} else {
					cmds[i + 2]
				};

				let index = cmds[i + 3] as usize;
				cmds[index] = args.0 + args.1;
				i += 4;
			}
			'2' => {
				let modes = get_modes(opcode);
				let mut args: (i32, i32) = (0, 0);

				args.0 = if modes.0 == 0 {
					cmds[cmds[i + 1] as usize]
				} else {
					cmds[i + 1]
				};

				args.1 = if modes.1 == 0 {
					cmds[cmds[i + 2] as usize]
				} else {
					cmds[i + 2]
				};

				let index = cmds[i + 3] as usize;
				cmds[index] = args.0 * args.1;
				i += 4;
			}
			'3' => {
				//////////////////////////////
				// this should be reading input:
				// ```
				// let mut input = String::new();
				// std::io::stdin()
				// 	.read_line(&mut input)
				// 	.expect("failed to read from stdin");
				// let input = input.chars().nth(0).unwrap().to_digit(10).unwrap() as i32;
				// ```
				// however to avoid human interaction it is hardcoded to 1 (as specified in the challenge)
				//////////////////////////////
				let index = cmds[i + 1] as usize;
				cmds[index] = 1;
				i += 2;
			}
			'4' => {
				//////////////////////////////
				// this should be printing:
				// ```
				// let modes = get_modes(opcode);
				// if modes.0 == 0 {
				// 	let index = cmds[i + 1] as usize;
				// 	print!("{}", cmds[index]);
				// } else {
				// 	print!("{}", cmds[i + 1]);
				// }
				// ```
				// however to avoid human interaction it is hardcoded to return if not 0 (as specified in the challenge)
				//////////////////////////////
				let modes = get_modes(opcode);
				let res = if modes.0 == 0 {
					let index = cmds[i + 1] as usize;
					cmds[index]
				} else {
					cmds[i + 1]
				};
				if res != 0 {
					return res;
				}
				i += 2;
			}
			'9' => {
				break;
			}
			_ => panic!("unexpected opcode"),
		}
	}

	panic!("didnt find result")
}

pub fn part2() -> i32 {
	let mut cmds: Vec<i32> = fs::read_to_string("./inputs/day5.txt")
		.unwrap()
		.trim()
		.split(',')
		.map(|s| s.parse().expect("failed to parse file"))
		.collect();

	let mut i = 0;
	loop {
		let opcode: Vec<char> = cmds[i].to_string().chars().collect();

		match opcode[opcode.len() - 1] {
			'1' => {
				let modes = get_modes(opcode);
				let mut args: (i32, i32) = (0, 0);

				args.0 = if modes.0 == 0 {
					cmds[cmds[i + 1] as usize]
				} else {
					cmds[i + 1]
				};

				args.1 = if modes.1 == 0 {
					cmds[cmds[i + 2] as usize]
				} else {
					cmds[i + 2]
				};

				let index = cmds[i + 3] as usize;
				cmds[index] = args.0 + args.1;
				i += 4;
			}
			'2' => {
				let modes = get_modes(opcode);
				let mut args: (i32, i32) = (0, 0);

				args.0 = if modes.0 == 0 {
					cmds[cmds[i + 1] as usize]
				} else {
					cmds[i + 1]
				};

				args.1 = if modes.1 == 0 {
					cmds[cmds[i + 2] as usize]
				} else {
					cmds[i + 2]
				};

				let index = cmds[i + 3] as usize;
				cmds[index] = args.0 * args.1;
				i += 4;
			}
			'3' => {
				//////////////////////////////
				// this should be reading input:
				// ```
				// let mut input = String::new();
				// std::io::stdin()
				// 	.read_line(&mut input)
				// 	.expect("failed to read from stdin");
				// let input = input.chars().nth(0).unwrap().to_digit(10).unwrap() as i32;
				// ```
				// however to avoid human interaction it is hardcoded to 5 (as specified in the challenge)
				//////////////////////////////
				let index = cmds[i + 1] as usize;
				cmds[index] = 5;
				i += 2;
			}
			'4' => {
				//////////////////////////////
				// this should be printing:
				// ```
				// let modes = get_modes(opcode);
				// if modes.0 == 0 {
				// 	let index = cmds[i + 1] as usize;
				// 	print!("{}", cmds[index]);
				// } else {
				// 	print!("{}", cmds[i + 1]);
				// }
				// ```
				// however to avoid human interaction it is hardcoded to return (as specified in the challenge)
				//////////////////////////////
				let modes = get_modes(opcode);
				let res = if modes.0 == 0 {
					let index = cmds[i + 1] as usize;
					cmds[index]
				} else {
					cmds[i + 1]
				};
				return res;
				// i += 2;
			}
			'5' => {
				let modes = get_modes(opcode);

				let to_check = if modes.0 == 0 {
					cmds[cmds[i + 1] as usize]
				} else {
					cmds[i + 1]
				};
				if to_check != 0 {
					i = if modes.1 == 0 {
						cmds[cmds[i + 2] as usize]
					} else {
						cmds[i + 2]
					} as usize;
				} else {
					i += 3;
				}
			}
			'6' => {
				let modes = get_modes(opcode);

				let to_check = if modes.0 == 0 {
					cmds[cmds[i + 1] as usize]
				} else {
					cmds[i + 1]
				};
				if to_check == 0 {
					i = if modes.1 == 0 {
						cmds[cmds[i + 2] as usize]
					} else {
						cmds[i + 2]
					} as usize;
				} else {
					i += 3;
				}
			}
			'7' => {
				let modes = get_modes(opcode);
				let mut args: (i32, i32) = (0, 0);

				args.0 = if modes.0 == 0 {
					cmds[cmds[i + 1] as usize]
				} else {
					cmds[i + 1]
				};

				args.1 = if modes.1 == 0 {
					cmds[cmds[i + 2] as usize]
				} else {
					cmds[i + 2]
				};

				let index = cmds[i + 3] as usize;

				cmds[index] = if args.0 < args.1 { 1 } else { 0 };
				i += 4;
			}
			'8' => {
				let modes = get_modes(opcode);
				let mut args: (i32, i32) = (0, 0);

				args.0 = if modes.0 == 0 {
					cmds[cmds[i + 1] as usize]
				} else {
					cmds[i + 1]
				};

				args.1 = if modes.1 == 0 {
					cmds[cmds[i + 2] as usize]
				} else {
					cmds[i + 2]
				};

				let index = cmds[i + 3] as usize;

				cmds[index] = if args.0 == args.1 { 1 } else { 0 };
				i += 4;
			}
			'9' => {
				break;
			}
			_ => panic!("unexpected opcode"),
		}
	}

	panic!("didnt find result")
}
