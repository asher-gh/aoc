use std::collections::{HashSet, VecDeque};

use colored::*;

fn main() {
    let input = if cfg!(debug_assertions) {
        r#"
.|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|....
"#
    } else {
        include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/input.txt"))
    }
    .trim()
    .lines()
    .map(|line| line.trim().chars().collect())
    .collect();

    println!(
        "Part One: {}",
        format!("{}", p1(&input, (0, -1, 0, 1))).yellow().bold()
    );
    println!("Part Two: {}", format!("{}", p2(&input)).yellow().bold());
}

fn p1(grid: &Vec<Vec<char>>, init: (i8, i8, i8, i8)) -> usize {
    // encoding as r, c, dr, dc
    // starting at top left and going right
    // NOTE: could probably encode this in a byte

    let mut q: VecDeque<(i8, i8, i8, i8)> = VecDeque::from([init]);
    let mut seen: HashSet<(i8, i8, i8, i8)> = HashSet::new();

    while !q.is_empty() {
        let (mut r, mut c, dr, dc) = q.pop_front().unwrap();
        r += dr;
        c += dc;

        // bounds
        if r < 0 || r >= grid.len() as i8 || c < 0 || c >= grid[0].len() as i8 {
            continue;
        }
        let mut push = |x| {
            if seen.insert(x) {
                q.push_back(x);
            }
        };

        let ch = grid[r as usize][c as usize];

        match ch {
            '.' => push((r, c, dr, dc)),
            '-' if dc != 0 => push((r, c, dr, dc)),
            '|' if dr != 0 => push((r, c, dr, dc)),
            '/' => push((r, c, -dc, -dr)),
            '\\' => push((r, c, dc, dr)),
            _ => {
                let split = if ch == '|' {
                    [(1, 0), (-1, 0)]
                } else {
                    [(0, 1), (0, -1)]
                };

                for (dr, dc) in split {
                    push((r, c, dr, dc));
                }
            }
        }
    }

    seen.into_iter()
        .fold(HashSet::<(i8, i8)>::new(), |mut a, t| {
            a.insert((t.0, t.1));
            a
        })
        .len()
}

fn p2(grid: &Vec<Vec<char>>) -> usize {
    let mut inits: Vec<(i8, i8, i8, i8)> = vec![];
    let w = grid[0].len() as i8;
    let h = grid.len() as i8;

    // |> and <|
    for i in 0..h {
        inits.push((i as i8, -1, 0, 1));
        inits.push((i as i8, h, 0, -1));
    }

    //  󰔶 and 󱨉
    for i in 0..w {
        inits.push((-1, i as i8, 1, 0));
        inits.push((w, i as i8, -1, 0));
    }

    inits.into_iter().map(|init| p1(grid, init)).max().unwrap()
}
