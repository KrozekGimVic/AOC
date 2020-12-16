use crate::common::Part;
use std::collections::HashMap;
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

fn compute_error_rate(r: &Rules, tickets: &Vec<Vec<i32>>) -> i32 {
    let mut valid = vec![false; 1000];
    for rules in r.rules.values() {
        for span in rules {
            for v in span.clone() {
                valid[v as usize] = true;
            }
        }
    }
    tickets.iter().filter_map(|t| t.iter().find(|&v| !valid[*v as usize])).sum()
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
        Part::Second => println!("{}", 1),
    }
}
