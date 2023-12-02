#![allow(unused)]
use std::{
	fmt, fs,
	ops::{Deref, DerefMut},
};

#[derive(Debug, Clone)]
struct Stack {
	values: Vec<char>,
}

impl Deref for Stack {
	type Target = Vec<char>;

	fn deref(&self) -> &Self::Target {
		&self.values
	}
}

impl DerefMut for Stack {
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.values
	}
}

impl fmt::Display for Stack {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(
			f,
			"{}",
			format!(
				"{}",
				self.values
					.iter()
					.fold(String::new(), |acc, &arg| acc + " " + &arg.to_string())
			)
		)
	}
}

impl Stack {
	fn new(x: &[u8]) -> Stack {
		let space = ' ' as u8;
		Stack {
			values: x
				.into_iter()
				.filter_map(|x| match x {
					_y if _y == &space => None,
					_ => Some(*x as char),
				})
				.collect::<Vec<char>>(),
		}
	}
}

fn main() {
	// ================================ Reading Input =================================
	let input = fs::read_to_string("../inputs/d5").expect("\x1b[31mInput not found\x1b[0m");

	// 	let input = "
	//     [D]
	// [N] [C]
	// [Z] [M] [P]
	//  1   2   3
	//
	// move 1 from 2 to 1
	// move 3 from 1 to 3
	// move 2 from 2 to 1
	// move 1 from 1 to 2
	// ";

	// =========================== Finding Number of Stacks ===========================
	let re_stack = regex::Regex::new(r"(?m)^\s[\s\d\s]+\s$").unwrap();
	let mut start: usize = 0;
	let mut end: usize = 3;

	if let Some(matched) = re_stack.find(&input) {
		start = matched.start();
		end = matched.end();
	}

	let stacks_n = &input[end - 3..end]
		.trim()
		.parse::<u8>()
		.expect("Failed to parse number of stacks.");

	// ========================== Reading the initial stacks ==========================
	let mut stack_list: Vec<Stack> = vec![Stack::new(&[]); *stacks_n as usize];
	let mut stack_size: usize = 0;

	for line in input[0..start].lines().rev() {
		for (i, c) in line.chars().enumerate() {
			let j = i as u8 % (stacks_n * 3 + stacks_n - 1) / 4;

			match c {
				c if i % 2 != 0 && c != ' ' => {
					let mut stack = &mut stack_list[j as usize];
					stack.push(c);
					if stack.len() > stack_size {
						stack_size = stack.len();
					}
				}
				_ => (),
			}
		}
	}

	println!("{:=^1$}", " Initial Stack ", stack_list.len() * 4);
	print_stacks(&stack_list, stack_size);

	// ======================== Parsing the move instructions =========================
	let re_move =
		regex::Regex::new(r"(?m)^move (\d{1,}) from (\d) to (\d)$").expect("Move regex failed");

	let mut move_counts = 0;

	for cap in re_move.captures_iter(&input) {
		move_counts += 1;
		// println!("\x1b[40m{} [{}]-> {}\x1b[0m", &cap[2], &cap[1], &cap[3]);

		let from = &mut stack_list[(*&cap[2].parse::<usize>().unwrap()) - 1];
		let split_at = from
			.len()
			.saturating_sub(*&cap[1].parse::<usize>().unwrap());
		let mut tail = from.split_off(split_at);
		// tail.reverse();

		let to = &mut stack_list[(*&cap[3].parse::<usize>().unwrap()) - 1];

		to.append(&mut tail);

		stack_size = stack_list.iter().map(|x| x.len()).max().unwrap();

		// print_stacks(&stack_list, stack_size);
	}

	println!("{:=^1$}", " Final Stack ", stack_list.len() * 4);
	print_stacks(&stack_list, stack_size);
	println!("Total moves: {move_counts}");

	println!("Message: \"{}\"", stack_top(&stack_list));
}

fn stack_top(stack_list: &[Stack]) -> String {
	let mut result = String::new();

	for stack in stack_list {
		result.push(match stack.last() {
			Some(&c) => c,
			None => ' ',
		});
	}

	result
}

fn print_stacks(stack_list: &[Stack], max_height: usize) {
	for height in (1..max_height + 1).rev() {
		for i in (0..stack_list.len()) {
			let stack = &stack_list[i].values;
			print!(
				"{}",
				if stack.len() < height {
					format!("{: <4}", "")
				} else {
					format!("[{}] ", stack[height - 1])
				}
			);
		}
		println!();
	}
	println!("{:-<1$}", "", stack_list.len() * 4);

	for i in 1..stack_list.len() + 1 {
		print!(" {i}  ");
	}
	println!("\n{:-<1$}", "", stack_list.len() * 4);
}
