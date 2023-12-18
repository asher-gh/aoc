use anyhow;
use std::{collections::HashMap, fs, path::Path};

fn main() -> anyhow::Result<()> {
    let input = fs::read_to_string(Path::new(env!("CARGO_MANIFEST_DIR")).join(
        if cfg!(debug_assertions) {
            "test.txt"
        } else {
            "input.txt"
        },
    ))?;

    let plan: Vec<(char, i64)> = input
        .trim()
        .lines()
        .map(|line| {
            let x = line.trim().split_whitespace().collect::<Vec<_>>();
            (x[0].chars().last().unwrap(), x[1].parse().unwrap())
        })
        .collect();

    println!("Part One: {}", area(&plan));

    let plan: Vec<(char, i64)> = input
        .trim()
        .lines()
        .filter_map(|line| {
            let code = &line
                .trim()
                .split_whitespace()
                .last()
                .unwrap()
                .trim_matches(&['(', ')'] as &[char])[1..];

            let n = i64::from_str_radix(&code[..5], 16).unwrap();

            match code.chars().last().unwrap() {
                '0' => Some(('R', n)),
                '1' => Some(('D', n)),
                '2' => Some(('L', n)),
                '3' => Some(('U', n)),
                _ => None,
            }
        })
        .collect();

    println!("Part Two: {}", area(&plan));

    Ok(())
}

fn area(plan: &Vec<(char, i64)>) -> usize {
    let dirs: HashMap<char, (i64, i64)> =
        HashMap::from([('U', (-1, 0)), ('D', (1, 0)), ('L', (0, -1)), ('R', (0, 1))]);

    // Get boundary vertices
    let mut points: Vec<(i64, i64)> = Vec::from([(0, 0)]);

    let mut b = 0;

    for (d, n) in plan {
        b += n;
        let (dr, dc) = dirs[&d];
        let (r, c) = points.last().unwrap();
        points.push((r + dr * n, c + dc * n));
    }

    // Calculate area using the shoelace formula
    let a: i64 = (points
        .windows(2)
        .map(|p| {
            // (x_i * y_i+1) - (x_i+1 * y_i)
            (p[0].0 * p[1].1) as i64 - (p[1].0 * p[0].1) as i64
        })
        .sum::<i64>()
        / 2)
    .abs();

    // get internal points (area) using Pick's theorem
    let i = (a + 1) - (b / 2);

    (i + b) as usize
}
