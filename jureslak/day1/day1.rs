use std::io;
use std::io::BufRead;

// Accepts a sorted list of ints and returns the first pair that sums to value.
fn sum_of_two(arr : &[i64], value: i64) -> Result<(usize, usize), &str> {
    let mut j = arr.len() - 1;
    for i in 0..arr.len() {
        if i >= j { break }
        while j > i {
            if  arr[i] + arr[j] == value {
                return Ok((i, j));
            } else if arr[i] + arr[j] < value {
                j += 1;
                break;
            }
            j -= 1;
        }
    }
    Err("The list does not contain the desired value.")
}

// Accepts a sorted list of ints and returns the first triplet that sums to value.
fn sum_of_three(arr : &[i64], value: i64) -> Result<(usize, usize, usize), &str> {
    for (i, v) in arr.iter().enumerate() {
        let offset = i+1;
        let r = sum_of_two(&arr[offset..], value - v);
        match r {
            Ok((j, k)) => return Ok((i, offset+j, offset+k)),
            _ => ()
        }
    }
    Err("The list does not contain the desired value.")
}

fn main() {
    let mut nums: Vec<i64> = io::stdin().lock().lines().map(|l| l.unwrap().parse().unwrap()).collect();
    nums.sort();

    let args : Vec<String> = std::env::args().collect();
    let part = args[1].parse().unwrap();
    match part {
        1 => {
            let (i, j) = sum_of_two(&nums[..], 2020).unwrap();
            println!("{}", nums[i]*nums[j]);
        }
        2 => {
            let (i, j, k) = sum_of_three(&nums[..], 2020).unwrap();
            println!("{}", nums[i]*nums[j]*nums[k]);
        }
        _ => {
            panic!("Invalid part.")
        }
    }
}
