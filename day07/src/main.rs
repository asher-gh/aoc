use std::{cmp::Ordering, collections::HashMap};

fn main() {
    let input = if cfg!(debug_assertions) {
        r#"
        32T3K 765
        T55J5 684
        KK677 28
        KTJJT 220
        QQQJA 483
        "#
    } else {
        include_str!(concat!(std::env!("CARGO_MANIFEST_DIR"), "/d7input"))
    };

    p2(input);
}

fn p2(input: &str) {
    let mut plays: Vec<Play> = input
        .trim()
        .lines()
        .map(|line| {
            let vals = line.split_whitespace().collect::<Vec<&str>>();
            Play {
                hand: Hand::from(vals[0]),
                bid: vals[1].parse().unwrap(),
            }
        })
        .collect();

    plays.sort_unstable_by_key(|x| x.hand);

    let result: u32 = plays
        .into_iter()
        .enumerate()
        .fold(0, |a, (i, Play { bid, .. })| a + bid * (i as u32 + 1));

    println!("Part 2: {result}");
}

#[derive(PartialEq, Debug)]
struct Play {
    hand: Hand,
    bid: u32,
}

impl PartialOrd for Play {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.hand.partial_cmp(&other.hand)
    }
}

#[repr(u8)]
#[derive(PartialEq, Debug, Eq, Ord, Copy, Clone)]
enum Hand {
    None,
    HighCard([u8; 5]),
    OnePair([u8; 5]),
    TwoPair([u8; 5]),
    ThreeOfAKind([u8; 5]),
    FullHouse([u8; 5]),
    FourOfAKind([u8; 5]),
    FiveOfAKind([u8; 5]),
}

impl Hand {
    fn values(&self) -> [u8; 5] {
        use Hand::*;

        match self {
            HighCard(x) => *x,
            OnePair(x) => *x,
            TwoPair(x) => *x,
            ThreeOfAKind(x) => *x,
            FullHouse(x) => *x,
            FourOfAKind(x) => *x,
            FiveOfAKind(x) => *x,
            _ => [0; 5],
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.eq(other) {
            for (s, o) in self.values().into_iter().zip(other.values()) {
                if s > o {
                    return Some(Ordering::Greater);
                } else if s < o {
                    return Some(Ordering::Less);
                }
            }
            Some(Ordering::Equal)
        } else {
            Some(self.cmp(other))
        }
    }
}

impl From<&str> for Hand {
    fn from(value: &str) -> Self {
        let mut char_set = value
            .chars()
            .fold(HashMap::new(), |mut a: HashMap<char, usize>, c| {
                a.entry(c).and_modify(|count| *count += 1).or_insert(1);
                a
            });

        let val: [u8; 5] = value
            .chars()
            .map(|c| match c {
                c if c.is_numeric() => c.to_digit(10).unwrap() as u8,
                'T' => 10,
                'J' => 1,
                'Q' => 12,
                'K' => 13,
                'A' => 14,
                _ => 0,
            })
            .collect::<Vec<u8>>()
            .try_into()
            .unwrap();

        // For `JJJJJ`
        if char_set.len() > 1 {
            if let Some(jct) = char_set.remove(&'J') {
                if let Some(k) = char_set.iter().max_by_key(|(_, &ct)| ct).map(|(k, _)| k) {
                    char_set.entry(*k).and_modify(|x| *x += jct);
                };
            };
        }

        use Hand::*;
        match char_set.len() {
            5 => HighCard(val),
            4 => OnePair(val),
            3 => {
                if char_set.iter().find(|(_, &count)| count > 2).is_some() {
                    ThreeOfAKind(val)
                } else {
                    TwoPair(val)
                }
            }
            2 => {
                if char_set.iter().find(|(_, &count)| count > 3).is_some() {
                    FourOfAKind(val)
                } else {
                    FullHouse(val)
                }
            }
            1 => FiveOfAKind(val),
            _ => None,
        }
        // todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const IN_HIGH_CARD: &str = "23456";
    const IN_ONE_PAIR: &str = "A23A4";
    const IN_TWO_PAIR: &str = "23432";
    const IN_THREE_KIND: &str = "TTT98";
    const IN_FULL_HOUSE: &str = "23332";
    const IN_FOUR_KIND: &str = "AA8AA";
    const IN_FIVE_KIND: &str = "AAAAA";

    #[test]
    fn hand_parsing() {
        let hand_high_card: Hand = Hand::from(IN_HIGH_CARD);
        let hand_one_pair: Hand = Hand::from(IN_ONE_PAIR);
        let hand_two_pair: Hand = Hand::from(IN_TWO_PAIR);
        let hand_three_kind: Hand = Hand::from(IN_THREE_KIND);
        let hand_full_house: Hand = Hand::from(IN_FULL_HOUSE);
        let hand_four_kind: Hand = Hand::from(IN_FOUR_KIND);
        let hand_five_kind: Hand = Hand::from(IN_FIVE_KIND);
        assert_eq!(hand_high_card, Hand::HighCard([2, 3, 4, 5, 6]));
        assert_eq!(hand_one_pair, Hand::OnePair([14, 2, 3, 14, 4]));
        assert_eq!(hand_two_pair, Hand::TwoPair([2, 3, 4, 3, 2]));
        assert_eq!(hand_three_kind, Hand::ThreeOfAKind([10, 10, 10, 9, 8]));
        assert_eq!(hand_full_house, Hand::FullHouse([2, 3, 3, 3, 2]));
        assert_eq!(hand_four_kind, Hand::FourOfAKind([14, 14, 8, 14, 14]));
        assert_eq!(hand_five_kind, Hand::FiveOfAKind([14, 14, 14, 14, 14]));
    }

    #[test]
    fn hand_ordering() {
        // different hands
        let hand_high_card: Hand = Hand::from(IN_HIGH_CARD);
        let hand_one_pair: Hand = Hand::from(IN_ONE_PAIR);
        let hand_two_pair: Hand = Hand::from(IN_TWO_PAIR);
        let hand_three_kind: Hand = Hand::from(IN_THREE_KIND);
        let hand_full_house: Hand = Hand::from(IN_FULL_HOUSE);
        let hand_four_kind: Hand = Hand::from(IN_FOUR_KIND);
        let hand_five_kind: Hand = Hand::from(IN_FIVE_KIND);
        assert!(hand_high_card < hand_one_pair);
        assert!(hand_one_pair < hand_two_pair);
        assert!(hand_two_pair < hand_three_kind);
        assert!(hand_three_kind < hand_full_house);
        assert!(hand_full_house < hand_four_kind);
        assert!(hand_four_kind < hand_five_kind);

        // same hands
        let hand1 = Hand::from("KK677");
        let hand2 = Hand::from("KTJJT"); // KTTTT
        assert!(hand1 < hand2);
        let hand1 = Hand::from("T55J5"); // T5555
        let hand2 = Hand::from("QQQJA"); // QQQQA
        assert!(hand1 < hand2);
    }
}
