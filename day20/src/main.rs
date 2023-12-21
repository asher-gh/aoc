mod module;
use std::collections::{HashMap, VecDeque};

use module::Module;

fn main() {
    let input = if cfg!(debug_assertions) {
        r#"
        broadcaster -> a
        %a -> inv, con
        &inv -> b
        %b -> con
        &con -> output
    "#
    } else {
        include_str!("../input")
    };

    let mut modules: HashMap<&str, Module> = input
        .trim()
        .lines()
        .filter_map(|line| line.trim().split(" -> ").next())
        .map(|input| input.trim())
        .fold(HashMap::new(), |mut modules, x| {
            if let Some((k, v)) = Module::from_str(x) {
                modules.insert(k, v);
            };
            modules
        });

    let config: HashMap<&str, Vec<&str>> = input
        .trim()
        .lines()
        .filter_map(|line| {
            if let [src, dst_list] = line
                .trim()
                .split(" -> ")
                .take(2)
                .collect::<Vec<&str>>()[..]
            {
                return Some((
                    match src {
                        "broadcaster" => src,
                        _ => &src[1..],
                    },
                    dst_list
                        .split(",")
                        .map(|x| x.trim())
                        .collect::<Vec<&str>>(),
                ));
            }
            None
        })
        .collect();

    // initialize conjunction input states
    for (label, module) in modules.iter_mut() {
        if let Module::Conjunction(inputs) = module {
            for (src, dst_list) in config.clone() {
                if dst_list.contains(label) {
                    inputs.push((src, false));
                }
            }
        }
    }

    let mut q: VecDeque<(&str, bool, &str)> = VecDeque::new();
    let mut count = (0, 0);

    let mut press_button = || {
        q.push_back(("broadcaster", false, "button"));

        while !q.is_empty() {
            if let Some((dst, pulse, src)) = q.pop_front() {
                if let Some(x) = config.get(dst) {
                    if let Some(dst_mod) = modules.get_mut(dst) {
                        if let Some(fwd_pulse) =
                            dst_mod.process_pulse(pulse, Some(src))
                        {
                            let dst2_list = config.get(dst).unwrap();
                            for dst2 in dst2_list {
                                q.push_back((dst2, fwd_pulse, dst));
                            }
                        }
                    };
                }

                // println!(
                //     "[{}] {}-{}->{}",
                //     q.len(),
                //     src,
                //     if pulse { "high" } else { "low" },
                //     dst
                // );

                if pulse {
                    count.0 += 1;
                } else {
                    count.1 += 1;
                }
            }
        }
    };

    for _ in 0..1000 {
        press_button();
    }

    println!("{}", (count.0 * count.1))
}
