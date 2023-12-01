use crate::common::Part;

fn count_increasing(heights: &Vec<i32>) -> i32 {
   heights.iter().zip(heights[1..].iter()).map(|(a, b)| (a < b) as i32).sum()
}

fn count_moving_sum_increasing(heights: &Vec<i32>, length: usize) -> i32 {
    assert!(length <= heights.len());
    let mut prev_sum : i32 = heights[..length].iter().sum();
    let mut cnt = 0;
    for i in length..heights.len() {
        let cur_sum = prev_sum - heights[i-length] + heights[i];
        if cur_sum > prev_sum {
            cnt += 1
        }
        prev_sum = cur_sum
    }
    cnt
}

pub fn solve(data : &Vec<String>, part : Part) {
    let heights: Vec<i32> = data.iter().map(|l| l.parse().expect("Invalid input")).collect();

    match part {
        Part::First => {
            println!("{}", count_increasing(&heights));
        }
        Part::Second => {
            println!("{}", count_moving_sum_increasing(&heights, 3));

        }
    }
}
