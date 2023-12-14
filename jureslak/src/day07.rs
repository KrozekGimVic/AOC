use std::cmp::Ordering;
use crate::common::Part;

#[derive(PartialEq, Eq)]
struct Card {
    str: String,
    bid: i32,
}

#[derive(Eq, Ord, PartialEq, PartialOrd, Debug)]
enum Rank {
    Five,
    Four,
    FullHouse,
    Three,
    TwoPair,
    OnePair,
    High,
}

impl Rank {
    fn from_count_counts(cc: &[i32; 6]) -> Rank {
        if cc[5] == 1 {
            Rank::Five
        } else if cc[4] == 1 {
            Rank::Four
        } else if cc[3] == 1 && cc[2] == 1 {
            Rank::FullHouse
        } else if cc[3] == 1 {
            Rank::Three
        } else if cc[2] == 2 {
            Rank::TwoPair
        } else if cc[2] == 1 {
            Rank::OnePair
        } else {
            Rank::High
        }
    }
}

fn card_to_idx(c: char) -> usize {
    match c {
        'A' => 0,
        'K' => 1,
        'Q' => 2,
        'J' => 3,
        'T' => 4,
        '9' => 5,
        '8' => 6,
        '7' => 7,
        '6' => 8,
        '5' => 9,
        '4' => 10,
        '3' => 11,
        '2' => 12,
        _ => panic!()
    }
}

fn card_to_idx2(c: char) -> usize {
    match c {
        'A' => 0,
        'K' => 1,
        'Q' => 2,
        'T' => 3,
        '9' => 4,
        '8' => 5,
        '7' => 6,
        '6' => 7,
        '5' => 8,
        '4' => 9,
        '3' => 10,
        '2' => 11,
        'J' => 12,
        _ => panic!()
    }
}

impl Card {
    fn rank(&self) -> Rank {
        let mut counts = [0; 13];
        for c in self.str.chars() {
            counts[card_to_idx(c)] += 1;
        }

        let mut cc = [0; 6];
        for c in counts {
            cc[c] += 1;
        }
        Rank::from_count_counts(&cc)
    }

    fn str_to_vec(&self) -> Vec<usize> {
        self.str.chars().map(card_to_idx).collect()
    }

    fn rank2(&self) -> Rank {
        Self::max_rank(self.str.clone())
    }

    fn max_rank(s: String) -> Rank {
        for c in s.chars() {
            if c == 'J' {
                return "AKQT98765432".chars().map(|rc| {
                     Self::max_rank(s.replacen(c, &rc.to_string(), 1))
                }).min().unwrap()
            }
        }
        Card { str: s, bid: 0}.rank()
    }

    fn str_to_vec2(&self) -> Vec<usize> {
        self.str.chars().map(card_to_idx2).collect()
    }
}

impl Card {
    fn cmp(&self, other: &Self) -> Ordering {
        (self.rank(), self.str_to_vec()).cmp(&(other.rank(), other.str_to_vec()))
    }

    fn cmp2(&self, other: &Self) -> Ordering {
        (self.rank2(), self.str_to_vec2()).cmp(&(other.rank2(), other.str_to_vec2()))
    }
}

pub fn solve(data : &Vec<String>, part : Part) {
    let mut cards : Vec<Card> = data.iter().map(|s| {
        let mut splitter = s.split_whitespace();
        let s = splitter.next().unwrap();
        let b = splitter.next().unwrap();
        Card { str: s.to_string(), bid: b.parse().unwrap() }
    }).collect();

    match part {
        Part::First => {
            cards.sort_by(|a, b| a.cmp(b));
            // for c in &cards {
            //     println!("{}", c.str);
            // }
            let n = cards.len();
            let r : i32 = cards.iter().enumerate().map(|(i, c)| (n-i) as i32*c.bid).sum();
            println!("{}", r);
        }
        Part::Second => {
            cards.sort_by(|a, b| a.cmp2(b));
            let n = cards.len();
            let r : i32 = cards.iter().enumerate().map(|(i, c)| (n-i) as i32*c.bid).sum();
            println!("{}", r);

        }
    }
}
