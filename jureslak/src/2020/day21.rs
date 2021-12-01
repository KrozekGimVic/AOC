use crate::common::Part;
use std::collections::{HashMap, HashSet};
struct MenuItem<'a> {
    ingredients: HashSet<&'a str>,
    allergens: Vec<&'a str>,
}
fn parse_ingredients(s: &String) -> MenuItem {
    let parts : Vec<_> = s.split(" (contains ").collect();
    assert_eq!(parts.len(), 2);
    let ingredients: HashSet<&str> = parts[0].split(' ').collect();
    let allergens : Vec<&str> = parts[1][..parts[1].len()-1].split(", ").collect();
    MenuItem{ ingredients, allergens }
}

// Maps allergen to a list of possible ingredients that contain it.
fn construct_allergen_map<'a>(menu: &'a Vec<MenuItem>) -> HashMap<&'a str, HashSet<&'a str>> {
    let mut allergen_map: HashMap<&str, HashSet<&str>> = HashMap::new();
    for item in menu {
        for &al in item.allergens.iter() {
            match allergen_map.get_mut(al) {
                Some(possible_ingredients) => {
                    for &ing in possible_ingredients.clone().iter() {
                        if !item.ingredients.contains(ing) {
                            possible_ingredients.remove(ing);
                        }
                    }
                },
                None => { allergen_map.insert(al, item.ingredients.clone()); },
            }
        }
    }
    allergen_map
}

fn compute_allergen_sources<'a>(mut allergen_map: HashMap<&'a str, HashSet<&'a str>>) -> HashMap<&str, &str> {
    let mut allergen_source = HashMap::new();
    while !allergen_map.is_empty() {
        let (&k, v) = allergen_map.iter().find(|(_, v)| v.len() == 1).expect("Cannot deduce");
        let ing = *v.iter().next().unwrap();
        allergen_source.insert(k, ing);
        allergen_map.remove(k);
        for opt in allergen_map.values_mut() {
            opt.remove(ing);
        }
    }
    allergen_source
}

pub fn solve(data : &Vec<String>, part : Part) {
    let list : Vec<MenuItem> = data.iter().map(parse_ingredients).collect();
    let allergen_map = construct_allergen_map(&list);
    println!("{:?}", allergen_map);
    match part {
        Part::First => {
            let possible : HashSet<&str>= allergen_map.iter().flat_map(|(_, v)| v.iter().map(|s| *s)).collect();
            let count = list.iter().flat_map(|item| item.ingredients.iter()).filter(|&ing| !possible.contains(*ing)).count();
            println!("{}", count);
        },
        Part::Second => {
            let allergen_sources = compute_allergen_sources(allergen_map);
            let mut ingredients : Vec<(&str, &str)> = allergen_sources.iter().map(|(k, v)| (*k, *v)).collect();
            ingredients.sort_by_key(|(a, _)| *a);
            println!("{}", ingredients.iter().map(|(_, v)| *v).collect::<Vec<_>>().join(","));
        },
    }
}