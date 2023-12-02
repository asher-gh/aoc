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

	let mut o2_vec = byte_vec.to_owned();
	let mut co2_vec = byte_vec.to_owned();

	for i in (0..word_length).rev() {
		let o2_len = o2_vec.len();
		let co2_len = co2_vec.len();
		let o2_hsize = (o2_len / 2);
		let co2_hsize = (co2_len / 2);
		let mut o2_set = 0;
		let mut co2_set = 0;
		let mask = 2_i32.pow(i as u32);

		// 101110 & 100000
		for &x in &o2_vec {
			let y = x & mask;
			o2_set += if (y == mask) { 1 } else { 0 };
		}

		for &x in &co2_vec {
			let y = x & mask;
			co2_set += if (y == mask) { 1 } else { 0 };
		}

		if (o2_len > 1) {
			if (o2_len % 2 == 0 && o2_set == o2_hsize) || (o2_set > o2_hsize) {
				// filter out the numbers with set bit at current place
				o2_vec = o2_vec.into_iter().filter(|x| x & mask == mask).collect();
			} else {
				// filter out the number with unset bit
				o2_vec = o2_vec.into_iter().filter(|x| x & mask != mask).collect();
			}
		}

		if (co2_len > 1) {
			if (co2_set > co2_hsize) || (co2_set == co2_hsize && co2_len % 2 == 0) {
				co2_vec = co2_vec.into_iter().filter(|x| x & mask != mask).collect();
			} else {
				co2_vec = co2_vec.into_iter().filter(|x| x & mask == mask).collect();
			}
		}
	}

	println!("Solution: {}", &o2_vec[0] * &co2_vec[0])
}
