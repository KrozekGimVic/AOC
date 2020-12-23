use crate::common::Part;
use std::fmt::Display;
use std::fmt;

struct Game {
    next: Vec<usize>,
    current: usize,
    max: usize,
}

impl Game {
    fn make_move(&mut self) {
        let first = self.next[self.current];
        let second = self.next[first];
        let third = self.next[second];
        let mut dest= self.current;
        loop {
            dest = if dest == 0 { self.max - 1 } else { dest - 1 };
            if dest != first && dest != second && dest != third {
                break;
            }
        }
        // println!("p: {}, {}, {}, dest: {}", first+1, second+1, third+1, dest+1);
        self.next[self.current] = self.next[third];
        self.next[third] = self.next[dest];
        self.next[dest] = first;
        self.current = self.next[self.current];
    }

    fn get_order(&self) -> Vec<usize> {
        let mut order = Vec::with_capacity(self.max);
        let first = self.current;
        let mut current = self.current;
        order.push(current);
        while self.next[current] != first {
            current = self.next[current];
            order.push(current);
        }
        order
    }

    fn from(order: &Vec<usize>) -> Game {
        let mut next = vec![0; order.len()];
        for i in 0..order.len() {
            let j = (i+1) % order.len();
            next[order[i]-1] = order[j]-1;
        }
        let game = Game {
            next,
            current: order[0]-1,
            max: order.len(),
        };
        game
    }
}

impl Display for Game {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", String::from_utf8(
            self.get_order().iter().map(|i| b'0' + (i+1) as u8).collect::<Vec<u8>>()
        ).unwrap())
    }
}

pub fn solve(data : &Vec<String>, part : Part) {
    let order: Vec<usize> = data[0].as_bytes().iter().map(|b| (b-b'0') as usize).collect();

    match part {
        Part::First => {
            let mut game = Game::from(&order);
            for _ in 0..100 {
                game.make_move();
            }
            println!("{}", game);
        },
        Part::Second => {
            let mut new_order = order.clone();
            new_order.resize(1_000_000, 0);
            for i in order.len()..1_000_000 {
                new_order[i] = i+1;
            }
            let mut game = Game::from(&new_order);
            for _ in 0..10_000_000 {
                game.make_move();
            }
            let first = game.next[0];
            let second = game.next[first];
            println!("{}, {}, {}", first+1, second+1, (first+1)*(second+1));
        },
    }
}