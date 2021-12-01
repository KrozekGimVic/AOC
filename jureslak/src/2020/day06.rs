use crate::common::Part;
use std::collections::HashSet;

pub fn solve(data : &Vec<String>, part : Part) {
    let groups : Vec<Vec<String>> = data.join("\n").split("\n\n").map(|s: &str| {
        s.split_whitespace().map(|s| s.to_owned()).collect()
    }).collect();

    match part {
        Part::First => {
            let questions = groups.iter().map(|s| {
                s.iter().flat_map(|t| t.bytes()).collect()
            });
            println!("{}", questions.map(|h: HashSet<u8>| h.len()).sum::<usize>())
        },
        Part::Second => {
            const ASCII_LOWER : &str = "abcdefghijklmnopqrstuvwxyz";
            let counts : usize = groups.iter().map(|v| {
                ASCII_LOWER.chars().filter(|&b| {
                    v.iter().all(|s| s.contains(b))
                }).count()
            }).sum();
            println!("{}", counts);
        },
    }
}
