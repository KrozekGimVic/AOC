use std::collections::{HashMap, VecDeque};
use crate::common::Part;

#[derive(Debug)]
enum Component {
    X, M, A, S,
}

#[derive(Debug)]
enum Operation { Lt, Gt }

#[derive(Debug, Clone)]
enum Outcome {
    Accepted, Rejected, Redirect(String),
}

impl Outcome {
    fn from(s: &str) -> Outcome {
        if s == "A" {
            Outcome::Accepted
        } else if s == "R" {
            Outcome::Rejected
        } else {
            Outcome::Redirect(s.to_string())
        }
    }
}

#[derive(Debug)]
enum Condition {
    Any,
    Check(Component, Operation, i64),
}

impl Condition {
    fn check_from_str(s: &str) -> Condition {
        let v = s.as_bytes();
        let comp = if v[0] == b'x' { Component::X }
        else if v[0] == b'm' { Component::M }
        else if v[0] == b'a' { Component::A }
        else { Component:: S };
        let op = if v[1] == b'<' { Operation::Lt }
        else { Operation::Gt };
        Condition::Check(comp, op, s[2..].parse().unwrap())
    }
}

#[derive(Debug)]
struct Rule {
    condition: Condition,
    outcome: Outcome,
}

#[derive(Debug)]
struct Workflow {
    // name: String,
    rules: Vec<Rule>,
}

#[derive(Debug, Clone)]
struct Rating {
    x: i64, m: i64, a: i64, s: i64,
}
fn apply(part: &Rating, wf: &Workflow) -> Outcome {
    for rule in &wf.rules {
        match &rule.condition {
            Condition::Any => { return rule.outcome.clone() },
            Condition::Check(comp, op, val) => {
                let c = match comp {
                    Component::X => part.x,
                    Component::M => part.m,
                    Component::A => part.a,
                    Component::S => part.s,
                };
                match op {
                    Operation::Lt => { if c < *val { return rule.outcome.clone(); }}
                    Operation::Gt => { if c > *val { return rule.outcome.clone(); }}
                }
            },
        }
    }
    panic!("No rule applies!");
}

fn check_ok(part: &Rating, workflows: &HashMap<&str, Workflow>) -> bool {
    let mut s: String = "in".to_string();
    loop {
        match apply(part, &workflows[&s[..]]) {
            Outcome::Accepted => return true,
            Outcome::Rejected => return false,
            Outcome::Redirect(str) => s = str,
        }
    }
}

#[derive(Debug, Clone)]
struct RatingRange {
    x1: i64, x2: i64, m1: i64, m2: i64, a1: i64, a2: i64, s1: i64, s2: i64,
}

impl RatingRange {
    fn comb(&self) -> i64 {
        assert!(self.x1 <= self.x2);
        assert!(self.m1 <= self.m2);
        assert!(self.a1 <= self.a2);
        assert!(self.s1 <= self.s2);
        (self.x2-self.x1+1)*
        (self.m2-self.m1+1)*
        (self.a2-self.a1+1)*
        (self.s2-self.s1+1)
    }
}

#[derive(Debug)]
struct RangeOutcome {
    range: RatingRange,
    outcome: Outcome,
}

