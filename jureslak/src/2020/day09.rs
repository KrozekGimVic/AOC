use crate::common::Part;
use crate::day01::sum_of_two;

fn find_mismatch(v: &Vec<i64>, preamble_length: usize) -> Result<i64, &str> {
    for i in preamble_length..v.len() {
        let mut part= v[i - preamble_length..i].to_vec();
        part.sort();
        let r = sum_of_two(&part, v[i]);
        match r {
            Err(_) => {
                return Ok(v[i])
            },
            _ => continue,
        }
    }
    Err("No mismatch")
}

// Works for arrays with negative numbers as well.
fn find_sum_range(v: &[i64], value : i64) -> Result<(usize, usize), &str> {
    for i in 0..v.len() {
        let mut s = 0;
        for j in i..v.len() {
            s += v[j];
            if s == value {
                return Ok((i, j+1));
            }
        }
    }
    Err("No range sums to value.")
}


pub fn solve(data : &Vec<String>, part : Part) {
    let nums : Vec<i64> = data.iter().map(|s| s.parse().unwrap()).collect();
    let mismatch = find_mismatch(&nums, 25).unwrap();
    match part {
        Part::First => println!("{}", mismatch),
        Part::Second => {
            let (i, j) = find_sum_range(&nums, mismatch).unwrap();
            let min = nums[i..j].iter().min().unwrap();
            let max = nums[i..j].iter().max().unwrap();
            println!("{}", min+max);
        }
    }
}
