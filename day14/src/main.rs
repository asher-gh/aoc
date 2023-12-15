use std::{
    collections::{hash_map::DefaultHasher, HashMap},
    hash::{Hash, Hasher},
};

fn main() {
    let input = if cfg!(debug_assertions) {
        r#"
        O....#....
        O.OO#....#
        .....##...
        OO.#O....O
        .O.....O#.
        O.#..O.#.#
        ..O..#O..O
        .......O..
        #....###..
        #OO..#....
    "#
    } else {
        include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/input.txt"))
    };

    let grid: Vec<String> = input.trim().lines().map(|s| s.trim().to_owned()).collect();

    println!("Part 1: {}", p1(&mut grid.clone()));
    println!("Part 2: {}", p2(&mut grid.clone()));
}

fn p1(grid: &mut Vec<String>) -> usize {
    tilt_north(grid);
    calc_load(grid)
}

fn p2(grid: &mut Vec<String>) -> usize {
    let offset: usize;
    let period: usize;
    // (index, hash)
    let mut hashes: HashMap<u64, usize> = HashMap::new();
    let mut count = 0;

    let mut grid_cpy = grid.clone();

    loop {
        let mut hasher = DefaultHasher::new();
        grid_cpy.hash(&mut hasher);
        let hash = hasher.finish();

        if hashes.contains_key(&hash) {
            offset = *hashes.get(&hash).unwrap();
            period = count - offset;
            break;
        } else {
            hashes.insert(hash, count);
            count += 1;
            cycle(&mut grid_cpy);
        }
    }

    let iterations = offset + ((1_000_000_000 - offset) % period);

    for _ in 0..iterations {
        cycle(grid);
    }

    calc_load(grid)
}
fn calc_load(grid: &mut Vec<String>) -> usize {
    grid.into_iter()
        .rev()
        .enumerate()
        .map(|(i, s)| s.chars().filter(|&c| c == 'O').count() * (i + 1))
        .sum()
}

fn cycle(grid: &mut Vec<String>) {
    tilt_north(grid);
    tilt_west(grid);
    tilt_south(grid);
    tilt_east(grid);
}

fn tilt_north(grid: &mut Vec<String>) {
    *grid = transpose(grid);
    tilt_west(grid);
    *grid = transpose(grid);
}
fn tilt_west(grid: &mut Vec<String>) {
    for line in grid {
        let mut chars = line.chars().collect::<Vec<char>>();
        let l = chars.len();

        for i in 0..l {
            if chars[i] == '.' {
                for j in i + 1..l {
                    match chars[j] {
                        'O' => {
                            chars.swap(i, j);
                            break;
                        }
                        '#' => {
                            break;
                        }
                        _ => {}
                    }
                }
            }
        }

        *line = chars.iter().collect();
    }
}
fn tilt_east(grid: &mut Vec<String>) {
    *grid = flip(grid);
    tilt_west(grid);
    *grid = flip(grid);
}
fn tilt_south(grid: &mut Vec<String>) {
    *grid = transpose(grid);
    *grid = flip(grid);
    tilt_west(grid);
    *grid = flip(grid);
    *grid = transpose(grid);
}

fn transpose(grid: &Vec<String>) -> Vec<String> {
    if grid.is_empty() {
        return vec![];
    }
    (0..grid[0].len()).fold(Vec::new(), |mut a, i| {
        let s = grid.iter().fold(String::new(), |mut b, line| {
            b.push(line.chars().nth(i).unwrap());
            b
        });
        a.push(s);
        a
    })
}

fn flip(grid: &Vec<String>) -> Vec<String> {
    if grid.is_empty() {
        return vec![];
    }

    grid.iter().map(|s| s.chars().rev().collect()).collect()
}

fn print_grid(grid: &Vec<String>) {
    for line in grid {
        println!("{line}");
    }
}
