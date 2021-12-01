use crate::common::Part;
use std::collections::HashMap;

struct MemoryGame {
    start: Vec<i32>,
    last_seen: HashMap<i32, usize>,
    iter: usize,
    last_val: i32,
}

impl MemoryGame {
    fn new(start: Vec<i32>) -> MemoryGame {
        assert!(!start.is_empty());
        MemoryGame {
            start,
            last_seen: HashMap::new(),
            iter: 0,
            last_val: -1,
        }
    }
}

impl Iterator for MemoryGame {
    type Item = i32;
    fn next(&mut self) -> Option<Self::Item> {
        let val = if self.iter < self.start.len() {
            self.start[self.iter]
        } else {
            let found = self.last_seen.get(&self.last_val);
            match found {
                None => 0,
                Some(last_idx) => (self.iter - 1 - last_idx) as i32
            }
        };
        if self.last_val != -1 {
            self.last_seen.insert(self.last_val, self.iter-1);
        }
        self.last_val = val;
        self.iter += 1;
        Some(val)
    }
}

pub fn solve(data : &Vec<String>, part : Part) {
    let game = MemoryGame::new(data[0].split(',').map(|s| s.parse().unwrap()).collect());

    match part {
        Part::First => println!("{}", game.take(2020).last().unwrap()),
        Part::Second => println!("{}", game.take(30000000).last().unwrap()),
    }
}
