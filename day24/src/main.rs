use std::ops::RangeInclusive;

#[derive(Debug)]
pub struct Hail {
    sx: f64,
    sy: f64,
    sz: f64,
    vx: f64,
    vy: f64,
    vz: f64,
    a: f64,
    b: f64,
    c: f64,
}

impl Hail {
    fn new(
        sx: f64,
        sy: f64,
        sz: f64,
        vx: f64,
        vy: f64,
        vz: f64,
    ) -> Self {
        Hail {
            sx,
            sy,
            sz,
            vx,
            vy,
            vz,
            a: vy,
            b: -vx,
            c: (vy * sx) - (vx * sy),
        }
    }
}

static BOUNDS: RangeInclusive<f64> = if cfg!(debug_assertions) {
    7_f64..=27_f64
} else {
    200000000000000f64..=400000000000000_f64
};

fn main() {
    let input = if cfg!(debug_assertions) {
        r#"
        19, 13, 30 @ -2,  1, -2
        18, 19, 22 @ -1, -1, -2
        20, 25, 34 @ -2, -2, -4
        12, 31, 28 @ -1, -2, -1
        20, 19, 15 @  1, -5, -3
        "#
    } else {
        include_str!("../input")
    }
    .trim();

    println!("Part I: {}", p1(input));
}

fn p1(input: &str) -> usize {
    let input = input
        .lines()
        .filter_map(|line| {
            let spl = line.trim().replace("@", ",");
            let spl = spl
                .split(",")
                .filter_map(|c| c.trim().parse::<f64>().ok());

            if let [sx, sy, sz, vx, vy, vz] =
                spl.collect::<Vec<_>>()[..]
            {
                Some(Hail::new(sx, sy, sz, vx, vy, vz))
            } else {
                None
            }
        })
        .collect::<Vec<Hail>>();

    let mut ans = 0usize;

    for (i, h1) in input.iter().enumerate() {
        // only check for non-parallel lines
        for h2 in &input[..i] {
            let pc = (h1.a * h2.b) - (h2.a * h1.b);
            if pc as i8 != 0 {
                let x = ((h1.c * h2.b) - (h2.c * h1.b)) as f64
                    / pc as f64;
                let y = ((h1.a * h2.c) - (h2.a * h1.c)) as f64
                    / pc as f64;

                // check bounds
                if [x, y].iter().all(|a| BOUNDS.contains(a)) {
                    // check if intersected in past
                    if [h1, h2].iter().all(|&h| {
                        ((x - h.sx) * h.vx).is_sign_positive()
                            && ((y - h.sy) * h.vy).is_sign_positive()
                    }) {
                        ans += 1;
                    }
                }
            }
        }
    }

    ans
}
