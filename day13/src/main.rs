use std::{fs, path::Path};

use anyhow::Result;

fn main() -> Result<()> {
    let input_file = if cfg!(debug_assertions) {
        "test.txt"
    } else {
        "input.txt"
    };

    let input = fs::read_to_string(Path::new(env!("CARGO_MANIFEST_DIR")).join(input_file))?;

    let patterns: Vec<&str> = input.split("\n\n").collect();

    let mut total: usize = patterns
        .iter()
        .filter_map(|&x| row_index(x))
        .map(|x| x * 100)
        .sum();

    total += patterns.iter().filter_map(|&x| col_index(x)).sum::<usize>();

    println!("Part 1: {total}");

    Ok(())
}

fn row_index(pat: &str) -> Option<usize> {
    let lines: Vec<String> = pat.lines().map(|s| s.to_owned()).collect();

    reflection_index(lines)
}

fn col_index(pat: &str) -> Option<usize> {
    #[cfg(debug_assertions)]
    {
        println!("COLUMN");
        println!("{pat}");
    }

    let lines = pat.lines().collect::<Vec<&str>>();

    let cols: Vec<String> = (0..lines[0].len()).fold(Vec::new(), |mut a, i| {
        a.push(lines.iter().fold(String::new(), |mut b: String, line| {
            b.push(line.chars().nth(i).unwrap());
            b
        }));
        a
    });

    reflection_index(cols)
}

fn reflection_index(lines: Vec<String>) -> Option<usize> {
    #[cfg(debug_assertions)]
    {
        println!("REFLECTION");

        for (i, line) in lines.iter().enumerate() {
            println!("{i}:\t{line}");
        }
    }
    let mut result = 0;

    let mut indices: Vec<usize> = vec![];

    for (i, pair) in lines.windows(2).enumerate() {
        if pair[0] == pair[1] {
            indices.push(i);
        }
    }

    for index in indices {
        for i in 0..=index {
            if index + i + 1 == lines.len() {
                result += index + 1;
                break;
            }
            if lines[index - i] != lines[index + i + 1] {
                break;
            }
            if index == i {
                result += index + 1;
            }
        }
    }

    if result == 0 {
        None
    } else {
        Some(result)
    }
}
