use crate::common::Part;
use std::collections::HashSet;

#[derive(Debug)]
enum RuleType {
    Char(u8),
    Alt(Vec<Vec<usize>>),
}

#[derive(Debug)]
struct Rule {
    id: usize,
    class: RuleType,
}

fn parse_rule(s: &String) -> Rule {
    let parts : Vec<&str> = s.split(": ").collect();
    assert_eq!(parts.len(), 2);
    let class= if parts[1].starts_with('"') {
        RuleType::Char(parts[1].as_bytes()[1])
    } else {
        RuleType::Alt(parts[1].split(" | ").map(|p|
            p.split(' ').map(|s| s.parse().unwrap()).collect()
        ).collect())
    };
    Rule {
        id: parts[0].parse().unwrap(),
        class,
    }
}


fn build_dict_for_rule(rule: usize, rules: &Vec<Rule>, dict: &mut Vec<HashSet<String>>) {
    if dict[rule].len() > 0 { return; }
    match &rules[rule].class {
        RuleType::Char(c) => { dict[rule].insert(String::from(*c as char)); },
        RuleType::Alt(opts) => {
            for opt in opts {
                // println!("{}, {:?}", rule, opt);
                // concatenate
                let mut possible = vec![String::new()];
                for sub_rule in opt {
                    build_dict_for_rule(*sub_rule, rules, dict);
                    let mut new_possible = Vec::with_capacity(possible.len() * dict[*sub_rule].len());
                    for old in possible.iter() {
                        for word in dict[*sub_rule].iter() {
                            new_possible.push(old.to_owned() + word);
                        }
                    }
                    possible = new_possible;
                }
                for word in possible {
                    dict[rule].insert(word);
                }
            }
        }
    }
}

fn build_dict(rules: &Vec<Rule>) -> Vec<HashSet<String>> {
    let mut dict = vec![HashSet::new(); rules.len()];
    for i in 0..rules.len() {
        build_dict_for_rule(i, rules, &mut dict);
    }
    dict
}

pub fn solve(data : &Vec<String>, part : Part) {
    let mut rules : Vec<Rule> = data.iter().take_while(|&s| !s.is_empty()).map(parse_rule).collect();
    rules.sort_by_key(|r| r.id);
    // println!("{:?}", rules);
    // println!("{}", rules.len());

    // println!("{:?}", dictionary);
    let dictionary = build_dict(&rules);

    match part {
        Part::First => {
            let count = data[rules.len()+1..].iter().filter(|&word| {
                dictionary[0].contains(word)
            }).count();
            println!("{}", count);
        },

        Part::Second => {
            // Add new rules:
            // 8: 42 ...
            // 11: 42 42 42 ... 31 31 31
            // 0: 8 11
            let count = data[rules.len()+1..].iter().filter(|&word| {
                let n = word.len();
                if n % 8 != 0 { return false; }
                let mut n42 = 0;
                for i in 0..n/8 {
                    if dictionary[42].contains(&word[8*i..8*(i+1)]) {
                        n42 += 1;
                    } else {
                        break;
                    }
                }
                let mut n31 = 0;
                for i in 0..n/8 {
                    if dictionary[31].contains(&word[n-8*(i+1)..n-8*i]) {
                        n31 += 1;
                    } else {
                        break;
                    }
                }
                n <= 8*(n42+n31) && n31 < n42 && n31 > 0 && n42 > 0
            }).count();
            println!("{}", count);
        },
    }
}
