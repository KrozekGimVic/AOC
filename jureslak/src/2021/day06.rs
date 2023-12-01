use crate::common::Part;

type State = [i64; 9];
#[derive(Debug)]
struct FishTracker {
    fish_counts: State,
}

impl FishTracker {
    fn from(fish_list: &Vec<i32>) -> FishTracker {
        let mut fish_counts = [0; 9];
        for f in fish_list.iter() {
            fish_counts[*f as usize] += 1;
        }
        FishTracker{ fish_counts }
    }

    fn num_fish(&self) -> i64 {
        self.fish_counts.iter().sum()
    }
}

impl Iterator for FishTracker {
    type Item = State;

    fn next(&mut self) -> Option<Self::Item> {
        let mut new_fish_counts = [0; 9];

        for i in 1..9 {
            new_fish_counts[i-1] = self.fish_counts[i];
        }
        new_fish_counts[6] += self.fish_counts[0];
        new_fish_counts[8] += self.fish_counts[0];
        self.fish_counts = new_fish_counts;

        Some(self.fish_counts)
    }
}

pub fn solve(data : &Vec<String>, part : Part) {
    let fish_timers : Vec<i32> = data[0].split(',').map(|s| s.parse().expect("int")).collect();
    let mut tracker = FishTracker::from(&fish_timers);

    match part {
        Part::First => {
            for _ in 0..80 {
                tracker.next();
            }
            println!("{}", tracker.num_fish());
        }
        Part::Second => {
            for _ in 0..256 {
                tracker.next();
            }
            println!("{}", tracker.num_fish());
        }
    }
}
