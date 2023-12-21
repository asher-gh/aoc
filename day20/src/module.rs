#[derive(Eq, Hash, PartialEq, Debug)]
pub enum Module<'a> {
    Broadcaster,
    FlipFlop(bool),
    Conjunction(Vec<(&'a str, bool)>),
}

impl<'a> Module<'a> {
    pub fn from_str(s: &str) -> Option<(&str, Self)> {
        use Module::*;
        match s {
            "broadcaster" => Some((s, Self::Broadcaster)),
            _ => match &s[..1] {
                "%" => Some((&s[1..], FlipFlop(false))),
                "&" => Some((&s[1..], Conjunction(vec![]))),
                _ => None,
            },
        }
    }

    pub fn process_pulse(
        &mut self,
        pulse: bool,
        src: Option<&'a str>,
    ) -> Option<bool> {
        use Module::*;
        match self {
            Broadcaster => Some(false),
            FlipFlop(signal) if !pulse => {
                *signal = !*signal;
                Some(*signal)
            }
            Conjunction(inputs) => {
                // update the signal corresponding to the input
                // it assumes that all inputs will be initialized
                if let Some(src) = src {
                    if let Some(input) = inputs
                        .into_iter()
                        .find(|(label, stat)| label == &src)
                    {
                        input.1 = pulse;
                    }
                }
                // if all high, send low, else high
                Some(!inputs.iter().fold(true, |a, (_, x)| a && *x))
            }
            _ => None,
        }
    }
}
