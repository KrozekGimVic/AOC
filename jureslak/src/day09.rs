use crate::common::Part;

fn extrapolate(data: &Vec<i32>) -> i32 {
    let n = data.len();
    let mut m : Vec<Vec<i32>> = vec![vec![0; n+1]; n+1];
    for j in 0..n {
        m[0][j] = data[j];
    }
    for i in 1..n {
        for j in 0..(n-i) {
            m[i][j] = m[i-1][j+1] - m[i-1][j];
        }
    }
    for i in 1..n {
        m[n-i-1][i+1] = m[n-i][i] + m[n-i-1][i]
    }
    m[0][n]
}

pub fn solve(data : &Vec<String>, part : Part) {
    let nums : Vec<Vec<i32>> = data.iter().map(|s| s.split_whitespace().map(|x| x.parse().unwrap()).collect()).collect();
    match part {
        Part::First => {
            let r : i32 = nums.iter().map(extrapolate).sum();
            println!("{}", r);
        }
        Part::Second => {
            let r : i32 = nums.iter().map(|r| {
                let mut v = r.clone();
                v.reverse();
                extrapolate(&v)
            }).sum();
            println!("{}", r);

        }
    }
}
