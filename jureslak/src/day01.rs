use crate::common::Part;

// Accepts a sorted list of ints and returns the first pair that sums to value.
pub fn sum_of_two(arr : &[i64], value: i64) -> Result<(usize, usize), &str> {
    let mut j = arr.len() - 1;
    for i in 0..arr.len() {
        if i >= j { break }
        while j > i && arr[i] + arr[j] >= value {
            if  arr[i] + arr[j] == value {
                return Ok((i, j));
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

pub fn solve(data : &Vec<String>, part : Part) {
    let mut nums: Vec<i64> = data.iter()
        .map(|l| l.parse().expect("Failed to parse an integer")).collect();
    nums.sort();
    let nums = nums;

    match part {
        Part::First => {
            let (i, j) = sum_of_two(&nums[..], 2020).unwrap();
            println!("{}", nums[i]*nums[j]);
        }
        Part::Second => {
            let (i, j, k) = sum_of_three(&nums[..], 2020).unwrap();
            println!("{}", nums[i]*nums[j]*nums[k]);
        }
    }
}
