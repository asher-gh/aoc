use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{digit1, multispace1},
    combinator::{map, map_res},
    error::ErrorKind,
    multi::separated_list0,
    sequence::pair,
    IResult,
};
use std::path::PathBuf;

fn main() {
    let input_path = format!("{}/input.txt", std::env!("CARGO_MANIFEST_DIR"));
    let inputs = std::fs::read_to_string(PathBuf::from(input_path)).unwrap();

    println!("p1: {}", p1(&inputs));
    println!("p2: {}", p2(&inputs));
}

fn p1(inputs: &str) -> usize {
    let mut res = 0;

    for input in inputs.trim().lines() {
        if let Ok((_, game)) = parse_valid_games(input.trim()) {
            res += game.id;
        }
    }

    res
}

fn p2(inputs: &str) -> usize {
    let mut res = 0;

    for input in inputs.trim().lines() {
        if let Ok((_, game)) = parse_min_cubes(input) {
            res += game.red * game.blue * game.green
        }
    }

    res
}

#[derive(Debug, Default)]
struct Game {
    id: usize,
    red: usize,
    blue: usize,
    green: usize,
}

fn parse_min_cubes(input: &str) -> IResult<&str, Game> {
    let mut game = Game::default();

    let (input, _) = tag("Game ")(input)?;
    let (input, id) = map_res(digit1, |s: &str| s.parse::<usize>())(input)?;

    game.id = id;

    let (input, _) = tag(": ")(input)?;

    let color_pair = map(
        pair(
            pair(digit1, multispace1),
            alt((tag("blue"), tag("red"), tag("green"))),
        ),
        |x| (x.0 .0, x.1),
    );

    let (_, colors) = separated_list0(alt((tag(", "), tag("; "))), color_pair)(input)?;

    for (count, color) in colors {
        let count = usize::from_str_radix(count, 10).unwrap();
        match color {
            "red" => game.red = count.max(game.red),
            "green" => game.green = count.max(game.green),
            "blue" => game.blue = count.max(game.blue),
            _ => {
                return Err(nom::Err::Error(nom::error::Error {
                    input,
                    code: ErrorKind::Tag,
                }))
            }
        }
    }

    Ok((input, game))
}

fn parse_valid_games(input: &str) -> IResult<&str, Game> {
    let mut game = Game::default();

    let (input, _) = tag("Game ")(input)?;
    let (input, id) = map_res(digit1, |s: &str| s.parse::<usize>())(input)?;

    game.id = id;

    let (input, _) = tag(": ")(input)?;

    let color_pair = map(
        pair(
            pair(digit1, multispace1),
            alt((tag("blue"), tag("red"), tag("green"))),
        ),
        |x| (x.0 .0, x.1),
    );

    let (_, colors) = separated_list0(alt((tag(", "), tag("; "))), color_pair)(input)?;

    for (count, color) in colors {
        let count = usize::from_str_radix(count, 10).unwrap();
        match color {
            "red" if count <= 12 => game.red += count,
            "green" if count <= 13 => game.green += count,
            "blue" if count <= 14 => game.blue += count,
            _ => {
                return Err(nom::Err::Error(nom::error::Error {
                    input,
                    code: ErrorKind::Tag,
                }))
            }
        }
    }

    Ok((input, game))
}
