use crate::common::Part;

struct Card {
    win: Vec<i32>,
    have: Vec<i32>,
}

impl Card {
    fn num_winning(&self) -> i32 {
        self.have.iter().map(|n| if self.win.contains(n) { 1 } else { 0 } ).sum()
    }
    fn points(&self) -> i32 {
        let c = self.num_winning();
        if c == 0 {
            0
        } else {
            1 << (c - 1)
        }
    }
}

fn process_cards(cards: &Vec<Card>) -> i32 {
    let mut card_count = vec![1; cards.len()];
    for (i, c) in cards.iter().enumerate() {
        let w = c.num_winning();
        for j in 0..w {
            card_count[i + j as usize + 1] += card_count[i]
        }
    }
    card_count.iter().sum()
}

pub fn solve(data : &Vec<String>, part : Part) {
    let mut cards = vec![];
    for s in data {
        let c = s.find(':').unwrap();
        let mut splitter = s[c+2..].split(" | ");
        let p1 = splitter.next().unwrap();
        let p2 = splitter.next().unwrap();

        let win : Vec<i32> = p1.split_whitespace().map(|s| s.parse().unwrap()).collect();
        let have : Vec<i32> = p2.split_whitespace().map(|s| s.parse().unwrap()).collect();
        cards.push(Card{ win, have });
    }

    match part {
        Part::First => {
            println!("{}", cards.iter().map(|c| c.points()).sum::<i32>());
        }
        Part::Second => {
            println!("{}", process_cards(&cards));

        }
    }
}
