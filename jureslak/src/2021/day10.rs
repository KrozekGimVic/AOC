use crate::common::Part;
use crate::day10::ParseResult::{Corrupt, Incomplete};

fn find_corrupt(data : &Vec<String>) -> Vec<char> {
    let mut corrupt = Vec::new();
    for line in data.iter() {
        if let Corrupt(c) = parse(line) {
            corrupt.push(c);
        }
    }
    corrupt
}


fn score_incomplete(data : &Vec<String>) -> i64 {
    let mut scores = Vec::new();
    for line in data.iter() {
        if let Incomplete(left) = parse(line) {
            scores.push(score2(&left));
        }
    }
    scores.sort();
    scores[scores.len()/2]
}

enum ParseResult {
    Corrupt(char),
    Incomplete(Vec<char>),
    Ok(),
}

fn parse(line: &String) -> ParseResult {
    let mut stack = Vec::new();
    for c in line.chars() {
        match c {
            '(' | '['| '<' | '{' => stack.push(c),
            ')' => if !stack.ends_with(&['(']) {
                return Corrupt(c);
            } else {
                stack.pop();
            },
            ']' => if !stack.ends_with(&['[']) {
                return Corrupt(c);
            } else {
                stack.pop();
            },
            '}' => if !stack.ends_with(&['{']) {
                return Corrupt(c);
            } else {
                stack.pop();
            },
            '>' => if !stack.ends_with(&['<']) {
                return Corrupt(c);
            } else {
                stack.pop();
            },
            _ => panic!("Wrong input")
        }
    }
    if stack.is_empty() {
        ParseResult::Ok()
    } else {
        ParseResult::Incomplete(stack)
    }
}

fn score(chars: &Vec<char>) -> i64 {
    chars.iter().map(|c|
        match c {
            ')' => 3,
            ']' => 57,
            '}' => 1197,
            '>' => 25137,
            _ => 0,
        }).sum()
}


fn score2(chars: &Vec<char>) -> i64 {
    let mut s = 0;
    for c in chars.iter().rev() {
        s *= 5;
        s += match c {
            '(' => 1,
            '[' => 2,
            '{' => 3,
            '<' => 4,
            _ => 0,
        }
    }
    s
}

pub fn solve(data : &Vec<String>, part : Part) {

    match part {
        Part::First => {
            println!("{}", score(&find_corrupt(data)));
        }
        Part::Second => {
            println!("{}", score_incomplete(data));
        }
    }
}
