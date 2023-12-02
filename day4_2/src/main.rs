#![allow(unused)]
use std::{
	fs::File,
	io::{BufRead, BufReader, Read, Seek},
	str::Lines,
};

fn main() {
	// let mut reader = include_str!("../../inputs/day4dummy").lines();
	let mut reader = include_str!("../../inputs/day4input.txt").lines();

	let calls: Vec<u16> = load_calls(&mut reader);
	let mut boards: Vec<Vec<u16>> = load_boards(&mut reader);

	// I THINK I NEED TO USE ADAPTERS
	// Filter the boards based on check_board method
	// boards = boards.into_iter().filter(|board| !check_board(board)).collect()
	// check the last board and loop till you find the winning number
	//
	for call in calls {
		for board in &mut boards {
			mark_board(call, board);
		}

		if boards.len() == 1 {
			print_board(&boards[0]);
			let sum: u16 = (&boards[0]).into_iter().sum();
			println!("\ncall: {}\nsum:{}\nsum * call: {}", call, sum, sum * call);
			if check_board(&boards[0]) {
				break;
			}
		}

		boards = boards
			.into_iter()
			.filter(|board| !check_board(board))
			.collect();
	}

	// check_board(&boards[0]);
}

fn mark_all_boards(call: u16, boards: &mut Vec<Vec<u16>>) {
	for i in 0..boards.len() {
		let board = &mut boards[i];
		mark_board(call, board);
	}
}

// -------------------- Helper Functions -------------------- //
fn check_board(board: &Vec<u16>) -> bool {
	// let sum = 0;

	for i in 0..5 {
		// check rows
		let j = i * 5;
		let row_sum: u16 = board[j..(j + 5)].into_iter().sum();

		/*
			0  1  2  3  4  5
			22 13 17 11 0  8  2 23  4 24 21  9 14 16  7 6 10  3 18  5 1 12 20 15 19

			 7 42 22 92 60
			 8 88 99 13 12
			16 62 86 24 77
			20 57 19 67 46
			36 83 54 63 82
		*/
		let col_sum: u16 = board
			.into_iter()
			.enumerate()
			.filter(|&(pos, _)| ((pos >= i) && (pos - i) % 5 == 0))
			.map(|(_, v)| v)
			.sum();

		println!("row_sum: {row_sum}\tcol_sum: {col_sum}");

		if row_sum == 0 || col_sum == 0 {
			return true;
		};
	}

	// row_sum == 0 || column_sum == 0
	false
}

fn print_board(board: &Vec<u16>) {
	println!("\n");
	for (i, x) in board.into_iter().enumerate() {
		if i % 5 != 0 || i == 0 {
			print!("{x}\t");
		} else {
			println!();
			print!("{x}\t");
		}
	}
}

fn mark_board(x: u16, board: &mut Vec<u16>) {
	// for i in 0..board.len() {
	// 	dbg!(board[i]);
	// }
	// println!("marking {x}");
	for ele in board {
		if *ele == x {
			*ele = 0;
		}
	}
}

fn load_calls(reader: &mut Lines) -> Vec<u16> {
	let calls = reader
		.next()
		.unwrap()
		.trim()
		.split(",")
		.map(|x| x.parse::<u16>().unwrap())
		.collect();
	reader.next().unwrap();
	calls
}

fn load_boards(reader: &mut Lines) -> Vec<Vec<u16>> {
	let mut boards: Vec<Vec<u16>> = Vec::new();
	let mut board: Vec<u16> = Vec::new();
	let mut count = 1;

	while let Some(line) = reader.next() {
		// dbg!(line);
		// if new line
		if count % 6 == 0 {
			&boards.push(board.clone());
			board.clear();
		// continue;
		} else {
			line.trim().split_whitespace().for_each(|x| {
				board.push(x.parse().unwrap());
			});
		}

		count += 1;
	}

	// need to call one last time as the last line will return None causing the loop to end and
	// so `&boards.push(board.clone())` will not be reached
	boards.push(board);
	boards
}

/*
 * calls: Vec<u16> <- first line
 * while reader.next()
 *      Vec::new() <-


	0:  22 13 17 11 00
	5:  08 02 23 04 24
	10: 21 09 14 16 07
	15: 06 10 03 18 05
	20: 01 12 20 15 19

 [22 13 17 11  0 8  2 23  4 24 21  9 14 16  7 6 10  3 18  5 1 12 20 15 19]

	for rows: sum(0,1,2,3,4) x 5
	for columns: sum(0,5,10,15,20) x 5


*/
