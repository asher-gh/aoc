use std::collections::HashMap;

struct Maze {
    matrix: Vec<Vec<char>>,
    start: Option<Cursor>,
}

impl Maze {
    fn new(input: &str) -> Self {
        let matrix: Vec<Vec<char>> = input
            .trim()
            .lines()
            .map(|line| line.trim().chars().collect())
            .collect();

        let start = matrix
            .clone()
            .into_iter()
            .enumerate()
            .map(|(i, row)| {
                row.into_iter()
                    .enumerate()
                    .map(|(j, c)| (i, j, c))
                    .collect::<Vec<(usize, usize, char)>>()
            })
            .flatten()
            .find(|(_, _, c)| *c == '*')
            .map(|(i, j, _)| Cursor {
                pos: (i, j),
                dir: None,
            });

        Maze { matrix, start }
    }

    fn print(&self) {
        for row in &self.matrix {
            for c in row {
                print!("{c}");
            }
            println!();
        }
    }

    fn walk(&mut self) -> char {
        if let Some(cursor) = &mut self.start {
            cursor.walk(&self.matrix)
        } else {
            '*'
        }
    }
}

#[derive(Copy, Clone, Debug)]
enum Direction {
    North,
    South,
    East,
    West,
}

#[derive(Default, Debug)]
struct Cursor {
    pos: (usize, usize),
    dir: Option<Direction>,
}

impl Cursor {
    fn walk(&mut self, maze: &[Vec<char>]) -> char {
        use Direction::*;
        let (i, j) = self.pos;
        let dir = self.dir;
        let (width, height) = (maze[0].len(), maze.len());
        let def = ((i, j), dir);
        (self.pos, self.dir) = match maze[i][j] {
            '│' => match dir {
                Some(North) if i > 0 => ((i - 1, j), dir),
                Some(South) if i < height - 1 => ((i + 1, j), dir),
                _ => def,
            },
            '─' => match dir {
                Some(East) if j < width - 1 => ((i, j + 1), dir),
                Some(West) if j > 0 => ((i, j - 1), dir),
                _ => def,
            },
            '└' => match dir {
                Some(South) if j < width - 1 => ((i, j + 1), Some(East)),
                Some(West) if i > 0 => ((i - 1, j), Some(North)),
                _ => def,
            },
            '┘' => match dir {
                Some(South) if j > 0 => ((i, j - 1), Some(West)),
                Some(East) if i > 0 => ((i - 1, j), Some(North)),
                _ => def,
            },
            '┐' => match dir {
                Some(East) if i < height - 1 => ((i + 1, j), Some(South)),
                Some(North) if j > 0 => ((i, j - 1), Some(West)),
                _ => def,
            },
            '┌' => match dir {
                Some(North) if j < width - 1 => ((i, j + 1), Some(East)),
                Some(West) if i < height - 1 => ((i + 1, j), Some(South)),
                _ => def,
            },
            '*' if dir.is_none() => {
                if i > 0 && maze[i - 1][j] != '.' {
                    ((i - 1, j), Some(North))
                } else if i < height && maze[i + 1][j] != '.' {
                    ((i + 1, j), Some(South))
                } else if j > 0 && maze[i][j - 1] != '.' {
                    ((i, j - 1), Some(West))
                } else if j < width && maze[i][j + 1] != '.' {
                    ((i, j + 1), Some(East))
                } else {
                    def
                }
            }
            _ => def,
        };

        maze[self.pos.0][self.pos.1]
    }
}

fn main() {
    let input = if cfg!(debug_assertions) {
        r#"
        ..F7.
        .FJ|.
        SJ.L7
        |F--J
        LJ...
        "#
    } else {
        include_str!(concat!(std::env!("CARGO_MANIFEST_DIR"), "/input.txt"))
    };

    let input = HashMap::from([
        ("|", "│"),
        ("-", "─"),
        ("L", "└"),
        ("J", "┘"),
        ("7", "┐"),
        ("F", "┌"),
        ("S", "*"),
    ])
    .into_iter()
    .fold(input.to_owned(), |input, (k, v)| input.replace(k, v));

    let mut maze = Maze::new(&input);
    let mut length = 1;

    // dbg!(maze.start);

    while maze.walk() != '*' {
        length += 1;
    }

    println!("Part 1: {}", length / 2);
}
