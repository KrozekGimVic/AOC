use crate::common::Part;
use std::collections::{HashMap, HashSet, VecDeque};

fn parse_line(s: &String) -> (String, Vec<(String, i32)>) {
    let parts: Vec<&str> = s.split(" bags contain ").collect();
    assert_eq!(parts.len(), 2);
    let mut children : Vec<(String, i32)> = vec![];
    let mut iter = parts[1].split(' ');
    while let Some(num) = iter.next() {
        if num.starts_with("no") { break; }
        let val : i32 = num.parse().expect("Expected an integer.");
        let c1 = iter.next().expect("Expected first part of the color");
        let c2 = iter.next().expect("Expected second part of the color");
        let color : String = c1.to_owned() + " " + c2;
        let bag = iter.next().expect("Expected 'bag'");
        assert!(bag.starts_with("bag"));
        children.push((color, val));
    }

    (parts[0].to_owned(), children)
}

fn count_reachable(graph: &HashMap<&str, Vec<&str>>) -> i32 {
    let mut count : i32 = 0;
    let mut visited = HashSet::new();
    let mut q = VecDeque::new();
    q.push_back("shiny gold");
    visited.insert("shiny gold");
    while !q.is_empty() {
        let c = q.pop_back().unwrap();
        println!("{}", c);
        count += 1;
        for &n in graph.get(c).unwrap_or(&vec![]).iter() {
            if !visited.contains(n) {
                visited.insert(n);
                q.push_back(n);
            }
        }
    }
    count
}

fn compute_num_bags<'a>(v: &'a str, graph: &'a HashMap<String, Vec<(String, i32)>>, num_bags: &mut HashMap<&'a str, i32>) -> i32 {
    if num_bags.contains_key(v) { return num_bags[v] }
    let num = 1 + graph[v].iter().map(|(s, c)| {
        compute_num_bags(s, graph, num_bags) * c
    }).sum::<i32>();
    num_bags.insert(v, num);
    num
}

fn number_of_bags_to_buy(graph: &HashMap<String, Vec<(String, i32)>>) -> i32 {
    let mut num_bags = HashMap::new();
    compute_num_bags("shiny gold", graph, &mut num_bags)-1
}

pub fn solve(data : &Vec<String>, part : Part) {
    let graph: HashMap<String, Vec<(String, i32)>> = data.iter().map(parse_line).collect();
    let mut parents: HashMap<&str, Vec<&str>> = HashMap::new();
    for (k, v) in graph.iter() {
        for (c, _) in v {
            parents.entry(c).or_insert(vec![]).push(k);
        }
    }
    let parents = parents;

    match part {
        Part::First => println!("{}", count_reachable(&parents)-1),
        Part::Second => println!("{}", number_of_bags_to_buy(&graph)),
    }
}
