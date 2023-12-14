use std::collections::HashMap;
use crate::common::Part;

struct Dest<'a> {
    left: &'a str,
    right: &'a str,
}

fn follow<'a>(mut start: &'a str, map: &'a HashMap<&'a str, Dest>, instructions: &[u8], is_end: fn(&str) -> bool) -> usize {
    let mut i = 0;
    while !is_end(start) {
        start = match instructions[i % instructions.len()] {
            b'L' => map[start].left,
            b'R' => map[start].right,
            _ => panic!("Instruction not L or R"),
        };
        i += 1;
    }
    i
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
    let cyc : Vec<usize> = starts.iter().map(|&s| follow(s, map, instructions, |s| s.ends_with('Z'))).collect();
    let g = cyc.clone().into_iter().reduce(gcd).unwrap();
    println!("{:?}", cyc);
    println!("{}", g);
    cyc.iter().product::<usize>() / g
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

            println!("{}", follow("AAA", &nodes, dir, |s| s == "ZZZ"));
        }
        Part::Second => {
            println!("{}", follow2(&nodes, dir));

        }
    }
}
