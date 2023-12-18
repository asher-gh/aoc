pub fn gcd(a: usize, b: usize) -> usize {
    if a == b {
        return a;
    }

    let (a, b) = (a.max(b), a.min(b));

    if a > b && a % b == 0 {
        return b;
    }

    return gcd(b, a % b);
}

pub fn lcm(a: usize, b: usize) -> usize {
    // a*b = lcm(a,b) * gcd(a,b)
    a * b / gcd(a, b)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_gcd() {
        assert_eq!(gcd(2, 6), 2);
    }

    #[test]
    fn test_lcm() {
        assert_eq!(lcm(100, 75), 300);

        let nums = [12, 48, 64];
        assert_eq!(nums.into_iter().fold(nums[0], |a, b| lcm(a, b)), 192);
    }
}
