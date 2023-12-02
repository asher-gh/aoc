use std::{
	fs::File,
	io::{BufRead, BufReader},
};

fn main() {
	let input = File::open("input").unwrap();
	let mut prev = u32::MAX;
	let mut count = 0;

	for line in BufReader::new(input).lines() {
		let n = line.unwrap().parse::<u32>().unwrap();

		if n > prev {
			count += 1;
		}

        prev = n;
	}

    println!("{count}");
}
