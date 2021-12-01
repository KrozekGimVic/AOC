use crate::common::Part;
use std::collections::{VecDeque, HashSet};
use std::cmp::{max,min};

#[derive(Hash, PartialEq, Eq, Clone, Debug)]
struct Player {
    deck: VecDeque<i32>,
}

impl Player {
    fn from(s: &str) -> Player {
        Player {
            deck: s.split("\n").skip(1).map(|s| s.parse().unwrap()).collect()
        }
    }

    fn score(&self) -> i64 {
        self.deck.iter().enumerate().map(|(i, v)| {
            (self.deck.len() - i) as i64 * (*v as i64)
        }).sum()
    }
}

fn play(mut p1: Player, mut p2: Player) -> (Player, Player) {
    while !p1.deck.is_empty() && !p2.deck.is_empty() {
        let t1 = p1.deck.pop_front().unwrap();
        let t2 = p2.deck.pop_front().unwrap();
        let winner = if t1 > t2 { &mut p1 } else { &mut p2 };
        winner.deck.push_back(max(t1, t2));
        winner.deck.push_back(min(t1, t2));
    }
    (p1, p2)
}

#[derive(Hash, PartialEq, Eq, Clone)]
struct GameState {
    p1: Player,
    p2: Player,
}

fn play_recursive(mut p1: Player, mut p2: Player) -> (Player, Player) {
    let mut memo = HashSet::new();

    while !p1.deck.is_empty() && !p2.deck.is_empty() {
        // println!("{:?}, {:?}", p1, p2);
        if memo.contains(&GameState { p1: p1.clone(), p2: p2.clone() }) {
            // instant player 1 win
            p2.deck.clear();
            return (p1, p2);
        }
        memo.insert(GameState{ p1: p1.clone(), p2: p2.clone() });

        let t1 = p1.deck.pop_front().unwrap();
        let t2 = p2.deck.pop_front().unwrap();

        let winner = if p1.deck.len() >= t1 as usize && p2.deck.len() >= t2 as usize {
            let (_, sub_p2) = play_recursive(
                Player{ deck: p1.deck.iter().take(t1 as usize).map(|i| *i).collect() },
                Player{ deck: p2.deck.iter().take(t2 as usize).map(|i| *i).collect() },
            );
            if sub_p2.deck.is_empty() { 1 } else { 2 }
        } else {
            if t1 < t2 { 2 } else { 1 }
        };
        if winner == 2 {
            p2.deck.push_back(t2);
            p2.deck.push_back(t1);
        } else {
            p1.deck.push_back(t1);
            p1.deck.push_back(t2);
        }
    }
    (p1, p2)
}


pub fn solve(data : &Vec<String>, part : Part) {
    let data = data.join("\n");
    let parts : Vec<_> = data.split("\n\n").collect();
    let p1 = Player::from(parts[0]);
    let p2 = Player::from(parts[1]);

    match part {
        Part::First => {
            let (p1, p2) = play(p1, p2);
            println!("{}", p1.score() + p2.score());
        },
        Part::Second => {
            let (p1, p2) = play_recursive(p1, p2);
            println!("{:?}, {:?}", p1, p2);
            println!("{}", p1.score() + p2.score());
        },
    }
}