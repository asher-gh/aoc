use std::{collections::BTreeSet, path::PathBuf};

fn main() {
    let input = if cfg!(debug_assertions) {
        r#"
    467..114..
    ...*......
    ..35..633.
    ......#...
    617*......
    .....+.58.
    ..592.....
    ......755.
    ...$.*....
    .664.598..
    "#
        .to_string()
    } else {
        let input_path = format!("{}/input_d3", std::env!("CARGO_MANIFEST_DIR"));
        std::fs::read_to_string(PathBuf::from(input_path)).unwrap()
    };

    /* Goal: find special characters and get the surrounding box(3x3 matrix).
    * 1. Find special chars
    * 2. Find surrounding numbers
    * 3. Greedily seek numbers
    * 4. Push to a BTreeSet
    * 5. Sum everything in the set
    *   ┌───────┐ ┌───────────────┐
    *   │ 7 . . │ │[0,2][0,3][0,4]│
    *   │ . * . │ │[1,2][1,3][1,4]│
    *   │ 3 5 . │ │[2,2][2,3][2,4]│
    *   └───────┘ └───────────────┘

    */

    let matrix: Vec<Vec<char>> = to_matrix(&input);

    if cfg!(debug_assertions) {
        print_matrix(&matrix);
    }

    p1(&matrix);
    p2(&matrix);
}

fn p1(matrix: &Vec<Vec<char>>) {
    let mut sum = 0;

    for (i, row) in matrix.iter().enumerate() {
        for (j, c) in row.iter().enumerate() {
            if *c != '.' && !c.is_numeric() {
                let surrounding_box = surrounding_box(&matrix, (i, j));
                let mut surrounding_nums = BTreeSet::<u32>::new();

                for (s, e) in &surrounding_box {
                    for i in s.1..=e.1 {
                        if matrix[s.0][i].is_numeric() {
                            surrounding_nums.insert(seek_num(&matrix, (s.0, i)));
                        }
                    }
                }

                sum += surrounding_nums.iter().sum::<u32>();
            }
        }
    }

    println!("Part 1: {}", sum);
}

fn p2(matrix: &Vec<Vec<char>>) {
    let mut sum = 0;
    for (i, row) in matrix.iter().enumerate() {
        for (j, c) in row.iter().enumerate() {
            if *c == '*' {
                let surrounding_box = surrounding_box(&matrix, (i, j));
                let mut surrounding_nums = BTreeSet::<u32>::new();

                for (s, e) in &surrounding_box {
                    for i in s.1..=e.1 {
                        if matrix[s.0][i].is_numeric() {
                            surrounding_nums.insert(seek_num(&matrix, (s.0, i)));
                        }
                    }
                }

                if surrounding_nums.len() == 2 {
                    sum += surrounding_nums.iter().product::<u32>();
                }
            }
        }
    }

    println!("Part 2: {}", sum);
}

fn surrounding_box(
    m: &Vec<Vec<char>>,
    pos: (usize, usize),
) -> Vec<((usize, usize), (usize, usize))> {
    let (width, height) = (m[0].len(), m.len());
    let mut out = vec![];

    if pos.0 > 0 {
        out.push((
            (pos.0 - 1, (pos.1 as i32 - 1).max(0) as usize),
            (pos.0 - 1, (pos.1 + 1).min(width - 1)),
        ));
    }

    out.push((
        (pos.0, (pos.1 as i32 - 1).max(0) as usize),
        (pos.0, (pos.1 + 1).min(width - 1)),
    ));

    if pos.0 < height - 1 {
        out.push((
            (pos.0 + 1, (pos.1 as i32 - 1).max(0) as usize),
            (pos.0 + 1, (pos.1 + 1).min(width - 1)),
        ));
    }

    if cfg!(debug_assertions) {
        print_surrounding_box(&m, &out);
    }

    out
}

fn print_surrounding_box(m: &Vec<Vec<char>>, pos: &[((usize, usize), (usize, usize))]) {
    // Fancy print the box
    println!("┌{:─^w$}┐", "", w = 3 * 2 + 1);
    for (s, e) in pos {
        let row: String = (s.1..=e.1)
            .map(|i| m[s.0][i].to_string())
            .collect::<Vec<String>>()
            .join(" ");
        println!("│ {:<6}│", row);
    }
    println!("└{:─^w$}┘", "", w = 3 * 2 + 1);
}

fn to_matrix(input: &str) -> Vec<Vec<char>> {
    input
        .trim()
        .lines()
        .map(|line| (line.trim().chars().collect()))
        .collect()
}

fn print_matrix(m: &Vec<Vec<char>>) {
    println!("┌{:─^width$}┐", "┤matrix├", width = m[0].len() * 2 + 1);
    for (i, row) in m.iter().enumerate() {
        print!("│ ");
        for (_j, c) in row.iter().enumerate() {
            print!("{c} ");
        }
        print!("│");

        print!("{:5}", "");

        for (j, _c) in row.iter().enumerate() {
            print!("[{i},{j}]");
        }

        println!("");
    }
    println!("└{:─<width$}┘", "", width = m[0].len() * 2 + 1);
}

fn seek_num(m: &Vec<Vec<char>>, pos: (usize, usize)) -> u32 {
    let mut s = String::from(m[pos.0][pos.1]);
    // go left
    for i in (0..pos.1).rev() {
        let c = m[pos.0][i];
        if !c.is_numeric() {
            break;
        };
        s.insert(0, c);
    }
    // go right
    for i in pos.1 + 1..m[0].len() {
        let c = m[pos.0][i];
        if !c.is_numeric() {
            break;
        };
        s.push(c);
    }

    s.parse().unwrap_or(0)
}
