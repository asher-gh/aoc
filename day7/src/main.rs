fn main() {
	// after trying mean, median and mode, I'll just brute force all values against themselves
	// and take the smallest value from that.
	let mut input: Vec<u64> = include_str!("../../inputs/day7")
		.trim()
		.split(',')
		.map(|x| x.parse().unwrap())
		.collect();
	input.sort();

	let mut fuel: Vec<(u64, u64)> = Vec::new();

	for i in 0..*input.last().unwrap() {
		// println!("--------------------------------");
		let current = i;

		let sum: u64 = *&input
			.iter()
			.map(|x| {
				let diff = x.abs_diff(current);
				let res = (diff * (diff + 1)) / 2;
				// dbg!(current, res);
				res
			})
			.sum();
		fuel.push((sum, current));
	}

	fuel.sort();

	dbg!(fuel[0]);
}
