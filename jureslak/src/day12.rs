use std::collections::HashMap;
use crate::common::Part;

struct Case {
    s: Vec<u8>,
    g: Vec<i32>,
}

fn case_dot(r: i32, si: usize, gi: usize, s: &[u8], g: &[i32], memo: &mut HashMap<(i32, usize, usize), usize>) -> usize {
    if r > 0 {
        if gi < g.len() && g[gi] == r {
            opts_impl(0, si+1, gi+1, s, g, memo)
        } else {
            0
        }
    } else {
        opts_impl(0, si+1, gi, s, g, memo)
    }
}

fn case_hash(r: i32, si: usize, gi: usize, s: &[u8], g: &[i32], memo: &mut HashMap<(i32, usize, usize), usize>) -> usize {
    if gi < g.len() && r+1 <= g[gi] {
        opts_impl(r+1, si+1, gi, s, g, memo)
    } else {
        0
    }
}

fn opts_impl(r: i32, si: usize, gi: usize, s: &[u8], g: &[i32], memo: &mut HashMap<(i32, usize, usize), usize>) -> usize {
    // println!("r: {} s: {:?} g: {:?}", r, String::from_utf8(s[si..].to_vec()).unwrap(), &g[gi..]);
    if si == s.len() {
        return if r == 0 {
            if gi == g.len() { 1 } else { 0 }
        } else {
            if gi == g.len() - 1 && r == g[gi] { 1 } else { 0 }
        }
    }
    if let Some(res) = memo.get(&(r, si, gi)) {
        return *res;
    }
    let res = match s[si] {
        b'.' => case_dot(r, si, gi, s, g, memo),
        b'#' => case_hash(r, si, gi, s, g, memo),
        b'?' => case_hash(r, si, gi, s, g, memo) + case_dot(r, si, gi, s, g, memo),
        _ => panic!("Unknown char {}", s[0])
    };
    memo.insert((r, si, gi), res);
    res
}

fn opts(c: &Case) -> usize {
    // println!("---------");
    let mut memo = HashMap::new();
    opts_impl(0, 0, 0, &c.s, &c.g, &mut memo)
}

pub fn solve(data : &Vec<String>, part : Part) {
    let mut cases = vec![];
    cases.reserve(data.len());
    for line in data {
        let mut splitter = line.split_whitespace();
        let s = splitter.next().unwrap();
        let g = splitter.next().unwrap();
        let gs = g.split(',').map(|c| c.parse().unwrap()).collect();
        cases.push(Case {
            s: s.as_bytes().to_vec(),
            g: gs,
        })
    }
    match part {
        Part::First => {
            let s : usize = cases.iter().map(|c| opts(c)).sum();
            println!("{}", s);
        }
        Part::Second => {
            let s : usize = cases.iter().map(|c| {
                let mut g = c.g.to_vec();
                g.extend(c.g.iter());
                g.extend(c.g.iter());
                g.extend(c.g.iter());
                g.extend(c.g.iter());

                let mut s = c.s.to_vec();
                s.push(b'?');
                s.extend(c.s.iter());
                s.push(b'?');
                s.extend(c.s.iter());
                s.push(b'?');
                s.extend(c.s.iter());
                s.push(b'?');
                s.extend(c.s.iter());
                opts(&Case {s, g})
            }).sum();
            println!("{}", s);

        }
    }
}
