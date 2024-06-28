// Template.
// Copy to daynum.rs, and uncomment relevant lins in main to add

use std::collections::{HashMap, VecDeque};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Pulse {
    Low,
    High,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Module {
    Broadcast(Vec<String>),
    FlipFlop(bool, Vec<String>),
    Conjunction(HashMap<String, Pulse>, Vec<String>),
}

impl Module {
    fn parse(s: &str) -> (&str, Self) {
        let (name, dests) = s.split_once(" -> ").unwrap();
        let dest_names = dests.split(", ").map(|s| s.to_string()).collect::<Vec<_>>();
        if name == "broadcaster" {
            (name, Self::Broadcast(dest_names))
        } else if name.starts_with('%') {
            let mname = name.trim_start_matches('%');
            (mname, Self::FlipFlop(false, dest_names))
        } else {
            let mname = name.trim_start_matches('&');
            (mname, Self::Conjunction(HashMap::new(), dest_names))
        }
    }

    fn outputs(&self) -> &[String] {
        match self {
            Self::Broadcast(d) => d,
            Self::Conjunction(_, d) => d,
            Self::FlipFlop(_, d) => d,
        }
    }

    fn recv(&mut self, input: Pulse, sent_from: &str) -> Option<Pulse> {
        match self {
            Self::Broadcast(_) => Some(input),
            Self::FlipFlop(state, _) => {
                if input == Pulse::Low {
                    let old_state = *state;
                    if old_state {
                        *state = false;
                        Some(Pulse::Low)
                    } else {
                        *state = true;
                        Some(Pulse::High)
                    }
                } else {
                    None
                }
            }
            Self::Conjunction(memory, _) => {
                let last = memory.get_mut(sent_from).unwrap();
                *last = input;
                if memory.values().all(|v| *v == Pulse::High) {
                    Some(Pulse::Low)
                } else {
                    Some(Pulse::High)
                }
            }
        }
    }

    #[allow(dead_code)]
    fn is_reset(&self) -> bool {
        match self {
            Module::Broadcast(_) => true,
            Module::FlipFlop(s, _) => *s,
            Module::Conjunction(s, _) => s.values().all(|&v| v == Pulse::Low),
        }
    }
}
type ModulesState<'a> = HashMap<&'a str, Module>;

#[derive(Debug, Clone, PartialEq, Eq)]
struct PulseCounts {
    high: u64,
    low: u64,
}

// Simulate a button press, and return the number of
fn run_single_loop<'a>(state: &'a mut ModulesState, debug: bool) -> PulseCounts {
    let mut low = 0;
    let mut high = 0;
    let mut queue = VecDeque::new();

    queue.push_back(("".to_string(), "broadcaster".to_string(), Pulse::Low));
    while let Some((sender, target, pulse)) = queue.pop_front() {
        if pulse == Pulse::High {
            high += 1
        } else {
            low += 1
        }
        if let Some(module) = state.get_mut(target.as_str()) {
            let output = module.recv(pulse, &sender);
            if let Some(new_pulse) = output {
                for d in module.outputs() {
                    if debug {
                        println!("Adding output: {target} {:?} -> {d}", new_pulse);
                    }
                    queue.push_back((target.clone(), d.to_string(), new_pulse));
                }
            }
        }
    }

    PulseCounts { high, low }
}

pub fn run(input_path: String) {
    let input = std::fs::read_to_string(input_path).unwrap();
    let mut input_map = HashMap::new();
    let mut modules = input
        .lines()
        .map(|s| {
            let (name, module) = Module::parse(s);
            for d in module.outputs() {
                input_map.entry(d.to_string()).or_insert(vec![]).push(name)
            }
            (name, module)
        })
        .collect::<HashMap<_, _>>();

    // Initialize all the conjunction modules.
    for (dest, inputs) in &input_map {
        if let Some(module) = modules.get_mut(dest.as_str()) {
            if let Module::Conjunction(state, _) = module {
                for i in inputs {
                    state.insert(i.to_string(), Pulse::Low);
                }
            }
        }
    }

    let start_state = modules.clone();
    let mut loops = 0;
    let mut answer = PulseCounts { high: 0, low: 0 };

    while loops < 1000 {
        let new = run_single_loop(&mut modules, false);
        answer.high += new.high;
        answer.low += new.low;
        loops += 1;
    }

    println!("Part 1: {}", answer.high * answer.low);
    modules = start_state;

    let rx_inputs = input_map.get("rx").unwrap();
    assert_eq!(rx_inputs.len(), 1);
    let rx_input = rx_inputs[0];
    let num_inputs = input_map.get(rx_input).unwrap().len();

    let mut counters = HashMap::with_capacity(num_inputs);
    let mut loops = 0;

    while counters.len() < num_inputs {
        loops += 1;
        if loops % 1000000 == 0 {
            dbg!(loops);
        }
        let _ = run_single_loop(&mut modules, false);
        if let Some(Module::Conjunction(in_state, _)) = modules.get(rx_input) {
            if loops % 1000000 == 0 {
                dbg!(&in_state);
            }
            for (name, last_input) in in_state {
                // This module sends to the aggregator that sends to rx.
                //   name -> rx_input -> rx
                // To trigger low to rx, we need to send High from all  modules to rx_input.
                if last_input == &Pulse::High {
                    println!("{loops}: High pulse received from {name}");
                    let this_count = counters.entry(name.to_string()).or_insert(0);
                    *this_count = loops - *this_count;
                }
            }
        }
    }
    let part2 = counters
        .values()
        .fold(1, |acc, val| num::integer::lcm(acc, *val));
    println!("Part 2: {}", part2);
}
