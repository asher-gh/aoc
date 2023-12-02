use std::{
	fs::File,
	io::{BufRead, BufReader},
};

fn main() {
	let input = File::open("day2input").unwrap();

	let mut horizontal: u32 = 0;
	let mut depth: u32 = 0;

	for line in BufReader::new(input).lines() {
		let line = line.unwrap();
		let mut command: Vec<&str> = line.split(' ').collect();
		let value = command[1].parse::<u32>().unwrap();

		match command[0] {
			"forward" => {
				horizontal += value;
			}
			"up" => {
				depth -= value;
			}
			"down" => {
				depth += value;
			}
			_ => (),
		}

		println!("{:?}", command);
	}

	println!("output: {}", horizontal * depth);
}
