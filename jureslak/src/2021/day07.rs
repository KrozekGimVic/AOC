use crate::common::Part;

pub fn solve(data : &Vec<String>, part : Part) {
    let mut nums : Vec<i32> = data[0].split(',').map(|s| s.parse().expect("int")).collect();
    nums.sort();

    match part {
        Part::First => {
            let med = nums[nums.len()/2];
            println!("{}", nums.iter().map(|n| (n-med).abs()).sum::<i32>());
        }
        Part::Second => {
            let mean : i32 = nums.iter().sum::<i32>() / nums.len() as i32;
            println!("{}", nums.iter().map(|&n| { let d = (n-mean).abs(); d*(d+1)/2 }).sum::<i32>());
        }
    }
}
