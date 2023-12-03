use crate::common::Part;

struct EnginePart {
    num: i32,
}

fn is_digit(c: u8) -> bool {
    b'0' <= c && c <= b'9'
}

fn is_symbol(c: u8) -> bool {
    return c != b'.' && !is_digit(c)
}

fn find_part_number(data: &mut [u8], i: i32, w: i32) -> EnginePart {
    let mut s = i;
    while s >= 0 && is_digit(data[s as usize]) {
        s -= 1;
    }
    s += 1;

    let mut e = i;
    while e < w && is_digit(data[e as usize]) {
        e += 1;
    }

    let s = s as usize;
    let e = e as usize;
    let num : i32 = std::str::from_utf8(&data[s..e]).unwrap().parse().unwrap();
    for c in &mut data[s..e] {
        *c = b'.'
    }
    EnginePart { num}
}

fn find_part_numbers(data: &mut Vec<Vec<u8>>, i: i32, j: i32, h: i32, w: i32) -> Vec<EnginePart> {
    let mut parts = vec![];
    for di in -1..2 {
        for dj in -1..2 {
            if di == 0 && dj == 0 { continue; }
            let ni = i + di;
            let nj = j + dj;
            if 0 <= ni && ni < h && 0 <= nj && nj < w && is_digit(data[ni as usize][nj as usize]) {
                let symb = data[i as usize][j as usize];
                parts.push(find_part_number(&mut data[ni as usize], nj, w));
            }
        }
    }
    parts
}

fn get_all_parts(mut data: Vec<Vec<u8>>) -> Vec<EnginePart> {
    let mut parts = vec![];
    let h = data.len() as i32;
    let w = data[0].len() as i32;
    for i in 0..h {
        for j in 0..w {
            if is_symbol(data[i as usize][j as usize]) {
                parts.extend(find_part_numbers(&mut data, i, j, h, w))
            }
        }
    }
    parts
}

fn find_gear_ratio(data: &mut Vec<Vec<u8>>, i: i32, j: i32, h: i32, w: i32) -> Option<i32> {
    let mut parts = vec![];
    for di in -1..2 {
        for dj in -1..2 {
            if di == 0 && dj == 0 { continue; }
            let ni = i + di;
            let nj = j + dj;
            if 0 <= ni && ni < h && 0 <= nj && nj < w && is_digit(data[ni as usize][nj as usize]) {
                let symb = data[i as usize][j as usize];
                parts.push(find_part_number(&mut data[ni as usize], nj, w));
            }
        }
    }
    if parts.len() == 2 {
        Some(parts[0].num*parts[1].num)
    } else {
        None
    }
}

fn get_all_gear_ratios(mut data: Vec<Vec<u8>>) -> Vec<i32> {
    let mut parts = vec![];
    let h = data.len() as i32;
    let w = data[0].len() as i32;
    for i in 0..h {
        for j in 0..w {
            if data[i as usize][j as usize] == b'*' {
                parts.extend(find_gear_ratio(&mut data, i, j, h, w))
            }
        }
    }
    parts
}


pub fn solve(data : &Vec<String>, part : Part) {
    match part {
        Part::First => {
            let data : Vec<Vec<u8>> = data.iter().map(|s| s.as_bytes().to_vec()).collect();
            let sum: i32 =  get_all_parts(data).into_iter().map(|p| p.num).sum();
            println!("{}", sum);
        }
        Part::Second => {
            let data : Vec<Vec<u8>> = data.iter().map(|s| s.as_bytes().to_vec()).collect();
            let sum: i32 =  get_all_gear_ratios(data).into_iter().sum();
            println!("{}", sum);

        }
    }
}
