use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashSet},
    fs,
    path::Path,
};

fn main() {
    let input = if cfg!(debug_assertions) {
        "test.txt"
    } else {
        "input.txt"
    };

    let input: Vec<Vec<usize>> =
        fs::read_to_string(Path::new(env!("CARGO_MANIFEST_DIR")).join(input))
            .unwrap()
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| c.to_digit(10).unwrap() as usize)
                    .collect()
            })
            .collect();

    println!("Part One: {}", p1(&input));
    println!("Part Two: {}", p2(&input));
}

fn p1(grid: &Vec<Vec<usize>>) -> usize {
    // hl, r, c, dr, dc, n
    let mut pq: BinaryHeap<Reverse<(usize, usize, usize, i32, i32, usize)>> =
        BinaryHeap::from([Reverse((0, 0, 0, 0, 0, 0))]);

    let (h, w) = (grid.len(), grid[0].len());

    // (r, c, dr, dc, n)
    let mut seen: HashSet<(usize, usize, i32, i32, usize)> = HashSet::new();

    while let Some(Reverse((hl, r, c, dr, dc, n))) = pq.pop() {
        if r == h - 1 && c == w - 1 {
            return hl;
        }

        if seen.contains(&(r, c, dr, dc, n)) {
            continue;
        }

        seen.insert((r, c, dr, dc, n));

        // move in the direction
        if n < 3 && (dr, dc) != (0, 0) {
            let nr = r as i32 + dr;
            let nc = c as i32 + dc;

            if nr >= 0 && nr < h as i32 && nc >= 0 && nc < w as i32 {
                pq.push(Reverse((
                    hl + grid[nr as usize][nc as usize],
                    nr as usize,
                    nc as usize,
                    dr,
                    dc,
                    n + 1,
                )));
            }
        }

        // check turns
        for (ndr, ndc) in [(1, 0), (0, 1), (-1, 0), (0, -1)] {
            if (ndr, ndc) != (-dr, -dc) && (ndr, ndc) != (dr, dc) {
                let nr = r as i32 + ndr;
                let nc = c as i32 + ndc;

                if nr >= 0 && nr < h as i32 && nc >= 0 && nc < w as i32 {
                    pq.push(Reverse((
                        hl + grid[nr as usize][nc as usize],
                        nr as usize,
                        nc as usize,
                        ndr,
                        ndc,
                        1,
                    )));
                }
            }
        }
    }

    0
}

fn p2(grid: &Vec<Vec<usize>>) -> usize {
    // hl, r, c, dr, dc, n
    let mut pq: BinaryHeap<Reverse<(usize, usize, usize, i32, i32, usize)>> =
        BinaryHeap::from([Reverse((0, 0, 0, 0, 0, 0))]);

    let (h, w) = (grid.len(), grid[0].len());

    // (r, c, dr, dc, n)
    let mut seen: HashSet<(usize, usize, i32, i32, usize)> = HashSet::new();

    while let Some(Reverse((hl, r, c, dr, dc, n))) = pq.pop() {
        if r == h - 1 && c == w - 1 && n >= 4 {
            return hl;
        }

        if seen.contains(&(r, c, dr, dc, n)) {
            continue;
        }

        seen.insert((r, c, dr, dc, n));

        // move in the direction
        if n < 10 && (dr, dc) != (0, 0) {
            let nr = r as i32 + dr;
            let nc = c as i32 + dc;

            if nr >= 0 && nr < h as i32 && nc >= 0 && nc < w as i32 {
                pq.push(Reverse((
                    hl + grid[nr as usize][nc as usize],
                    nr as usize,
                    nc as usize,
                    dr,
                    dc,
                    n + 1,
                )));
            }
        }

        // check turns
        if n >= 4 || (dr, dc) == (0, 0) {
            for (ndr, ndc) in [(1, 0), (0, 1), (-1, 0), (0, -1)] {
                if (ndr, ndc) != (-dr, -dc) && (ndr, ndc) != (dr, dc) {
                    let nr = r as i32 + ndr;
                    let nc = c as i32 + ndc;

                    if nr >= 0 && nr < h as i32 && nc >= 0 && nc < w as i32 {
                        pq.push(Reverse((
                            hl + grid[nr as usize][nc as usize],
                            nr as usize,
                            nc as usize,
                            ndr,
                            ndc,
                            1,
                        )));
                    }
                }
            }
        }
    }

    0
}
