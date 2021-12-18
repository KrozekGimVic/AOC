use crate::common::Part;
use regex::Regex;
use num;
use std::collections::HashSet;

struct Rect {
    min_x: i32,
    max_x: i32,
    min_y: i32,
    max_y: i32,
}

impl Rect {
    fn contains(&self, x: i32, y: i32) -> bool {
        self.min_x <= x && x <= self.max_x && self.min_y <= y && y <= self.max_y
    }
}

fn simulate_move(mut vx: i32, mut vy: i32, target: &Rect) -> (bool, i32) {
    let mut x = 0;
    let mut y = 0;
    let mut good = false;
    let mut max_y = y;
    while y >= target.min_y {
        x += vx;
        y += vy;
        vx -= num::signum(vx);
        vy -= 1;

        if y > max_y { max_y = y; }
        if target.contains(x, y) { good = true; }
    }
    (good, max_y)
}

fn best_shot(target: &Rect) -> i32 {
    let mut max_y = 0;
    for vx in 1..100 {
        for vy in 0..100 {
            let (good, max_flight_y) = simulate_move(vx, vy, target);
            if good && max_flight_y > max_y {
                max_y = max_flight_y;
            }
        }
    }
    max_y
}


fn all_shots(target: &Rect) -> usize {
    let mut velocities = HashSet::new();
    for vx in 1..1000 {
        for vy in -1000..1000 {
            let (good, _) = simulate_move(vx, vy, target);
            if good {
                velocities.insert((vx, vy));
            }
        }
    }
    velocities.len()
}


pub fn solve(data : &Vec<String>, part : Part) {
    let s = &data[0];
    let re = Regex::new(r"target area: x=(-?\d+)..(-?\d+), y=(-?\d+)..(-?\d+)").unwrap();
    let captures = re.captures(s).unwrap();
    let min_x: i32 = captures[1].parse().unwrap();
    let max_x: i32 = captures[2].parse().unwrap();
    let min_y: i32 = captures[3].parse().unwrap();
    let max_y: i32 = captures[4].parse().unwrap();
    let r = Rect { min_x, max_x, min_y, max_y };

    match part {
        Part::First => {
            println!("{}", best_shot(&r));
        }
        Part::Second => {
            println!("{}", all_shots(&r));
        }
    }
}
