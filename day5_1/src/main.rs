use regex::Regex;

fn main() {
	let input = include_str!("../../inputs/day5input");
	let mut slate = [0_u32; 1_000_000];
	// let mut lines = [0usize; 100];

	// 0,9 -> 5,9

	let re = Regex::new(r"(\d{1,}),(\d{1,})\s->\s(\d{1,}),(\d{1,})").unwrap();
	/* r is a raw string literal
	 * raw string literals do not process any escapes */

	// y increases by 10
	// x increases by 1 and resets to 0 every 10th
	// (0,0)(1,0)(2,0)(3,0)...(9,0)
	// (0,1)(1,1)(2,1)(3,1)...(9,1)
	//
	// I'll just do it in 1D like (0,9) position in the array will be 0+9*10=90

	// note that the entire capture group is stored at index 0

	for cap in re.captures_iter(input) {
		let (x1, y1, x2, y2): (usize, usize, usize, usize) = (
			cap[1].parse().unwrap(),
			cap[2].parse().unwrap(),
			cap[3].parse().unwrap(),
			cap[4].parse().unwrap(),
		);

		println!("\n({x1}, {y1}) -> ({x2}, {y2})");

		// horizontal
		if y1 == y2 {
			let i = (y1 * 1000) + x1;
			let j = (y2 * 1000) + x2;

			let range = if i > j { j..=i } else { i..=j };

			for a in range {
				slate[a] += 1_u32;
			}
		}

		// vertical
		if x1 == x2 {
			let i = y1 * 1000;
			let j = y2 * 1000;

			// dbg!(i, j);

			let range = if i > j { j..=i } else { i..=j };

			// range = range.step_by(10);

			for a in range.step_by(1000) {
				// dbg!(a);
				slate[a + x1] += 1;
			}
		}

		// for (i, n) in slate.iter().enumerate() {
		// 	if i > 0 && i % 10 == 0 {
		// 		println!("");
		// 	}
		// 	print!("{n}");
		// }
	}

	let result: Vec<&u32> = slate.iter().filter(move |x| x > &&1u32).collect();

	println!("\n\n{}", result.len());
}
