const DAYS: usize = 256;
fn main() {
	let input: Vec<usize> = include_str!("../../inputs/day6")
		.trim()
		.split(',')
		.map(|x| x.parse::<usize>().unwrap())
		.collect();

	// fishes[i] will correspond to number of fishes with that many life

	let mut fishes: [u64; 9] = [0; 9];

	input.into_iter().for_each(|x| {
		fishes[x] += 1;
	});

	for _ in 0..DAYS {
		let new_fishes = fishes[0];
		for i in 0..6 {
			fishes[i] = fishes[i + 1];
		}
		fishes[6] = fishes[7] + new_fishes;
		fishes[7] = fishes[8];
		fishes[8] = new_fishes;
	}

	println!(
		"\nTotal fishes after {} days: {}",
		DAYS,
		fishes.into_iter().sum::<u64>()
	);
}
