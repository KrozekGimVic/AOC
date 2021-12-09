use crate::common::Part;
use std::collections::HashMap;
use itertools::Itertools;

struct Entry<'a>  {
    input: Vec<&'a str>,
    output: Vec<&'a str>,
}

impl<'a>  Entry<'a> {
    fn digit_map() -> HashMap<&'static str, char> {
        HashMap::from([
            ("abcefg", '0'),
            ("cf", '1'),
            ("acdeg", '2'),
            ("acdfg", '3'),
            ("bcdf", '4'),
            ("abdfg", '5'),
            ("abdefg", '6'),
            ("acf", '7'),
            ("abcdefg", '8'),
            ("abcdfg", '9'),
        ])
    }

    fn decode(&self) -> String {
        let permutations: Vec<Vec<u8>> = (0..7).permutations(7).collect();

        for p in permutations.iter() {
            if self.input_compatible_with(p) {
                return self.output.iter().map(|&s| Entry::digit_map()[Entry::transform(s, p).as_str()]).collect();
            }
        }
        unreachable!()
    }

    fn input_compatible_with(&self, p: &Vec<u8>) -> bool {
        for digits in self.input.iter() {
            let lights = Entry::transform(digits, p);
            if !Entry::digit_map().contains_key(lights.as_str()) {
                return false;
            }
        }
        return true;
    }

    fn transform(s: &str, p: &Vec<u8>) -> String {
        let mut new : Vec<u8> = s.bytes().map(|c| p[(c-b'a') as usize]+b'a').collect();
        new.sort();
        String::from_utf8(new).unwrap()
    }
}

pub fn solve(data : &Vec<String>, part : Part) {
    let code : Vec<Entry> = data.iter().map(|s| {
        let mut x = s.split(" | ");
        Entry {
            input: x.next().unwrap().split(' ').collect(),
            output: x.next().unwrap().split(' ').collect(),
        }
    }).collect();

    match part {
        Part::First => {
            let unique = [2, 3, 4, 7];
            println!("{}", code.iter().map(|e| {
                e.output.iter().filter(|&w| unique.contains(&w.len())).count()
            }).sum::<usize>());
        }
        Part::Second => {
            let result : i32 = code.iter().map(|c| c.decode().parse::<i32>().unwrap()).sum();
            println!("{}", result);
        }
    }
}
