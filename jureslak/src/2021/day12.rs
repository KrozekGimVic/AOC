use crate::common::Part;
use std::collections::HashMap;
use std::iter::FromIterator;

fn is_small(s: &str) -> bool {
    s.chars().all(|c| c.is_lowercase())
}

fn join(v: &Vec<String>, s: &str) -> Vec<String> {
    let mut new_v = v.clone();
    new_v.push(s.to_string());
    new_v
}

fn list_all_paths(graph: &HashMap<String, Vec<String>>, node: &str, prefix: Vec<String>) -> Vec<Vec<String>> {
    if node == "end" {
        return vec![prefix];
    }
    let mut paths : Vec<Vec<String>> = Vec::new();
    for n in graph.get(node).unwrap().iter() {
        let is_small = is_small(n);
        if !is_small || is_small && !prefix.contains(n) {
            paths.extend(list_all_paths(graph, n, join(&prefix, n)));
        }
    }
    paths
}

fn list_all_paths2(graph: &HashMap<String, Vec<String>>, node: &str, prefix: Vec<String>, already_repeated: bool) -> Vec<Vec<String>> {
    if node == "end" {
        return vec![prefix];
    }
    let mut paths : Vec<Vec<String>> = Vec::new();
    for n in graph.get(node).unwrap().iter() {
        let is_small = is_small(n);
        if !is_small {
            paths.extend(list_all_paths2(graph, n, join(&prefix, n), already_repeated));
        } else if is_small && !prefix.contains(n) {
            paths.extend(list_all_paths2(graph, n, join(&prefix, n), already_repeated));
        } else if is_small && !already_repeated && prefix.contains(n) && n != "start" && n != "end" {
            paths.extend(list_all_paths2(graph, n, join(&prefix, n), true));
        }
    }
    paths
}

pub fn solve(data : &Vec<String>, part : Part) {
    let mut graph: HashMap<String, Vec<String>> = HashMap::from_iter(data.iter().map(|s| s.split('-')).flatten().map(|s| {
        (s.to_string(), vec![])
    }));
    for s in data.iter() {
        let mut sp = s.split('-');
        let a = sp.next().unwrap();
        let b = sp.next().unwrap();
        graph.get_mut(a).unwrap().push(b.to_string());
        graph.get_mut(b).unwrap().push(a.to_string());
    }


    match part {
        Part::First => {
            let paths = list_all_paths(&graph, "start", vec![String::from("start")]);
            // println!("{:?}", paths);
            println!("{}", paths.len());
        }
        Part::Second => {
            let paths = list_all_paths2(&graph, "start", vec![String::from("start")], false);
            println!("{}", paths.len());
        }
    }
}
