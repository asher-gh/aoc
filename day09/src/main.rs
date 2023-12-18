fn main() {
    let input = if cfg!(debug_assertions) {
        r#"
            0 3 6 9 12 15
            1 3 6 10 15 21
            10 13 16 21 30 45
        "#
    } else {
        include_str!(concat!(std::env!("CARGO_MANIFEST_DIR"), "/d9input"))
    };

    let input: Vec<_> = input
        .trim()
        .lines()
        .map(|line| {
            line.trim()
                .split_whitespace()
                .map(|n| n.parse().unwrap())
                .collect()
        })
        .collect();

    p1(&input);
    p2(&input);
}

fn p1(input: &[Vec<i32>]) {
    let sum = input.iter().fold(0, |a, slc| a + extrapolate(slc));

    println!("Part 1: {sum}");
}
fn p2(input: &[Vec<i32>]) {
    let sum = input.iter().fold(0, |a, slc| a + extrapolate_back(slc));

    println!("Part 2: {sum}");
}

fn extrapolate(seq: &[i32]) -> i32 {
    if seq.is_empty() || seq.into_iter().fold(true, |a, n| a && *n == 0) {
        return 0;
    }

    let diff_seq: Vec<_> = seq.windows(2).map(|w| (w[1] - w[0])).collect();
    seq.last().unwrap() + extrapolate(&diff_seq)
}

fn extrapolate_back(seq: &[i32]) -> i32 {
    if seq.is_empty() || seq.into_iter().fold(true, |a, n| a && *n == 0) {
        return 0;
    }

    let diff_seq: Vec<_> = seq.windows(2).map(|w| (w[1] - w[0])).collect();
    seq.first().unwrap() - extrapolate_back(&diff_seq)
}

#[cfg(test)]
mod tests {
    use crate::{extrapolate, extrapolate_back};

    #[test]
    fn test_extrapolate() {
        assert_eq!(
            extrapolate(&[
                4, 13, 31, 69, 161, 382, 885, 1967, 4170, 8430, 16326, 30587, 56237, 103154,
                191459, 362164, 697242, 1356689, 2647559, 5156471
            ]),
            10012899
        );
        assert_eq!(
            extrapolate(&[
                29, 42, 53, 56, 39, -4, -34, 127, 1001, 3868, 11543, 29740, 69331, 149906, 305152,
                590695, 1095189, 1955590, 3377721, 5663416,
            ]),
            9245727
        );
    }

    #[test]
    fn test_extrapolate_back() {
        assert_eq!(extrapolate_back(&[10, 13, 16, 21, 30, 45]), 5);
    }
}
