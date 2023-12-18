fn main() {
    let input = if cfg!(debug_assertions) {
        r#"
        Time:      7  15   30
        Distance:  9  40  200
        "#
    } else {
        include_str!(concat!(std::env!("CARGO_MANIFEST_DIR"), "/d6_input"))
    };

    p1(input);
    p2(input);
}

fn p1(input: &str) {
    let races = parse_races_p1(input);

    let result: usize = races
        .into_iter()
        .map(|(time, dist)| {
            (1..time)
                .into_iter()
                .filter_map(|x| if x * (time - x) > dist { Some(x) } else { None })
                .count()
        })
        .product();

    println!("Part 1: {}", result);
}

fn p2(input: &str) {
    let race: Vec<usize> = input
        .trim()
        .lines()
        .filter_map(|line| line.trim().split(":").last())
        .map(|s| s.replace(" ", ""))
        .filter_map(|s| s.parse().ok())
        .collect();

    let (time, dist) = (race[0], race[1]);

    let result = (1..time)
        .into_iter()
        .filter_map(|x| if x * (time - x) > dist { Some(x) } else { None })
        .count();

    println!("Part 2: {}", result);
}

fn parse_races_p1(input: &str) -> Vec<(usize, usize)> {
    let races: Vec<Vec<usize>> = input
        .trim()
        .lines()
        .filter_map(|line| line.trim().split(":").last())
        .map(|nums| {
            nums.split_whitespace()
                .filter_map(|x| x.parse().ok())
                .collect()
        })
        .collect();

    races
        .first()
        .unwrap()
        .clone()
        .into_iter()
        .zip(races.last().unwrap().clone())
        .collect()
}
