use std::collections::{HashSet, VecDeque};

fn main() {
    const STEPS: usize = 64;
    let input = if cfg!(debug_assertions) {
        r#"
        ...........
        .....###.#.
        .###.##..#.
        ..#.#...#..
        ....#.#....
        .##..S####.
        .##..#...#.
        .......##..
        .##.#.####.
        .##..##.##.
        ...........
    "#
    } else {
        include_str!("../input")
    }
    .trim()
    .lines()
    .map(|l| l.trim().chars().collect())
    .collect::<Vec<Vec<char>>>();

    let (sx, sy) = input
        .iter()
        .enumerate()
        .find_map(|(row_idx, row)| {
            row.iter().enumerate().find_map(|(col_idx, &val)| {
                match val {
                    'S' => Some((row_idx as i64, col_idx as i64)),
                    _ => None,
                }
            })
        })
        .unwrap();

    println!("Part I: {}", p1(&input, sx, sy, 64));
    println!("Part II: {}", p2(&input, sx, sy));
}

fn p1(input: &Vec<Vec<char>>, sx: i64, sy: i64, steps: i64) -> usize {
    // const STEPS: usize = 64;
    let (h, w) = (input.len() as i64, input[0].len() as i64);
    let mut q = VecDeque::<(i64, i64, i64)>::from([(sx, sy, steps)]);

    let mut seen = HashSet::<(i64, i64)>::from([(sx, sy)]);
    let mut ans = HashSet::<(i64, i64)>::new();

    while let Some((x, y, s)) = q.pop_front() {
        if s % 2 == 0 {
            ans.insert((x, y));
        }
        if s == 0 {
            continue;
        }

        for (a, b) in [(-1, 0), (1, 0), (0, 1), (0, -1)] {
            let (nx, ny) = ((x + a), (y + b));

            if nx < 0
                || nx >= h
                || ny < 0
                || ny >= w
                || input[nx as usize][ny as usize] == '#'
                || seen.contains(&(nx, ny))
            {
                continue;
            } else {
                seen.insert((nx, ny));
                q.push_back((nx, ny, s - 1));
            }
        }
    }

    ans.len()
}

fn p2(input: &Vec<Vec<char>>, sx: i64, sy: i64) -> usize {
    const STEPS: i64 = 26501365;
    let (h, w) = (input.len() as i64, input[0].len() as i64);
    assert_eq!(w, h);
    assert_eq!(sx, sy);
    assert_eq!(sx, w / 2);
    assert_eq!(STEPS % w, sx);

    let grid_width = STEPS / w - 1;
    let n_odd = (grid_width / 2 * 2 + 1).pow(2) as usize;
    let n_even = ((grid_width + 1) / 2 * 2).pow(2) as usize;

    let odd_points = p1(&input, sx, sy, w * 2 + 1);
    let even_points = p1(&input, sx, sy, w * 2);

    let t = p1(&input, w - 1, sy, w - 1);
    let b = p1(&input, 0, sy, w - 1);
    let r = p1(&input, sx, 0, w - 1);
    let l = p1(&input, sx, w - 1, w - 1);

    let s_tr = p1(&input, w - 1, 0, w / 2 - 1);
    let s_tl = p1(&input, w - 1, w - 1, w / 2 - 1);
    let s_br = p1(&input, 0, 0, w / 2 - 1);
    let s_bl = p1(&input, 0, w - 1, w / 2 - 1);

    let l_tr = p1(&input, w - 1, 0, w * 3 / 2 - 1);
    let l_tl = p1(&input, w - 1, w - 1, w * 3 / 2 - 1);
    let l_br = p1(&input, 0, 0, w * 3 / 2 - 1);
    let l_bl = p1(&input, 0, w - 1, w * 3 / 2 - 1);

    n_odd * odd_points
        + n_even * even_points
        + t
        + r
        + b
        + l
        + (grid_width as usize + 1) * (s_tr + s_tl + s_br + s_bl)
        + (grid_width as usize) * (l_tr + l_tl + l_br + l_bl)
}
