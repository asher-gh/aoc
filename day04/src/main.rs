use nom::{
    bytes::complete::{take, take_until},
    IResult,
};
use std::collections::{BTreeMap, BTreeSet};

fn main() {
    let input = if cfg!(debug_assertions) {
        r#"
        Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
        Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
        Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
        Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
        Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
        Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
        "#
    } else {
        include_str!(concat!(std::env!("CARGO_MANIFEST_DIR"), "/d4_input"))
    };

    let cards = parse_cards(input);
    p1(&cards);
    p2(&cards);
}

fn parse_cards(input: &str) -> Vec<(BTreeSet<u32>, Vec<u32>)> {
    let mut out = Vec::new();

    for line in input.trim().lines() {
        if let Ok((_, card)) = parse_card(line) {
            out.push(card)
        }
    }

    out
}

fn p1(cards: &Vec<(BTreeSet<u32>, Vec<u32>)>) {
    let mut total_points = 0;

    for (w, nums) in cards {
        let mut points = 0;
        for n in nums {
            if w.contains(n) {
                if points == 0 {
                    points = 1;
                } else {
                    points *= 2;
                }
            }
        }
        total_points += points;
    }

    println!("Part 1: {}", total_points);
}

#[derive(Default, Debug)]
struct Card {
    matches: usize,
    copies: usize,
}

fn matches(card: &(BTreeSet<u32>, Vec<u32>)) -> usize {
    card.1.iter().filter(|&x| card.0.contains(x)).count()
}

fn p2(cards: &Vec<(BTreeSet<u32>, Vec<u32>)>) {
    let mut cards: BTreeMap<usize, Card> = cards
        .iter()
        .enumerate()
        .map(|(i, card)| {
            (
                i,
                Card {
                    matches: matches(card),
                    copies: 1,
                },
            )
        })
        .collect();

    for i in 0..cards.len() {
        if let Some(card) = cards.get_mut(&i) {
            let copies = card.copies;
            for j in 1..=card.matches {
                cards.entry(i + j).and_modify(|card| card.copies += copies);
            }
        }
    }

    let total_cards = cards.iter().fold(0, |a, (_, card)| a + card.copies);
    println!("Part 2: {total_cards}");
}

fn parse_card(card: &str) -> IResult<&str, (BTreeSet<u32>, Vec<u32>)> {
    let (line, _) = take_until(": ")(card)?;
    let (line, _) = take(2u8)(line)?;

    let split: Vec<&str> = line.split("|").collect();

    let set: BTreeSet<u32> = split[0]
        .split(" ")
        .filter(|c| !c.is_empty())
        .map(|n| n.parse().unwrap())
        .collect();
    let v: Vec<u32> = split[1].split(" ").filter_map(|n| n.parse().ok()).collect();

    Ok(("", (set, v)))
}
