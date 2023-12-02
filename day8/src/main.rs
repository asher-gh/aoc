#![allow(unused)]

use std::{char::REPLACEMENT_CHARACTER, cmp::Ordering};

fn main() {
	// acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab
	//
	//  ddd
	// e   a
	// e   a
	//  fff
	// g   b
	// g   b
	//  ccc
	//

	/*
	 * I'll be using the contains method for comparing string slices
	 * The only digits that have same number of characters are with 5 and 6
	 * 0 -> 6 *
	 * 1 -> 2
	 * 2 -> 5 *
	 * 3 -> 5 *
	 * 4 -> 4
	 * 5 -> 5 *
	 * 6 -> 6 *
	 * 7 -> 3
	 * 8 -> 7
	 * 9 -> 6 *
	 *
	 * For 6
	 *   if (this does not contain 1) => 6
	 *   else if (this contains 4) => 9
	 *   else => 0
	 *
	 * For 5
	 *   if (this contains 1) => 3
	 *   else if (6 contains this) => 5
	 *   else => 2
	 *
	 */
	// parsing input as a Vec<Vec<&str>> such that
	// input[i[0]] => control signals
	// input[i[1]] => output value
	let input: Vec<Vec<&str>> = include_str!("../../inputs/day8")
		.trim()
		.lines()
		.map(|x| x.split('|').map(|s| s.trim()).collect::<Vec<&str>>())
		.collect();

	let mut outputs: Vec<u32> = vec![];

	for sig in &input {
		let inp = sig[0];
		let out = sig[1];
		let mut result: Vec<u8> = vec![];

		let key = decode(inp);

		for s in out.split_whitespace() {
			for i in 0..10 {
				// dbg!(s, key[i]);
				if s.len() == key[i].len() && contains(s, key[i]) {
					result.push(i as u8);
				}
			}
		}

		let result: u32 = result
			.iter()
			.map(|x| x.to_string())
			.collect::<String>()
			.parse()
			.unwrap();

		outputs.push(result);
	}

	println!("Sum of all outputs: {}", outputs.iter().sum::<u32>());
}

fn decode(s: &str) -> [&str; 10] {
	let mut key: [&str; 10] = [""; 10];
	println!("decoding: {s}");
	let mut sixes: Vec<&str> = Vec::new();
	let mut fives: Vec<&str> = Vec::new();

	s.split_whitespace().for_each(|w| match w.len() {
		2 => {
			key[1] = w;
		}
		3 => {
			key[7] = w;
		}
		4 => {
			key[4] = w;
		}
		5 => {
			fives.push(w);
		}
		6 => {
			sixes.push(w);
		}
		_ => {}
	});

	for s in sixes {
		if !(contains(s, key[1])) {
			key[6] = s;
		} else if contains(s, key[4]) {
			key[9] = s;
		} else {
			key[0] = s;
		}
	}

	for s in fives {
		if contains(s, key[1]) {
			key[3] = s;
		} else if contains(key[6], s) {
			key[5] = s;
		} else {
			key[2] = s;
		}
	}

	// for (i, s) in key.iter().enumerate() {
	// 	println!("{} -> {}", i, s);
	// }
	//

	key[8] = "abcdefg";

	key
}

fn contains(s1: &str, s2: &str) -> bool {
	let res = true;

	let chars: Vec<char> = s2.chars().collect();

	for c in chars {
		if !s1.contains(c) {
			return false;
		}
	}

	res
}
