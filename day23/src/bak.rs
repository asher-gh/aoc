use std::collections::{HashMap, HashSet};
use std::fs;

fn main() {
    let contents =
        fs::read_to_string("input").expect("Failed to read the file");
    let grid: Vec<Vec<char>> = contents
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let start = (0, grid[0].iter().position(|&c| c == '.').unwrap());
    let end = (
        grid.len() - 1,
        grid.last().unwrap().iter().position(|&c| c == '.').unwrap(),
    );

    let mut points = vec![start, end];

    for (r, row) in grid.iter().enumerate() {
        for (c, &ch) in row.iter().enumerate() {
            if ch == '#' {
                continue;
            }
            let mut neighbors = 0;
            for &(dr, dc) in [(-1, 0), (1, 0), (0, -1), (0, 1)].iter()
            {
                let (nr, nc) = (r as isize + dr, c as isize + dc);
                if nr >= 0
                    && (nr as usize) < grid.len()
                    && nc >= 0
                    && (nc as usize) < grid[0].len()
                    && grid[nr as usize][nc as usize] != '#'
                {
                    neighbors += 1;
                }
            }
            if neighbors >= 3 {
                points.push((r, c));
            }
        }
    }

    let mut graph: HashMap<
        (usize, usize),
        HashMap<(usize, usize), usize>,
    > = HashMap::new();
    for &point in &points {
        graph.insert(point, HashMap::new());
    }

    for &(sr, sc) in &points {
        let mut stack = vec![(0, sr, sc)];
        let mut seen = HashSet::new();
        seen.insert((sr, sc));

        while let Some((n, r, c)) = stack.pop() {
            if n != 0 && points.contains(&(r, c)) {
                graph.get_mut(&(sr, sc)).unwrap().insert((r, c), n);
                continue;
            }

            for &(dr, dc) in [(-1, 0), (1, 0), (0, -1), (0, 1)].iter()
            {
                let (nr, nc) = (r as isize + dr, c as isize + dc);
                if nr >= 0
                    && (nr as usize) < grid.len()
                    && nc >= 0
                    && (nc as usize) < grid[0].len()
                    && grid[nr as usize][nc as usize] != '#'
                    && !seen.contains(&(nr as usize, nc as usize))
                {
                    stack.push((n + 1, nr as usize, nc as usize));
                    seen.insert((nr as usize, nc as usize));
                }
            }
        }
    }

    println!(
        "{}",
        dfs(&start, &end, &graph, &mut HashSet::new()).unwrap_or(0)
    );
}

fn dfs(
    pt: &(usize, usize),
    end: &(usize, usize),
    graph: &HashMap<(usize, usize), HashMap<(usize, usize), usize>>,
    seen: &mut HashSet<(usize, usize)>,
) -> Option<isize> {
    if pt == end {
        return Some(0);
    }

    if !seen.insert(*pt) {
        return None;
    }

    let mut max_dist = None;
    if let Some(neighbors) = graph.get(pt) {
        for (&next_pt, &dist) in neighbors {
            if let Some(sub_dist) = dfs(&next_pt, end, graph, seen) {
                let total_dist = sub_dist + dist as isize;
                max_dist =
                    Some(max_dist.map_or(total_dist, |m| -> isize {
                        m.max(total_dist)
                    }));
            }
        }
    }

    seen.remove(pt);
    max_dist
}
