use crate::common::Part;

struct Case<'a> {
    data: Vec<&'a [u8]>
}

#[derive(Debug)]
enum Reflection {
    LR(usize),
    UD(usize),
}

fn check_ud(i: usize, data: &Vec<&[u8]>, w: usize, h: usize) -> i32 {
    let mut mis = 0;
    for j in 1..=std::cmp::min(i, h-i) {
        for k in 0..w {
            if data[i-j][k] != data[i+j-1][k] {
                mis += 1;
            }
        }
    }
    mis
}


fn check_lr(i: usize, data: &Vec<&[u8]>, w: usize, h: usize) -> i32 {
    let mut mis = 0;
    for j in 1..=std::cmp::min(i, w-i) {
        for k in 0..h {
            if data[k][i-j] != data[k][i+j-1] {
                mis += 1;
            }
        }
    }
    mis
}

fn compute_reflection(case: &Case) -> Reflection {
    let h = case.data.len();
    let w = case.data[0].len();

    for l in &case.data {
        println!("{}", String::from_utf8(l.to_vec()).unwrap());
    }

    for i in 1..h {
        if check_ud(i, &case.data, w, h) == 0 {
            return Reflection::UD(i);
        }
    }

    for i in 1..w {
        if check_lr(i, &case.data, w, h) == 0 {
            return Reflection::LR(i);
        }
    }

    panic!("No reflection!");
}

fn compute_reflection2(case: &Case) -> Reflection {
    let h = case.data.len();
    let w = case.data[0].len();

    for l in &case.data {
        println!("{}", String::from_utf8(l.to_vec()).unwrap());
    }

    for i in 1..h {
        if check_ud(i, &case.data, w, h) == 1 {
            return Reflection::UD(i);
        }
    }

    for i in 1..w {
        if check_lr(i, &case.data, w, h) == 1 {
            return Reflection::LR(i);
        }
    }

    panic!("No reflection!");
}

pub fn solve(data : &Vec<String>, part : Part) {
    let mut cases = vec![Case{ data: vec![] }];
    for line in data {
        if line.is_empty() {
            let last = cases.last().unwrap();
            let w = last.data[0].len();
            for s in &last.data { assert_eq!(s.len(), w); }
            cases.push(Case{ data: vec![] });
            continue;
        }
        cases.last_mut().unwrap().data.push(line.as_bytes());
    }


    match part {
        Part::First => {
            let r : usize = cases.iter().map(|c| compute_reflection(c)).map(|r| {
                println!("{:?}", r);
                match r {
                    Reflection::LR(i) => i,
                    Reflection::UD(i) => 100*i,
                }
            }).sum();
            println!("{}", r);
        }
        Part::Second => {
            let r : usize = cases.iter().map(|c| compute_reflection2(c)).map(|r| {
                println!("{:?}", r);
                match r {
                    Reflection::LR(i) => i,
                    Reflection::UD(i) => 100*i,
                }
            }).sum();
            println!("{}", r);

        }
    }
}
