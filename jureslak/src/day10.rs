use std::iter::zip;
use crate::common::Part;

fn num_wins(max_time: i64, target_dist: i64) -> i32 {
    let mut num_wins = 0;
    for t in 1..max_time {
        // hold for t, get speed t, run for max_time-t seconds
        let speed = t;
        let run_time = max_time-t;
        let dist = run_time*speed;
        if dist > target_dist {
            num_wins += 1;
        }
    }
    num_wins
}

pub fn solve(data : &Vec<String>, part : Part) {
    match part {
        Part::First => {
            let times : Vec<i32> = data[0].split_whitespace().skip(1).map(|s| s.parse().unwrap()).collect();
            let dist : Vec<i32> = data[1].split_whitespace().skip(1).map(|s| s.parse().unwrap()).collect();
            println!("{}", zip(times.iter(), dist.iter()).map(|(&a, &b)| num_wins(a as i64, b as i64)).product::<i32>());
        }
        Part::Second => {
            let time : i64 = data[0].split_whitespace().skip(1).collect::<Vec<_>>().join("").parse().unwrap();
            let dist : i64 = data[1].split_whitespace().skip(1).collect::<Vec<_>>().join("").parse().unwrap();
            println!("{}", num_wins(time, dist));

        }
    }
}
