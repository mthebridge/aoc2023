// Part 1 is basically a parsing test.
// Part 2 more interesting.  Brute force clearly won't work (4000^4 is way too many options).
// However, we can analyze the rule flows, and work out where the actual important
// boundaries are.  We can then use that to track the total number of acceptable parts.
use std::{collections::HashMap, str::FromStr};

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

    let part2 = 0;
    println!("Part 2: {}", part2);
}
