use std::collections::HashMap;

fn main() {
    let input: Vec<&str> = if cfg!(debug_assertions) {
        include_str!("../test.txt")
    } else {
        include_str!("../input")
    }
    .split("\n\n")
    .collect();

    let mut workflows = HashMap::<&str, Vec<&str>>::new();

    for line in input[0].lines() {
        let mut s = line.trim().split("{");
        if let (Some(k), Some(v)) = (s.next(), s.next()) {
            workflows.insert(
                k,
                v.trim_end_matches('}').split(",").collect(),
            );
        }
    }

    let parts: Vec<HashMap<&str, i32>> = input[1]
        .lines()
        .filter_map(|line| {
            let mut m = HashMap::new();
            line.trim_matches(|c| matches!(c, '{' | '}'))
                .split(",")
                .for_each(|p| {
                    let mut kv = p.split("=");
                    if let (Some(k), Some(v)) = (kv.next(), kv.next())
                    {
                        m.insert(k, v.parse().unwrap());
                    }
                });

            Some(m)
        })
        .collect();

    println!("Part One: {}", p1(&workflows, &parts));
    println!("Part One: {}", p2(&workflows, &parts));

    let x = count(
        HashMap::from([
            ("x", [1, 4000]),
            ("m", [1, 4000]),
            ("a", [1, 4000]),
            ("s", [1, 4000]),
        ]),
        "in",
        &workflows,
    );
}

fn p1(
    workflows: &HashMap<&str, Vec<&str>>,
    parts: &Vec<HashMap<&str, i32>>,
) -> i32 {
    let mut acc = Vec::new();
    for p in parts {
        let mut wf = "in";
        let mut c = workflows.get(wf).unwrap().iter();

        while !matches!(wf, "R" | "A") {
            if let Some(w) = parse_condition(c.next().unwrap(), &p) {
                wf = w;
                if !matches!(wf, "R" | "A") {
                    c = workflows.get(wf).unwrap().iter();
                }
            }
        }

        if matches!(wf, "A") {
            acc.push(p);
        }
    }

    acc.iter().fold(0, |a, p| a + p.values().sum::<i32>())
}

fn p2(
    workflows: &HashMap<&str, Vec<&str>>,
    parts: &Vec<HashMap<&str, i32>>,
) -> usize {
    let ranges = HashMap::from([
        ("x", [1, 4000]),
        ("m", [1, 4000]),
        ("a", [1, 4000]),
        ("s", [1, 4000]),
    ]);

    count(ranges, "in", &workflows)
}

fn count<'a>(
    mut ranges: HashMap<&'a str, [usize; 2]>,
    name: &str,
    workflows: &HashMap<&str, Vec<&'a str>>,
) -> usize {
    match name {
        "R" => 0,
        "A" => ranges
            .values()
            .into_iter()
            .fold(1, |a, x| (a * (x[1].saturating_sub(x[0]) + 1))),
        _ => {
            let mut total = 0;

            if let Some(rules) = workflows.get(name) {
                for &rule in rules {
                    if let [cond, target] =
                        rule.split(":").take(2).collect::<Vec<&str>>()
                            [..]
                    {
                        let key = &cond[..1];
                        let cmp = &cond[1..2];
                        let n: usize = cond[2..].parse().unwrap();

                        let &[lo, hi] = ranges.get(key).unwrap();

                        let (t, f) = match cmp {
                            "<" => (
                                (lo, (n - 1).min(hi)),
                                (n.max(lo), hi),
                            ),
                            _ => (
                                ((n + 1).max(lo), hi),
                                (lo, n.min(hi)),
                            ),
                        };

                        if t.0 <= t.1 {
                            let mut copy = ranges.clone();
                            copy.entry(key)
                                .and_modify(|e| *e = [t.0, t.1]);
                            total += count(copy, target, &workflows);
                        }

                        if f.0 <= f.1 {
                            ranges
                                .entry(key)
                                .and_modify(|e| *e = [f.0, f.1]);
                        } else {
                            break;
                        }
                    } else {
                        total +=
                            count(ranges.clone(), rule, &workflows);
                    }
                }
            }

            total
        }
    }
}

fn parse_condition<'a>(
    rule: &'a str,
    parts: &HashMap<&'a str, i32>,
) -> Option<&'a str> {
    let mut out = None;
    // a<2006:qkq
    // R,A,rfq
    if rule.contains(":") {
        let mut s = rule.split(":");
        if let (Some(condition), Some(matched)) = (s.next(), s.next())
        {
            let cmp = if condition.contains(">") { ">" } else { "<" };

            if let [k, v] = condition
                .split(cmp)
                .take(2)
                .collect::<Vec<&str>>()[..]
            {
                if let (Some(a), Ok(b)) =
                    (parts.get(k), v.parse::<i32>())
                {
                    if match cmp {
                        ">" => *a > b,
                        "<" => *a < b,
                        _ => false,
                    } {
                        out = Some(matched);
                    }
                }
            }
        }
    } else {
        out = Some(rule)
    }
    out
}
