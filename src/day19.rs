// Part 1 is basically a parsing test.
// Part 2 more interesting.  Brute force clearly won't work (4000^4 is way too many options).
// However, we can analyze the rule flows, and work out where the actual important
// boundaries are.  We can then use that to track the total number of acceptable parts.
use std::{
    collections::{HashMap, VecDeque},
    str::FromStr,
};

#[derive(Debug, Clone, PartialEq, Eq)]
enum RuleTarget {
    Accept,
    Reject,
    Jump(String),
}

impl FromStr for RuleTarget {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(RuleTarget::Accept),
            "R" => Ok(RuleTarget::Reject),
            _ => Ok(RuleTarget::Jump(s.to_string())),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Cond {
    Less,
    Greater,
    None,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Part {
    x: u64,
    m: u64,
    a: u64,
    s: u64,
}

impl Part {
    fn with_attr_value(val: u64) -> Self {
        Part {
            x: val,
            a: val,
            m: val,
            s: val,
        }
    }

    fn get_value(&self) -> u64 {
        self.x + self.m + self.a + self.s
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Rule {
    cond: Cond,
    cond_var: String,
    limit: u64,
    if_true: RuleTarget,
}

fn is_accepted(part: &Part, rules: &HashMap<&str, Vec<Rule>>) -> bool {
    let mut workflow = rules.get("in");
    while let Some(w) = workflow {
        for rule in w {
            let is_true = match rule.cond {
                Cond::None => true,
                _ => {
                    let attr_check = match rule.cond_var.as_str() {
                        "x" => part.x,
                        "m" => part.m,
                        "a" => part.a,
                        "s" => part.s,
                        _ => panic!("bad attr"),
                    };
                    if rule.cond == Cond::Less {
                        attr_check < rule.limit
                    } else {
                        attr_check > rule.limit
                    }
                }
            };

            if is_true {
                match &rule.if_true {
                    RuleTarget::Accept => return true,
                    RuleTarget::Reject => return false,
                    RuleTarget::Jump(t) => {
                        workflow = rules.get(t.as_str());
                        break;
                    }
                }
            }
        }
    }
    unreachable!("No accept or reject rule found")
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct RuleState {
    minvals: Part,
    maxvals: Part,
}

fn count_acceptable_parts(rules: &HashMap<&str, Vec<Rule>>, max_attr: u64) -> u64 {
    let mut workflow = rules.get("in").unwrap().iter();
    let first_rule = workflow.next().unwrap();
    // Queue of rules to process.
    let mut stack = VecDeque::new();
    stack.push_back((
        first_rule,
        RuleState {
            minvals: Part::with_attr_value(1),
            maxvals: Part::with_attr_value(max_attr),
        },
    ));
    let mut final_states: Vec<RuleState> = vec![];

    while let Some((rule, state)) = stack.pop_front() {
        let mut true_state = state.clone();
        let mut false_state = state.clone();
        match rule.cond {
            Cond::None => (),
            Cond::Less => {
                match rule.cond_var.as_str() {
                    // The "if-true" maximum is the smaller of the current maximum, and one above the rule limit, down to the minimum.
                    // The "if-false" minimum is the larger of the current minimum, and the rule limit, up to the maximum.
                    "x" => {
                        true_state.maxvals.x = state.maxvals.x.min(rule.limit - 1).max(state.minvals.x);
                        false_state.minvals.x = state.minvals.x.max(rule.limit).min(state.maxvals.x)
                    }
                    "m" => {
                        true_state.maxvals.m = state.maxvals.m.min(rule.limit - 1).max(state.minvals.m);
                        false_state.minvals.m = state.minvals.m.max(rule.limit).min(state.maxvals.m)
                    }
                    "a" => {
                        true_state.maxvals.a = state.maxvals.a.min(rule.limit - 1).max(state.minvals.a);
                        false_state.minvals.a = state.minvals.a.max(rule.limit).min(state.maxvals.a)
                    }
                    "s" => {
                        true_state.maxvals.s = state.maxvals.s.min(rule.limit - 1).max(state.minvals.s);
                        false_state.minvals.s = state.minvals.s.max(rule.limit).min(state.maxvals.s)
                    }
                    _ => panic!("bad attr"),
                }
            }
            Cond::Greater => {
                match rule.cond_var.as_str() {
                    // The "if-true" minimum is the larger of the current minimum, and the rule limit, up to the maximum.
                    // The "if-false" maximum is the smaller of the current maximum, and one above the rule limit, down to the minimum.
                    "x" => {
                        true_state.minvals.x =
                            state.minvals.x.max(rule.limit + 1).min(state.maxvals.x);
                        false_state.maxvals.x = state.maxvals.x.min(rule.limit).max(state.minvals.x)
                    }
                    "m" => {
                        true_state.minvals.m =
                            state.minvals.m.max(rule.limit + 1).min(state.maxvals.m);
                        false_state.maxvals.m =
                            state.maxvals.m.min(rule.limit).max(state.minvals.m);
                    }
                    "a" => {
                        true_state.minvals.a =
                            state.minvals.a.max(rule.limit + 1).min(state.maxvals.a);
                        false_state.maxvals.a =
                            state.maxvals.a.min(rule.limit).max(state.minvals.a);
                    }
                    "s" => {
                        true_state.minvals.s =
                            state.minvals.s.max(rule.limit + 1).min(state.maxvals.s);
                        false_state.maxvals.s =
                            state.maxvals.s.min(rule.limit).max(state.minvals.s);
                    }
                    _ => panic!("bad attr"),
                };
            }
        };

        // If there's another rule after this one, add it to the stack as the false option
        if let Some(new_rule) = workflow.next() {
            stack.push_back((new_rule, false_state));
        }

        // Add the count or next steps for true state
        match &rule.if_true {
            RuleTarget::Accept => {
                // Valid range.  We should add this to the final states.
                // But before we do, remove any overlaps.
                for other in &final_states {
                    if true_state.minvals.x <= other.maxvals.x
                        && true_state.maxvals.x >= other.minvals.x
                    {
                        if true_state.minvals.x < other.minvals.x {
                            true_state.maxvals.x = other.minvals.x;
                        } else {
                            true_state.minvals.x = other.maxvals.x;
                        }
                    }
                    if true_state.minvals.a <= other.maxvals.a
                        && true_state.maxvals.a >= other.minvals.a
                    {
                        if true_state.minvals.a < other.minvals.a {
                            true_state.maxvals.a = other.minvals.a;
                        } else {
                            true_state.minvals.a = other.maxvals.a;
                        }
                    }
                    if true_state.minvals.s <= other.maxvals.s
                        && true_state.maxvals.s >= other.minvals.s
                    {
                        if true_state.minvals.s < other.minvals.s {
                            true_state.maxvals.s = other.minvals.s;
                        } else {
                            true_state.minvals.s = other.maxvals.s;
                        }
                    }
                    if true_state.minvals.m <= other.maxvals.m
                        && true_state.maxvals.m >= other.minvals.m
                    {
                        if true_state.minvals.m < other.minvals.m {
                            true_state.maxvals.m = other.minvals.m;
                        } else {
                            true_state.minvals.m = other.maxvals.m;
                        };
                    }
                }
                final_states.push(true_state);
            }
            RuleTarget::Reject => (),
            RuleTarget::Jump(t) => {
                workflow = rules.get(t.as_str()).unwrap().iter();
                let new_rule = workflow.next().unwrap();
                stack.push_back((new_rule, true_state))
            }
        }
    }

    // Now merge the ranges and count them.
    dbg!(&final_states);
    let (x, a, s, m) = final_states
        .iter()
        .fold((0, 0, 0, 0), |(x, a, s, m), state|{
            ((x + 1 + state.maxvals.x - state.minvals.x),
            (a + 1 + state.maxvals.a - state.minvals.a),
            (s + 1 + state.maxvals.s - state.minvals.s),
            (m + 1 + state.maxvals.m - state.minvals.m),
        )});
    x * a * s* m
}

pub fn run(input_path: String) {
    let input = std::fs::read_to_string(input_path).unwrap();
    let (workflows, parts) = input.split_once("\n\n").unwrap();

    let rules: HashMap<&str, Vec<Rule>> = workflows
        .lines()
        .map(|line| {
            let (name, rest) = line.split_once('{').unwrap();
            let rule_parts = rest.trim_end_matches('}').split(',');
            let rules = rule_parts
                .map(|rule| {
                    if let Some((cond, target)) = rule.split_once(':') {
                        if let Some((cond_var, limit)) = cond.split_once('<') {
                            Rule {
                                cond_var: cond_var.to_string(),
                                cond: Cond::Less,
                                limit: limit.parse().unwrap(),
                                if_true: target.parse().unwrap(),
                            }
                        } else if let Some((cond_var, limit)) = cond.split_once('>') {
                            Rule {
                                cond_var: cond_var.to_string(),
                                cond: Cond::Greater,
                                limit: limit.parse().unwrap(),
                                if_true: target.parse().unwrap(),
                            }
                        } else {
                            panic!("Invalid rule")
                        }
                    } else {
                        Rule {
                            cond: Cond::None,
                            cond_var: String::new(),
                            limit: 0,
                            if_true: rule.parse().unwrap(),
                        }
                    }
                })
                .collect();

            (name, rules)
        })
        .collect();

    let parts = parts.lines().map(|line| {
        let attrs = line
            .trim_end_matches('}')
            .trim_start_matches('{')
            .split(',');
        let mut part = Part {
            x: 0,
            m: 0,
            a: 0,
            s: 0,
        };
        for attr in attrs {
            let (k, v) = attr.split_once('=').unwrap();
            match k {
                "x" => part.x = v.parse().unwrap(),
                "m" => part.m = v.parse().unwrap(),
                "a" => part.a = v.parse().unwrap(),
                "s" => part.s = v.parse().unwrap(),
                _ => panic!("Invalid attribute"),
            }
        }
        part
    });

    let part1: u64 = parts
        .map(|part| {
            if is_accepted(&part, &rules) {
                part.get_value()
            } else {
                0
            }
        })
        .sum();
    println!("Part 1: {}", part1);

    let part2 = count_acceptable_parts(&rules, 4000);
    println!("Part 2: {}", part2);
}
