#![allow(unused)]

use std::{char, ops::RangeInclusive};

use regex::Regex;
const MAX: isize = 1000;
const BOARD_SIZE: isize = MAX * MAX;
static mut BOARD: [u32; BOARD_SIZE as usize] = [0_u32; BOARD_SIZE as usize];

fn main() {
	let input = include_str!("../../inputs/day5input");
	let re = Regex::new(r"(\d{1,}),(\d{1,})\s->\s(\d{1,}),(\d{1,})").unwrap();

	for cap in re.captures_iter(input) {
		let a = Point::new(&cap[1], &cap[2]);
		let b = Point::new(&cap[3], &cap[4]);

		mark_horizontal(a, b);
		mark_vertical(a, b);
		mark_diagnol(a, b);
	}
	print_board();
}

fn mark_horizontal(a: Point, b: Point) {
	if a.1 != b.1 {
		return;
	}

	unsafe {
		BOARD[Point::range(a, b)].iter_mut().for_each(|x| *x += 1);
	}
}

fn mark_vertical(a: Point, b: Point) {
	if a.0 != b.0 {
		return;
	}

	unsafe {
		BOARD[Point::range(a, b)]
			.iter_mut()
			.step_by(MAX as usize)
			.for_each(|x| *x += 1);
	}
}

fn mark_diagnol(mut a: Point, b: Point) {
	// there are 4 ways we can go diagnolly
	// right-down (+1,+1)
	// right-up (+1,-1)
	// left-up(-1,-1)
	// left-down(-1,+1)
	//
	// if |x1-x2|==|y1-y2| => there exists a diagnol path between P1 and P2

	let x_diff = a.0.abs_diff(b.0);
	let y_diff = a.1.abs_diff(b.1);

	if x_diff != y_diff {
		return;
	}

	let x_mov = match b.0 - a.0 {
		x if x < 0 => -1,
		_ => 1,
	};

	let y_mov = match b.1 - a.1 {
		y if y < 0 => -1,
		_ => 1,
	};

	let range = 0..=x_diff;

	for _ in range {
		unsafe {
			BOARD[a.flatten()] += 1;
		}

		a.0 += x_mov;
		a.1 += y_mov;
	}
}

fn print_board() {
	// for i in 0..BOARD_SIZE as usize {
	// 	if i != 0 && i % 10 == 0 {
	// 		println!();
	// 	}
	// 	unsafe {
	// 		print!(
	// 			"{}",
	// 			match BOARD[i] {
	// 				0 => '.',
	// 				x => char::from_digit(x, 10).unwrap(),
	// 			}
	// 		);
	// 	}
	// }
	let mut result: Vec<u32>;

	unsafe {
		result = BOARD.iter().filter(|x| **x > 1).map(|x| *x).collect();
	};

	println!("\n\n\x1b[33mResult: {}", result.len());
}

#[derive(Clone, Copy)]
struct Point(isize, isize);

impl Point {
	fn new(x: &str, y: &str) -> Self {
		Point(x.parse().unwrap(), y.parse().unwrap())
	}
	fn range(a: Point, b: Point) -> RangeInclusive<usize> {
		let (x, y) = (a.flatten(), b.flatten());
		if x < y {
			x..=y
		} else {
			y..=x
		}
	}
	fn flatten(&self) -> usize {
		(MAX * self.1 + self.0) as usize
	}
}
