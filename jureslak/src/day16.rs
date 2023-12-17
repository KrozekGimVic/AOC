use crate::common::Part;

fn turn_right(di: i32, dj: i32) -> (i32, i32) {
    (dj, -di)
}

fn turn_left(di: i32, dj: i32) -> (i32, i32) {
    (-dj, di)
}

fn dir2num(di: i32, dj: i32) -> usize {
    if di == 0 && dj == 1 {
        0
    } else if di == 0 && dj == -1 {
        1
    } else if di == 1 && dj == 0 {
        2
    } else {
        3
    }
}

fn reflect(map : &Vec<Vec<u8>>, i: i32, j: i32, di: i32, dj: i32) -> i32 {
    let h = map.len();
    let w = map[0].len();

    let mut seen = vec![vec![[false; 4]; w]; h];
    reflect_impl(i, j, di, dj, h as i32, w as i32, &map, &mut seen);
    let mut c = 0;
    for i in 0..h {
        for j in 0..w {
            if seen[i][j].iter().any(|&c| c == true) {
                c += 1
            }
        }
    }
    c
}

fn reflect_impl(i: i32, j: i32, mut di: i32, mut dj: i32, h: i32, w: i32, map : &Vec<Vec<u8>>, seen: &mut Vec<Vec<[bool; 4]>>) {
    if !(0 <= i && i < h) || !(0 <= j && j < w) {
        return;
    }
    let s = &mut seen[i as usize][j as usize][dir2num(di, dj)];
    if *s == true {
        return;
    }
    *s = true;
    let c = map[i as usize][j as usize];
    match c {
        b'.' => reflect_impl(i+di, j+dj, di, dj, h, w, map, seen),
        b'/' => {
            if di == 0 {
                (di, dj) = turn_left(di, dj);
                reflect_impl(i+di, j+dj, di, dj, h, w, map, seen);
            } else {
                (di, dj) = turn_right(di, dj);
                reflect_impl(i+di, j+dj, di, dj, h, w, map, seen);
            }
        }
        b'\\' => {
            if di == 0 {
                (di, dj) = turn_right(di, dj);
                reflect_impl(i+di, j+dj, di, dj, h, w, map, seen);
            } else {
                (di, dj) = turn_left(di, dj);
                reflect_impl(i+di, j+dj, di, dj, h, w, map, seen);
            }
        }
        b'-' => {
            if di == 0 {
                reflect_impl(i+di, j+dj, di, dj, h, w, map, seen)
            } else {
                let (di1, dj1) = turn_left(di, dj);
                reflect_impl(i+di1, j+dj1, di1, dj1, h, w, map, seen);

                (di, dj) = turn_right(di, dj);
                reflect_impl(i+di, j+dj, di, dj, h, w, map, seen);
            }
        }
        b'|' => {
            if dj == 0 {
                reflect_impl(i+di, j+dj, di, dj, h, w, map, seen)
            } else {
                let (di1, dj1) = turn_left(di, dj);
                reflect_impl(i+di1, j+dj1, di1, dj1, h, w, map, seen);

                (di, dj) = turn_right(di, dj);
                reflect_impl(i+di, j+dj, di, dj, h, w, map, seen);
            }
        }
        _ => panic!("Unknown char {}", c as char),
    }
}

pub fn solve(data : &Vec<String>, part : Part) {
    let map : Vec<Vec<u8>> = data.iter().map(|s| s.as_bytes().to_vec()).collect();
    match part {
        Part::First => {
            println!("{}", reflect(&map, 0, 0, 0, 1));
        }
        Part::Second => {
            let mut best = i32::MIN;
            for i in 0..map.len() as i32 {
                best = best.max(reflect(&map, i, 0, 0, 1));
                best = best.max(reflect(&map, i, 0, 0, -1));
            }
            for i in 0..map[0].len() as i32 {
                best = best.max(reflect(&map, 0, i, 1, 0));
                best = best.max(reflect(&map, 0, i, -1, 0));
            }

            println!("{}", best);

        }
    }
}
