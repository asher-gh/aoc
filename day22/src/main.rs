use std::collections::{HashMap, HashSet, VecDeque};
use utils::time;

fn main() {
    let input = if cfg!(debug_assertions) {
        r#" 
    1,0,1~1,2,1
    0,0,2~2,0,2
    0,2,3~2,2,3
    0,0,4~0,2,4
    2,0,5~2,2,5
    0,1,6~2,1,6
    1,1,8~1,1,9
    "#
    } else {
        include_str!("../input")
    }
    .trim();

    let (p1_time, p1) = time(|| p1(&input));
    let (p2_time, p2) = time(|| p2(&input));

    println!("P1: {p1}, ({p1_time:?})");
    println!("P2: {p2}, ({p2_time:?})");
}

fn p1(input: &str) -> u32 {
    let mut bricks = parse_input(&input);
    drop_bricks(&mut bricks);

    let mut k_supports_v: HashMap<usize, HashSet<usize>> =
        (0..bricks.len()).map(|i| (i, HashSet::new())).collect();
    let mut v_supports_k = k_supports_v.clone();

    for (j, upper) in bricks.iter().enumerate() {
        for (i, lower) in bricks[..j].iter().enumerate() {
            if overlap(lower, upper) && upper[2] == lower[5] + 1 {
                k_supports_v.get_mut(&i).unwrap().insert(j);
                v_supports_k.get_mut(&j).unwrap().insert(i);
            }
        }
    }

    let mut res = 0;

    for (i, _) in bricks.iter().enumerate() {
        if k_supports_v
            .get(&i)
            .unwrap()
            .iter()
            .all(|j| v_supports_k.get(j).unwrap().len() >= 2)
        {
            res += 1;
        }
    }

    res
}

fn p2(input: &str) -> u32 {
    let mut bricks = parse_input(&input);
    drop_bricks(&mut bricks);

    let mut k_supports_v: HashMap<usize, HashSet<usize>> =
        (0..bricks.len()).map(|i| (i, HashSet::new())).collect();
    let mut v_supports_k = k_supports_v.clone();

    for (j, upper) in bricks.iter().enumerate() {
        for (i, lower) in bricks[..j].iter().enumerate() {
            if overlap(lower, upper) && upper[2] == lower[5] + 1 {
                k_supports_v.get_mut(&i).unwrap().insert(j);
                v_supports_k.get_mut(&j).unwrap().insert(i);
            }
        }
    }

    let mut res = 0;

    for (i, _) in bricks.iter().enumerate() {
        let mut q: VecDeque<usize> = k_supports_v
            .get(&i)
            .unwrap()
            .iter()
            .filter_map(|j| {
                if v_supports_k.get(j).unwrap().len() == 1 {
                    Some(*j)
                } else {
                    None
                }
            })
            .collect();

        let mut falling: HashSet<usize> =
            q.clone().into_iter().collect();

        falling.insert(i);

        while let Some(j) = q.pop_front() {
            let set_diff = k_supports_v
                .get(&j)
                .unwrap()
                .difference(&falling)
                .map(|x| *x)
                .collect::<HashSet<usize>>();

            for k in set_diff {
                if v_supports_k.get(&k).unwrap().is_subset(&falling) {
                    q.push_back(k);
                    falling.insert(k);
                }
            }
        }

        res += falling.len() - 1
    }

    res as u32
}

fn overlap(a: &[u32], b: &[u32]) -> bool {
    a[0].max(b[0]) <= a[3].min(b[3])
        && a[1].max(b[1]) <= a[4].min(b[4])
}

fn parse_input(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|line| {
            line.trim()
                .replace("~", ",")
                .split(",")
                .filter_map(|d| d.parse().ok())
                .collect::<Vec<u32>>()
        })
        .collect()
}

fn sort_bricks(bricks: &mut Vec<Vec<u32>>) {
    bricks.sort_by_key(|x| x[2]);
}

fn drop_bricks(bricks: &mut Vec<Vec<u32>>) {
    sort_bricks(bricks);
    for i in 1..bricks.len() {
        let mut max_z = 1;
        for brick_below in &bricks[..i] {
            if overlap(&bricks[i], brick_below) {
                max_z = max_z.max(brick_below[5] + 1); // note
            }
        }
        let brick = &mut bricks[i];
        brick[5] -= brick[2] - max_z;
        brick[2] = max_z;
    }

    sort_bricks(bricks);
}
