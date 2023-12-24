use std::{
    collections::{HashMap, HashSet},
    thread::sleep,
    time::Duration,
};
use utils::time;

fn main() {
    let input = if cfg!(debug_assertions) {
        include_str!("../test")
    } else {
        include_str!("../input")
    }
    .trim()
    .lines()
    .map(|line| line.chars().collect::<Vec<char>>())
    .collect::<Vec<Vec<char>>>();

    for (i, p) in [p1, p2].iter().enumerate() {
        let (t, ans) = time(|| p(&input));
        println!("Part {}: {ans} ({t:?})", i + 1);
    }
}

fn p1(input: &Vec<Vec<char>>) -> usize {
    let (sx, sy) = (0, 1);
    let mut seen: HashSet<(i64, i64)> = HashSet::new();
    dfs(&mut seen, &input, sx, sy, 0, |c: char| match c {
        '.' | 'O' => vec![(-1, 0), (1, 0), (0, 1), (0, -1)],
        '>' => vec![(0, 1)],
        '<' => vec![(0, -1)],
        '^' => vec![(-1, 0)],
        'v' => vec![(1, 0)],
        _ => vec![],
    })
}

fn p2(grid: &Vec<Vec<char>>) -> usize {
    let (w, h) = (grid[0].len(), grid.len());
    let start = (0, grid[0].iter().position(|&c| c == '.').unwrap());
    let end: (usize, usize) = (
        grid.len() - 1,
        grid.last().unwrap().iter().position(|&c| c == '.').unwrap(),
    );
    let mut points = vec![start, end];

    grid.iter().enumerate().for_each(|(r, row)| {
        row.iter()
            .enumerate()
            .filter(|(_, &ch)| ch != '#')
            .for_each(|(c, &ch)| {
                let (r, c) = (r as isize, c as isize);
                if [(r - 1, c), (r + 1, c), (r, c - 1), (r, c + 1)]
                    .into_iter()
                    .filter(|(nr, nc)| {
                        (0..h).contains(&(*nr as usize))
                            && (0..w).contains(&(*nc as usize))
                            && grid[*nr as usize][*nc as usize] != '#'
                    })
                    .count()
                    >= 3
                {
                    points.push((r as usize, c as usize));
                }
            })
    });

    let mut graph: HashMap<
        (usize, usize),
        HashMap<(usize, usize), usize>,
    > = points
        .iter()
        .map(|&p| (p, HashMap::<(usize, usize), usize>::new()))
        .collect();

    for (sr, sc) in &points {
        let mut stack = vec![(0, *sr, *sc)];
        let mut seen = HashSet::from([(*sr, *sc)]);

        while let Some((n, r, c)) = stack.pop() {
            if n != 0 && points.contains(&(r, c)) {
                graph.entry((*sr, *sc)).and_modify(|cp| {
                    cp.insert((r, c), n);
                });
                continue;
            }

            for (dr, dc) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                let (r, c) = (r as isize, c as isize);
                let (nr, nc) = (r + dr, c + dc);
                if (0..h).contains(&(nr as usize))
                    && (0..w).contains(&(nc as usize))
                    && grid[nr as usize][nc as usize] != '#'
                    && !seen.contains(&(nr as usize, nc as usize))
                {
                    stack.push((n + 1, nr as usize, nc as usize));
                    seen.insert((nr as usize, nc as usize));
                }
            }
        }
    }
    fn dfs2(
        pt: &(usize, usize),
        end: &(usize, usize),
        seen: &mut HashSet<(usize, usize)>,
        graph: &HashMap<
            (usize, usize),
            HashMap<(usize, usize), usize>,
        >,
    ) -> isize {
        if pt == end {
            return 0;
        }

        let mut m = isize::MIN;

        seen.insert(*pt);
        for nx in graph.get(&pt).unwrap().keys() {
            if !seen.contains(nx) {
                let n = dfs2(nx, end, seen, graph)
                    + *graph.get(&pt).unwrap().get(nx).unwrap()
                        as isize;
                m = m.max(n);
            }
        }
        seen.remove(&pt);

        m
    }

    let mut seen: HashSet<(usize, usize)> = HashSet::new();
    dfs2(&start, &end, &mut seen, &graph) as usize
}

fn dfs(
    seen: &mut HashSet<(i64, i64)>,
    input: &Vec<Vec<char>>,
    x: i64,
    y: i64,
    depth: usize,
    directions: impl FnOnce(char) -> Vec<(i64, i64)> + Copy,
) -> usize {
    let (w, h) = (input[0].len() as i64, input.len() as i64);

    if !(0..w).contains(&x)
        || !(0..h).contains(&y)
        || seen.contains(&(x, y))
        || input[x as usize][y as usize] == '#'
        || (x == h - 1 && y == w - 2)
    {
        return depth;
    }

    let c = input[x as usize][y as usize];

    if let Some(sub_max) = directions(c)
        .iter()
        .map(|(dx, dy)| {
            seen.insert((x, y));
            let d = dfs(
                seen,
                input,
                x + dx,
                y + dy,
                depth + 1,
                directions,
            );
            seen.remove(&(x, y));
            d
        })
        .max()
    {
        return sub_max.max(depth);
    };

    depth
}

fn print_grid(grid: &Vec<Vec<char>>, seen: &HashSet<(i64, i64)>) {
    print!("{}[2J", 27 as char);
    sleep(Duration::from_millis(30));

    for (i, row) in grid.clone().iter().enumerate() {
        println!(
            "{}",
            row.iter()
                .enumerate()
                .map(|(j, c)| {
                    if seen.contains(&(i as i64, j as i64)) {
                        '＊'
                    } else {
                        match *c {
                            '#' => '⿴',
                            '.' => '〇',
                            '>' => '＞',
                            '<' => '＜',
                            '^' => 'Ｖ',
                            'v' => '＾',
                            _ => *c,
                        }
                    }
                })
                .collect::<String>()
        );
    }
}