fn apply2(mut part: RatingRange, wf: &Workflow) -> Vec<RangeOutcome> {
    let mut parts = vec![];
    for rule in &wf.rules {
        match &rule.condition {
            Condition::Any => {
                parts.push(RangeOutcome {
                    range: part,
                    outcome: rule.outcome.clone(),
                });
                break;
            },
            Condition::Check(comp, op, val) => {
                let (c1, c2) = match comp {
                    Component::X => (part.x1, part.x2),
                    Component::M => (part.m1, part.m2),
                    Component::A => (part.a1, part.a2),
                    Component::S => (part.s1, part.s2),
                };
                match op {
                    Operation::Lt => {
                        if c2 < *val {
                            parts.push(RangeOutcome {
                                range: part,
                                outcome: rule.outcome.clone(),
                            });
                            break;
                        } else if c1 < *val {
                            match comp {
                                Component::X => part.x2 = *val-1,
                                Component::M => part.m2 = *val-1,
                                Component::A => part.a2 = *val-1,
                                Component::S => part.s2 = *val-1,
                            };
                            parts.push(RangeOutcome {
                                range: part.clone(),
                                outcome: rule.outcome.clone(),
                            });
                            match comp {
                                Component::X => part.x2 = c2,
                                Component::M => part.m2 = c2,
                                Component::A => part.a2 = c2,
                                Component::S => part.s2 = c2,
                            };
                            match comp {
                                Component::X => part.x1 = *val,
                                Component::M => part.m1 = *val,
                                Component::A => part.a1 = *val,
                                Component::S => part.s1 = *val,
                            };
                        } else {
                        }
                    }
                    Operation::Gt => {
                        if c1 > *val {
                            parts.push(RangeOutcome {
                                range: part,
                                outcome: rule.outcome.clone(),
                            });
                            break;
                        } else if c2 > *val {
                            match comp {
                                Component::X => part.x1 = *val+1,
                                Component::M => part.m1 = *val+1,
                                Component::A => part.a1 = *val+1,
                                Component::S => part.s1 = *val+1,
                            };
                            parts.push(RangeOutcome {
                                range: part.clone(),
                                outcome: rule.outcome.clone(),
                            });
                            match comp {
                                Component::X => part.x1 = c1,
                                Component::M => part.m1 = c1,
                                Component::A => part.a1 = c1,
                                Component::S => part.s1 = c1,
                            };
                            match comp {
                                Component::X => part.x2 = *val,
                                Component::M => part.m2 = *val,
                                Component::A => part.a2 = *val,
                                Component::S => part.s2 = *val,
                            };
                        } else {
                        }
                    }
                }
            },
        }
    }
    parts
}

pub fn solve(data : &Vec<String>, part : Part) {
    let mut workflows = HashMap::new();
    let mut iter = data.iter();
    for line in &mut iter {
        if line.is_empty() { break; }
        let mut splitter = line.split('{');
        let name = splitter.next().unwrap();
        let rest = splitter.next().unwrap();
        let rules = rest.split(',').map(|s| {
            if s.contains('}') {
                Rule {
                    condition: Condition::Any,
                    outcome: Outcome::from(&s[..s.len()-1]),
                }
            } else {
                let mut splitter = s.split(':');
                Rule {
                    condition: Condition::check_from_str(splitter.next().unwrap()),
                    outcome: Outcome::from(splitter.next().unwrap()),
                }
            }
        }).collect();
        workflows.insert(name, Workflow { rules });
    }

    let parts : Vec<Rating> = iter.map(|s| {
        let mut splitter = s.split(',');
        let x = splitter.next().unwrap();
        let m = splitter.next().unwrap();
        let a = splitter.next().unwrap();
        let s = splitter.next().unwrap();
        Rating {
            x: x[3..].parse().unwrap(),
            m: m[2..].parse().unwrap(),
            a: a[2..].parse().unwrap(),
            s: s[2..s.len()-1].parse().unwrap(),
        }
    }).collect();


    // println!("w: {:?}", workflows);
    // println!("p: {:?}", parts);

    match part {
        Part::First => {
            let s : i64 = parts.iter().filter(|p| check_ok(p, &workflows)).map(|s| s.x+s.m+s.a+s.s).sum();
            println!("{}", s);
        }
        Part::Second => {
            let mut q = VecDeque::new();
            q.push_back((RatingRange {
                x1: 1, x2: 4000, a1: 1, a2: 4000, m1: 1, m2: 4000, s1: 1, s2: 4000,
            }, "in".to_string()));

            let mut done = vec![];
            while !q.is_empty() {
                let (r, name) = q.pop_front().unwrap();
                println!("{:?} {}", r, name);
                for r in apply2(r, &workflows[&name[..]]) {
                    match r.outcome {
                        Outcome::Accepted => { done.push(r.range); }
                        Outcome::Rejected => { }
                        Outcome::Redirect(s) => { q.push_back((r.range, s.to_string())) }
                    }
                }
            }
            for a in &done {
                println!("{:?}", a);
            }
            let s : i64 = done.iter().map(|r| r.comb()).sum();
            println!("{:?}", s);
        }
    }
}
