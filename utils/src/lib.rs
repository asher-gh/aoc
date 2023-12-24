use std::time::{Duration, Instant};

pub fn time<T, F: FnMut() -> T>(mut f: F) -> (Duration, T) {
    let start = Instant::now();
    let res = f();
    (start.elapsed(), res)
}
