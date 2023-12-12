use std::{env, fs, path::Path};

fn main() -> anyhow::Result<()> {
    let input_file = if cfg!(debug_assertions) {
        "test.txt"
    } else {
        "input.txt"
    };

    let input = fs::read_to_string(Path::new(env!("CARGO_MANIFEST_DIR")).join(input_file))?;

    let grid: Vec<Vec<char>> = input
        .trim()
        .lines()
        .map(|line| line.trim().chars().collect())
        .collect();

    let empty_rows: Vec<usize> = empty_rows(&grid);
    let empty_cols: Vec<usize> = empty_cols(&grid);

    let points: Vec<(usize, usize)> = grid
        .iter()
        .enumerate()
        .map(|(r, row)| {
            row.iter()
                .enumerate()
                .filter(|(c, &ch)| ch == '#')
                .map(move |(c, _)| (r, c))
        })
        .flatten()
        .collect();

    println!(
        "Part 1: {}",
        solution(&grid, &points, 2, &empty_rows, &empty_cols)
    );
    println!(
        "Part 2: {}",
        solution(&grid, &points, 1_000_000, &empty_rows, &empty_cols)
    );

    Ok(())
}

fn solution(
    grid: &Vec<Vec<char>>,
    points: &Vec<(usize, usize)>,
    scale: u64,
    empty_rows: &[usize],
    empty_cols: &[usize],
) -> u64 {
    let mut total = 0;
    for (i, (r1, c1)) in points.iter().enumerate() {
        for (r2, c2) in &points[0..i] {
            total += (*r1.min(r2)..*r1.max(r2))
                .map(|i| if empty_rows.contains(&i) { scale } else { 1 })
                .sum::<u64>();
            total += (*c1.min(c2)..*c1.max(c2))
                .map(|i| if empty_cols.contains(&i) { scale } else { 1 })
                .sum::<u64>();
        }
    }

    total
}

fn empty_rows(grid: &Vec<Vec<char>>) -> Vec<usize> {
    grid.iter()
        .enumerate()
        .filter_map(|(r, line)| {
            if line.iter().all(|&ch| ch == '.') {
                Some(r)
            } else {
                None
            }
        })
        .collect()
}

fn empty_cols(grid: &Vec<Vec<char>>) -> Vec<usize> {
    (0..grid[0].len())
        .filter_map(|c| {
            let mut col = grid.iter().enumerate().map(|(r, row)| row[c]);

            if col.all(|ch| ch == '.') {
                Some(c)
            } else {
                None
            }
        })
        .collect()
}
