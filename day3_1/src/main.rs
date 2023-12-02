#![allow(unused)]
use std::{
	fs::File,
	io::{BufRead, BufReader, Cursor, Read, Seek, SeekFrom},
	isize,
	os::unix::prelude::FileExt,
};

fn main() {
	let input_file = File::open("inputs/day3input").unwrap();
	let mut byte_vec: Vec<i32> = Vec::new();
	let mut gamma = 0_i32;
	let mut epsilon = 0_i32;
	let mut buf = String::new(); // will include \n
	let mut reader = BufReader::new(input_file);

	// read the first line and substract 1 for \n
	let word_length = reader.read_line(&mut String::new()).unwrap() - 1;

	// gotta reset the cursor in the internal buffer of BufReader
	reader.seek(SeekFrom::Start(0));

	// load all numbers in a vector
	for line in reader.lines() {
		let byte = isize::from_str_radix(&line.unwrap(), 2).unwrap();
		byte_vec.push(byte.try_into().unwrap());
	}

	let half_size = byte_vec.len() / 2;

	for i in (0..word_length).rev() {
		let mut count = 0;
		let mask = 2_i32.pow(i as u32);

		// 101110 & 100000
		for &x in &byte_vec {
			let y = x & mask;

			count += if (y == mask) { 1 } else { 0 };
		}

		if count >= half_size {
			gamma = gamma | mask;
		} else {
			epsilon |= mask;
		}
	}

	// epsilon = !gamma as u32;

	// println!("{epsilon:b}");
	//
	dbg!(epsilon * gamma);
}
