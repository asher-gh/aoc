#![allow(unused)]

use std::{
	fs::File,
	io::{BufRead, BufReader},
};

fn main() {
	let _input = File::open("input").unwrap();
	let mut count = 0u32;
	let mut increases = 0u32;

	let mut buffer = [0u32; 4];

	/* sample
	 199
	 200
	 208
	 210
	 200
	 207
	 240
	 269
	 260
	 263

	 200+208+210
	-199-200-208

	=> 210 - 199 positive => increase++

	 [0, 0, 0, 199]
	 [0, 0, 199, 200]
	 [0, 199, 200, 208]
	 [199, 200, 208, 210]
	 [200, 208, 210, 200]
	 [208, 210, 200, 207]
	 [210, 200, 207, 240]
	 [200, 207, 240, 269]
	 [207, 240, 269, 260]
	 [240, 269, 260, 263]
	 */

	let prev = 0u32;

	for line in BufReader::new(_input).lines() {
		let n: u32 = line.unwrap().parse().unwrap();

		if count > 2 && buffer[3] > buffer[0] {
			increases += 1;
		}

		buffer.rotate_left(1);
		buffer[3] = n;
		count += 1;

		println!("{:?}", buffer);
	}

	println!("count: {increases}");
}
