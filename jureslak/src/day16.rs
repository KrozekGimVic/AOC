use crate::common::Part;
use std::collections::{HashMap, HashSet};
use std::ops::Range;

struct Rules {
    rules: HashMap<String, Vec<Range<i32>>>
}

fn parse_rules(s: &String) -> (String, Vec<Range<i32>>) {
    let parts: Vec<&str> = s.split(": ").collect();
    assert_eq!(parts.len(), 2);
    let ranges: Vec<Range<i32>> = parts[1].split(" or ").map(|s| {
        let parts: Vec<i32> = s.split("-").map(|s| s.parse().unwrap()).collect();
        Range { start: parts[0], end: parts[1]+1 }
    }).collect();
    (parts[0].to_owned(), ranges)
}

fn valid_ranges(r: &Rules) -> Vec<bool> {
    let mut valid = vec![false; 1000];
    for rules in r.rules.values() {
        for span in rules {
            for v in span.clone() {
                valid[v as usize] = true;
            }
        }
    }
    valid
}

fn compute_error_rate(r: &Rules, tickets: &Vec<Vec<i32>>) -> i32 {
    let valid = valid_ranges(r);
    tickets.iter().filter_map(|t| t.iter().find(|&v| !valid[*v as usize])).sum()
}

fn guess_fields(r: &Rules, tickets: &Vec<Vec<i32>>) -> Vec<String> {
    let valid = valid_ranges(r);
    let tickets : Vec<&Vec<i32>> = tickets.iter().filter(|&t| !t.iter().any(|v| !valid[*v as usize])).collect();
    let mut possible = vec![HashSet::new(); tickets[0].len()];
    for i in 0..tickets[0].len() {
        for (field, rules) in r.rules.iter() {
            if tickets.iter().all(|t| rules.iter().any(|r| r.contains(&t[i]))) {
                possible[i].insert(field);
            }
        }
    }

    let mut fields = vec![String::new(); tickets[0].len()];
    loop {
        let determined = possible.iter().enumerate().find(|&(_i, s)| s.len() == 1);
        match determined {
            None => return fields,
            Some((i, field)) => {
                fields[i] = (*field.iter().next().unwrap()).clone();
                possible.iter_mut().for_each(|h| { h.remove(&fields[i]); });
            },
        }
    }
}

pub fn solve(data : &Vec<String>, part : Part) {
    let rules = Rules {
        rules: data.iter().take_while(|&x| !x.is_empty()).map(parse_rules).collect(),
    };
    let n = rules.rules.len();
    let my_ticket : Vec<i32> = data[n+2].split(",").map(|s| s.parse().unwrap()).collect();
    let nearby : Vec<Vec<i32>> = data[n+5..].iter().map(|line| {
        line.split(",").map(|s| s.parse().unwrap()).collect()
    }).collect();

    match part {
        Part::First => println!("{}", compute_error_rate(&rules, &nearby)),

        Part::Second => {
            let fields= guess_fields(&rules, &nearby);
            let mut r = 1i64;
            for (i, c) in fields.iter().enumerate() {
                if c.starts_with("departure") {
                    r *= my_ticket[i] as i64;
                }
            }
            println!("{}", r);
        },
    }
}
