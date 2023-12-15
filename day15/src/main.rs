use std::collections::HashMap;

use nom::{
    bytes::complete::take_while1,
    character::complete::{char, digit1},
    combinator::map_res,
    IResult,
};

fn main() {
    let input = if cfg!(debug_assertions) {
        r#"
rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7
    "#
    } else {
        include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/input.txt"))
    };

    let codes: Vec<&str> = input.trim().split(",").collect();

    println!("Part One: {}", p1(&codes));
    println!("Part One: {}", p2(&codes));
}

fn p1(codes: &[&str]) -> usize {
    codes.iter().map(|code| hash(code) as usize).sum()
}

fn p2(codes: &[&str]) -> usize {
    // Box# -> [(code, focal_length)]
    let mut map: HashMap<usize, Vec<(&str, u8)>> = HashMap::new();

    for code in codes {
        if let Ok((_, (label, op, focal_len))) = parse_code(code) {
            let hash = hash(label);
            match op {
                '-' => {
                    if let Some(boxes) = map.get_mut(&(hash as usize)) {
                        *boxes = boxes
                            .to_owned()
                            .into_iter()
                            .filter(|xl| xl.0 != label)
                            .collect();
                    }
                }
                '=' => {
                    let focal_len = focal_len.unwrap() as u8;
                    if let Some(boxes) = map.get_mut(&(hash as usize)) {
                        if let Some(x) = boxes.into_iter().find(|(l, fl)| l == &label) {
                            x.1 = focal_len;
                        } else {
                            boxes.push((label, focal_len))
                        }
                    } else {
                        map.insert(hash as usize, vec![(label, focal_len)]);
                    }
                }
                _ => {}
            }
        }
    }

    map.iter()
        .map(|(k, v)| {
            v.iter()
                .enumerate()
                .fold(0, |a, (i, (_, fl))| a + (k + 1) * (i + 1) * *fl as usize)
        })
        .sum()
}

fn hash(code: &str) -> u8 {
    code.chars().fold(0, |mut a, c| {
        a += c as usize;
        a *= 17;
        a %= 256;
        a
    }) as u8
}

fn parse_code(input: &str) -> IResult<&str, (&str, char, Option<usize>)> {
    let (input, label) = take_while1(|c: char| c.is_alphabetic())(input)?;
    let (input, operation) =
        char::<&str, nom::error::Error<&str>>('=')(input).or(char('-')(input))?;

    let (input, focal_length) = if operation == '=' {
        let (input, num) = map_res(digit1, str::parse::<usize>)(input)?;

        (input, Some(num))
    } else {
        (input, None)
    };

    Ok((input, (label, operation, focal_length)))
}
