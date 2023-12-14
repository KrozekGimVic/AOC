use std::collections::{HashMap, HashSet};
use crate::common::Part;

struct Dest<'a> {
    left: &'a str,
    right: &'a str,
}

fn follow<'a>(mut start: &'a str, map: &'a HashMap<&'a str, Dest>, instructions: &[u8], is_end: fn(&str) -> bool) -> (usize, &'a str) {
    let mut i = 0;
    while !is_end(start) {
        start = match instructions[i % instructions.len()] {
            b'L' => map[start].left,
            b'R' => map[start].right,
            _ => panic!("Instruction not L or R"),
        };
        i += 1;
    }
    (i, start)
}

fn gcd(mut a: usize, mut b: usize) -> usize {
    while b != 0 {
        let t = a % b;
        a = b;
        b = t;
    }
    a
}

fn follow2(map: &HashMap<&str, Dest>, instructions: &[u8]) -> usize {
    let starts : Vec<&str> = map.keys().filter(|&s| s.ends_with('A')).map(|&s| s).collect();
    let cyc : Vec<usize> = starts.iter().map(|&s| {
        let mut step_counts = vec![];
        let mut seen = HashSet::new();
        seen.insert(s);
        let mut start = s;
        loop {
            let (steps, end) = follow(start, map, instructions, |s| s.ends_with('Z'));
            if seen.contains(end) {
                break;
            } else {
                seen.insert(end);
            }
            step_counts.push(steps);
            start = end;
        }
        assert_eq!(step_counts.len(), 1);
        step_counts[0]
    }).collect();

    let g = cyc.clone().into_iter().reduce(gcd).unwrap();
    println!("{}", g);
    println!("{:?}", cyc);
        cyc.into_iter().reduce(|a, b| a*b/g).unwrap()
}


pub fn solve(data : &Vec<String>, part : Part) {
    let dir = data[0].as_bytes();
    let mut nodes = HashMap::new();
    for line in &data[2..] {
        let mut splitter = line.split(" = ");
        let name = splitter.next().unwrap();
        let lr = splitter.next().unwrap();
        let mut splitter = lr[1..lr.len()-1].split(", ");
        let left = splitter.next().unwrap();
        let right = splitter.next().unwrap();
        nodes.insert(name, Dest{left, right});
    }

    match part {
        Part::First => {

            println!("{}", follow("AAA", &nodes, dir, |s| s == "ZZZ").0);
        }
        Part::Second => {
            println!("{}", follow2(&nodes, dir));

        }
    }
}
