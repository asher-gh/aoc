use lazy_static::lazy_static;
use std::{env, path::PathBuf};

lazy_static! {
    static ref NUMBER_MAP: [(&'static str, &'static str); 9] = [
        ("one", "1"),
        ("two", "2"),
        ("three", "3"),
        ("four", "4"),
        ("five", "5"),
        ("six", "6"),
        ("seven", "7"),
        ("eight", "8"),
        ("nine", "9"),
    ];
}

fn main() {
    let mut input =
        PathBuf::from(env::var("CARGO_MANIFEST_DIR").expect("cargo manifest not found"));

    input.push("input.txt");
    let input = std::fs::read_to_string(input).unwrap();

    println!("p1: {}", p1(input.trim()));
    println!("p2: {}", p2(input.trim()));
}

fn p1(input: &str) -> usize {
    input
        .lines()
        .fold(0, |acc, line| acc + caliberation_vals(line))
}

fn p2(input: &str) -> usize {
    let input = input;

    let input = NUMBER_MAP.iter().fold(input.to_string(), |acc, cur| {
        acc.replace(cur.0, &format!("{}{}{}", cur.0, cur.1, cur.0))
    });

    p1(&input)
}

fn caliberation_vals(i: &str) -> usize {
    let s: Vec<char> = i.chars().filter(|c| c.is_numeric()).collect();

    let x: String = [s.first().unwrap().clone(), s.last().unwrap().clone()]
        .iter()
        .collect();

    x.parse().unwrap()
}
