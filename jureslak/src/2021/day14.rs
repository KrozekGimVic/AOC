use crate::common::Part;
use std::collections::HashMap;
use itertools::Itertools;
use std::hash::Hash;

fn apply(template: String, rules: &HashMap<String, u8>) -> String {
    if template.is_empty() { return template; }
    let bytes = template.as_bytes();
    let mut new_bytes = Vec::with_capacity(bytes.len());
    new_bytes.push(bytes[0]);
    for i in 1..bytes.len() {
        if let Some(&c) = rules.get(&template[i-1..i+1]) {
            new_bytes.push(c);
        }
        new_bytes.push(bytes[i]);
    }
    String::from_utf8(new_bytes).unwrap()
}

fn hist<T: IntoIterator>(s: T) -> HashMap<T::Item, usize>
    where <T as IntoIterator>::Item: Hash, <T as IntoIterator>::Item: Eq
{
    let mut h = HashMap::new();
    for c in s {
        *h.entry(c).or_insert(0) += 1;
    }
    h
}

fn apply_dumb(mut s: String, rules: &HashMap<String, u8>, n: i32) -> HashMap<u8, usize> {
    for _i in 0..n {
        s = apply(s, &rules);
    }
    hist(s.as_bytes().into_iter().map(|&c| c))
}


// Return how many of which letter we have after calling apply repeatedly `depth` times.
fn apply_fast(s: String, rules: &HashMap<String, u8>, depth: i32,
              memory: &mut HashMap<(String, i32), HashMap<u8, usize>>) -> HashMap<u8, usize> {
    if let Some(e) = memory.get(&(s.clone(), depth)) {
        return e.clone();
    }

    let mut h = HashMap::new();
    if s.is_empty() { return h; }
    if depth < 1 {
        h = apply_dumb(s.clone(), rules, depth);
    } else {
        let bytes = s.as_bytes();
        h.insert(bytes[0], 1);
        for i in 1..bytes.len() {
            if let Some(&c) = rules.get(&s[i-1..i+1]) {
                let sub_h = apply_fast(String::from_utf8(vec![bytes[i-1], c, bytes[i]]).unwrap(), &rules, depth -1, memory);
                for (k, v) in sub_h.into_iter() {
                    *h.entry(k).or_insert(0) += v;
                }
            }
        }
        for b in &bytes[0..bytes.len()-1] {
            *h.get_mut(b).unwrap() -= 1;
        }
    }
    memory.insert((s, depth), h.clone());
    h
}

pub fn solve(data : &Vec<String>, part : Part) {
    let start = data[0].clone();

    let rules : HashMap<String, u8> = data[2..].iter().map(|s| {
        let (s, c) = s.split(" -> ").collect_tuple().unwrap();
        (s.to_string(), c.as_bytes()[0])
    }).collect();

    match part {
        Part::First => {
            let h = apply_dumb(start.clone(), &rules, 10);
            let max = h.values().max().unwrap();
            let min = h.values().min().unwrap();
            println!("{}", max-min);
        }
        Part::Second => {
            let mut memory = HashMap::new();
            let h = apply_fast(start, &rules, 40, &mut memory);
            let max = h.values().max().unwrap();
            let min = h.values().min().unwrap();
            println!("{}", max-min);
        }
    }
}
