use crate::common::Part;

fn find_digit(s: &str) -> i32 {
    let mut tot = 0;
    for b in s.bytes() {
        if b'0' <= b && b <= b'9' {
            tot += (b - b'0') as i32;
            break;
        }
    }
    tot *= 10;
    for b in s.bytes().rev() {
        if b'0' <= b && b <= b'9' {
            tot += (b - b'0') as i32;
            break;
        }
    }
    tot
}

fn find_first_digit(s: &str, rev: bool) -> i32 {
    let words = vec!["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
    let digits = vec!["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"];

    let mut min = if rev { i32::MIN } else { i32::MAX };
    let mut mind = 0;
    for (i, n) in words.iter().enumerate() {
        if let Some(idx) = if rev { s.rfind(n) } else { s.find(n) } {
            let idx = idx as i32;
            if !rev && idx < min || rev && idx > min {
                min = idx;
                mind = i+1;
            }
        }
    }
    for (i, n) in digits.iter().enumerate() {
        if let Some(idx) = if rev { s.rfind(n) } else { s.find(n) } {
            let idx = idx as i32;
            if !rev && idx < min || rev && idx > min {
                min = idx;
                mind = i;
            }
        }
    }

    assert_ne!(min, if rev { i32::MIN } else { i32::MAX }, "str: {}", s);
    mind as i32
}

fn find_real_digit(s: &str) -> i32 {
    let f = find_first_digit(s, false);
    let l = find_first_digit(s, true);
    10*f + l
}

pub fn solve(data : &Vec<String>, part : Part) {

    match part {
        Part::First => {
            let cal_values: Vec<i32> = data.iter().map(|s| find_digit(s)).collect();
            println!("{}", cal_values.iter().sum::<i32>());
        }
        Part::Second => {
            let cal_values: Vec<i32> = data.iter().map(|s| find_real_digit(s)).collect();
            println!("{}", cal_values.iter().sum::<i32>());

        }
    }
}
