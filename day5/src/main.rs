use rayon::prelude::*;
use std::collections::HashMap;

fn main() {
    let input = if cfg!(debug_assertions) {
        r#"
        seeds: 79 14 55 13

        seed-to-soil map:
        50 98 2
        52 50 48

        soil-to-fertilizer map:
        0 15 37
        37 52 2
        39 0 15

        fertilizer-to-water map:
        49 53 8
        0 11 42
        42 0 7
        57 7 4

        water-to-light map:
        88 18 7
        18 25 70

        light-to-temperature map:
        45 77 23
        81 45 19
        68 64 13

        temperature-to-humidity map:
        0 69 1
        1 0 69

        humidity-to-location map:
        60 56 37
        56 93 4
    "#
    } else {
        include_str!(concat!(std::env!("CARGO_MANIFEST_DIR"), "/d5_input"))
    };

    let (seeds, map) = parse_input(&input);

    println!("Part 1: {}", p1(&seeds, &map));
    println!("Part 2: {}", p2(&seeds, &map));
}

/// What is the lowest location number that corresponds to any of the initial seed numbers?
fn p1(seeds: &Vec<u64>, map: &HashMap<&str, Vec<Range>>) -> u64 {
    seeds
        .par_iter()
        .map(|seed| {
            // seed -> soil
            let soil = get_mapping(*seed, map.get("seed-to-soil").unwrap());

            // soil -> fertilizer
            let fertilizer = get_mapping(soil, map.get("soil-to-fertilizer").unwrap());

            // fertilizer -> water
            let water = get_mapping(fertilizer, map.get("fertilizer-to-water").unwrap());

            // water -> light
            let light = get_mapping(water, map.get("water-to-light").unwrap());

            // light -> temp
            let temp = get_mapping(light, map.get("light-to-temperature").unwrap());

            // temp -> humidity
            let humidity = get_mapping(temp, map.get("temperature-to-humidity").unwrap());

            // humidity -> location
            let location = get_mapping(humidity, map.get("humidity-to-location").unwrap());

            location
        })
        .min()
        .unwrap()
}

fn p2(seeds: &Vec<u64>, map: &HashMap<&str, Vec<Range>>) -> u64 {
    // let mut new_seeds = Vec::new();
    let seeds: Vec<u64> = seeds
        .chunks(2)
        .flat_map(|w| Vec::from_iter(w[0]..w[0] + w[1]))
        .collect();

    p1(&seeds, map)
}

fn get_mapping(source: u64, map: &Vec<Range>) -> u64 {
    for Range { ss, se, ds, .. } in map {
        if (ss..se).contains(&&source) {
            let offset = source - ss;
            return ds + offset;
        }
    }

    // no match found
    source
}

#[derive(Debug, Copy, Clone)]
struct Range {
    ds: u64, // destination start
    ss: u64, // source start
    se: u64, // source end
}

impl Range {
    fn new(ds: u64, ss: u64, len: u64) -> Self {
        Range {
            ds,
            ss,
            se: ss + len,
        }
    }
}

fn parse_input(input: &str) -> (Vec<u64>, HashMap<&str, Vec<Range>>) {
    let mut map: HashMap<&str, Vec<Range>> = HashMap::new();
    let mut seeds = Vec::new();
    let mut current_section = "";
    for line in input.trim().lines() {
        if line.is_empty() {
            continue;
        }

        let words: Vec<&str> = line.trim().split_whitespace().collect();

        if words.last() == Some(&"map:") {
            current_section = words[0];
            continue;
        }
        let nums: Vec<u64> = words.iter().filter_map(|x| x.parse().ok()).collect();

        if words.first() == Some(&"seeds:") {
            seeds = nums;
            continue;
        }

        let range = Range::new(nums[0], nums[1], nums[2]);
        map.entry(current_section)
            .and_modify(|r| r.push(range))
            .or_insert(vec![range]);
    }

    (seeds, map)
}
