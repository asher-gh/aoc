use anyhow::Result;
use once_cell::sync::Lazy;
use std::{collections::HashMap, fs, path::Path};

static mut CACHE: Lazy<HashMap<(String, Vec<usize>), usize>> = Lazy::new(|| HashMap::new());

fn main() -> Result<()> {
    let input_file = if cfg!(debug_assertions) {
        "test.txt"
    } else {
        "input.txt"
    };

    let input = fs::read_to_string(Path::new(env!("CARGO_MANIFEST_DIR")).join(input_file))?;

    println!("Part 1: {}", p1(&input));
    println!("Part 2: {}", p2(&input));

    Ok(())
}

fn p1(input: &str) -> usize {
    let mut total = 0;
    for line in input.lines() {
        let split: Vec<&str> = line.trim().split_whitespace().take(2).collect();
        let cfg = split[0];
        let nums: Vec<usize> = split[1].split(",").map(|n| n.parse().unwrap()).collect();

        total += count(&cfg, &nums);
    }

    total
}

fn p2(input: &str) -> usize {
    let mut total = 0;
    for line in input.lines() {
        let split: Vec<&str> = line.trim().split_whitespace().take(2).collect();
        let cfg = split[0];
        let nums: Vec<usize> = split[1].split(",").map(|n| n.parse().unwrap()).collect();

        let cfg = (0..5)
            .fold(Vec::new(), |mut a, _| {
                a.push(cfg);
                a
            })
            .join("?");

        let nums = nums.repeat(5);

        total += count(&cfg, &nums);
    }

    total
}
fn count(cfg: &str, nums: &[usize]) -> usize {
    if cfg.is_empty() {
        return match nums.len() {
            0 => 1,
            _ => 0,
        };
    }

    if nums.is_empty() {
        return if cfg.contains("#") { 0 } else { 1 };
    }

    if let Some(x) = unsafe { CACHE.get(&(cfg.to_owned(), nums.to_owned())) } {
        return *x;
    }

    let mut result = 0;

    let (ch, n) = (cfg.chars().next().unwrap(), nums[0]);

    if ".?".contains(ch) {
        result += count(&cfg[1..], nums)
    }

    if "#?".contains(ch)
        && n <= cfg.len()
        && !cfg[..n].contains('.')
        && (n == cfg.len() || cfg.chars().nth(n).unwrap() != '#')
    {
        result += count(if n == cfg.len() { "" } else { &cfg[n + 1..] }, &nums[1..])
    }

    unsafe { CACHE.insert((cfg.to_owned(), nums.to_owned()), result) };

    result
}
