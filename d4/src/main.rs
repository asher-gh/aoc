use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
	static ref RE: Regex = Regex::new(r"(\d{0,})-(\d{0,}),(\d{0,})-(\d{0,})").unwrap();
}

fn main() {
	let input = include_str!("../../inputs/d4");
	let (mut day1_count, mut day2_count) = (0u32, 0u32);

	for line in input.lines() {
		for cap in RE.captures_iter(line) {
			let nums: Vec<u32> = [&cap[1], &cap[2], &cap[3], &cap[4]]
				.iter()
				.map(|x| x.parse().expect("could not parse {x}"))
				.collect();

			if pt1(&nums) {
				day1_count += 1;
			}

			if pt2(&nums) {
				day2_count += 1;
			}
		}
	}

	println!("Part 1: {day1_count}");
	println!("Part 2: {day2_count}");
}

fn pt2(nums: &[u32]) -> bool {
	(nums[0] <= nums[3] && nums[1] >= nums[2]) || (nums[3] <= nums[0] && nums[2] >= nums[1])
}

fn pt1(nums: &[u32]) -> bool {
	(nums[0] <= nums[2] && nums[1] >= nums[3]) || (nums[0] >= nums[2] && nums[1] <= nums[3])
}
