use math::lcm;
use rayon::prelude::*;
use std::collections::HashMap;
mod math;

fn main() {
    let input = if cfg!(debug_assertions) {
        r#"
        RL

        AAA = (BBB, CCC)
        BBB = (DDD, EEE)
        CCC = (ZZZ, GGG)
        DDD = (DDD, DDD)
        EEE = (EEE, EEE)
        GGG = (GGG, GGG)
        ZZZ = (ZZZ, ZZZ)
        "#
        .trim()
    } else {
        include_str!(concat!(std::env!("CARGO_MANIFEST_DIR"), "/d8input")).trim()
    };

    let lines = input.lines().collect::<Vec<&str>>();
    let directions = lines[0].trim();

    let mut map: HashMap<&str, (&str, &str)> = HashMap::new();

    for line in lines[2..].iter() {
        let split = line.trim().split(" = ").collect::<Vec<&str>>();
        let lhs = split[0];

        let rhs = split[1]
            .trim_matches(&['(', ')'] as &[char])
            .split(", ")
            .collect::<Vec<&str>>();

        map.insert(lhs, (rhs[0], rhs[1]));
    }

    println!("Part 1: {}", p1(&map, directions, "AAA", |s| s == "ZZZ"));
    println!("Part 2: {}", p2(&map, directions));
}

fn p1<F>(map: &HashMap<&str, (&str, &str)>, directions: &str, start: &str, end_fn: F) -> u32
where
    F: FnOnce(&str) -> bool + Copy,
{
    let mut count = 0;
    let mut next = start;

    for d in directions.chars().cycle() {
        if end_fn(next) {
            break;
        }
        let rhs = map.get(next).unwrap();

        next = match d {
            'L' => rhs.0,
            'R' => rhs.1,
            _ => continue,
        };
        count += 1;
    }

    count
}

fn p2(map: &HashMap<&str, (&str, &str)>, directions: &str) -> usize {
    let starts: Vec<&str> = map
        .keys()
        .filter(|k| k.chars().last() == Some('A'))
        .map(|x| *x)
        .collect();

    let counts: Vec<usize> = starts
        .par_iter()
        .map(|start| p1(&map, &directions, start, |s| s.chars().last() == Some('Z')) as usize)
        .collect();

    counts[1..].iter().fold(counts[0], |a, b| lcm(a, *b)) as usize
}
