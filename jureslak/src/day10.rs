use std::collections::{HashSet, VecDeque};
use crate::common::Part;
use std::iter::FromIterator;

fn get_loop(map: &Vec<Vec<u8>>) -> Vec<(usize, usize)> {
    let h = map.len();
    let w = map[0].len();
    let mut s = None;
    'out: for i in 0..h {
        for j in 0..w {
            if map[i][j] == b'S' {
                s = Some((i, j));
                break 'out;
            }
        }
    }
    assert!(s.is_some());
    let (si, sj) = s.unwrap();

    let mut vis = vec![vec![false; w]; h];
    let mut q = VecDeque::new();
    q.push_back((si, sj));
    vis[si][sj] = true;
    let mut res = vec![];
    while !q.is_empty() {
        let (ci, cj) = q.pop_back().unwrap();
        // println!("{} {} {}", ci, cj, map[ci][cj] as char);
        res.push((ci, cj));

        if ci > 0 && !vis[ci-1][cj] && has_up(map[ci][cj]) && has_down(map[ci-1][cj]) {
            vis[ci-1][cj] = true;
            q.push_back((ci-1, cj));
        }
        if cj > 0 && !vis[ci][cj-1] && has_left(map[ci][cj]) && has_right(map[ci][cj-1]) {
            vis[ci][cj-1] = true;
            q.push_back((ci, cj-1));
        }
        if ci+1 < h && !vis[ci+1][cj] && has_down(map[ci][cj]) && has_up(map[ci+1][cj]) {
            vis[ci+1][cj] = true;
            q.push_back((ci+1, cj));
        }
        if cj+1 < w && !vis[ci][cj+1] && has_right(map[ci][cj]) && has_left(map[ci][cj+1]) {
            vis[ci][cj+1] = true;
            q.push_back((ci, cj+1));
        }
    }
    res
}

fn has_left(a: u8) -> bool { a == b'J' || a == b'7' || a == b'-' || a == b'S' }
fn has_right(a: u8) -> bool { a == b'F' || a == b'L' || a == b'-' || a == b'S' }
fn has_up(a: u8) -> bool { a == b'|' || a == b'L' || a == b'J' || a == b'S' }
fn has_down(a: u8) -> bool { a == b'|' || a == b'F' || a == b'7' || a == b'S' }

pub fn solve(data : &Vec<String>, part : Part) {
    let mut map : Vec<Vec<u8>> = data.iter().map(|s| s.as_bytes().to_vec()).collect();
    let cyc = get_loop(&map);

    match part {
        Part::First => {
            println!("{}", cyc.len()/2);
        }
        Part::Second => {
            let h = map.len();
            let w = map[0].len();
            let pts : HashSet<(usize, usize)> = HashSet::from_iter(cyc.clone().into_iter());
            let (mut di, mut dj) = ((cyc[1].0-cyc[0].0) as isize, (cyc[1].1-cyc[0].1) as isize);
            for (ci, cj) in cyc {
                match map[ci][cj] {
                    b'|' | b'-' => {
                        mark_right(&mut map, ci, cj, di, dj, &pts);
                    }
                    b'L' => {
                        mark_right(&mut map, ci, cj, di, dj, &pts);
                        if (di, dj) == (1, 0) { (di, dj) = (0, 1); }
                        else if (di, dj) == (0, -1) { (di, dj) = (-1, 0); }
                        else { panic!("L"); }
                        mark_right(&mut map, ci, cj, di, dj, &pts);
                    }
                    b'F' => {
                        mark_right(&mut map, ci, cj, di, dj, &pts);
                        if (di, dj) == (-1, 0) { (di, dj) = (0, 1); }
                        else if (di, dj) == (0, -1) { (di, dj) = (1, 0); }
                        else { panic!("F"); }
                        mark_right(&mut map, ci, cj, di, dj, &pts);
                    }
                    b'J' => {
                        mark_right(&mut map, ci, cj, di, dj, &pts);
                        if (di, dj) == (1, 0) { (di, dj) = (0, -1); }
                        else if (di, dj) == (0, 1) { (di, dj) = (-1, 0); }
                        else { panic!("J"); }
                        mark_right(&mut map, ci, cj, di, dj, &pts);
                    }
                    b'7' => {
                        mark_right(&mut map, ci, cj, di, dj, &pts);
                        if (di, dj) == (-1, 0) { (di, dj) = (0, -1); }
                        else if (di, dj) == (0, 1) { (di, dj) = (1, 0); }
                        else { panic!("J"); }
                        mark_right(&mut map, ci, cj, di, dj, &pts);
                    }
                    _ => ()
                }
            }

            for line in &map {
                println!("{}", String::from_utf8(line.to_vec()).unwrap())
            }

            let mut q = VecDeque::new();
            for i in 0..h {
                for j in 0..w {
                    if map[i][j] == b'I' {
                        q.push_back((i, j));
                    }
                }
            }
            while !q.is_empty() {
                let (ci, cj) = q.pop_front().unwrap();
                // println!("{} {} {}", ci, cj, map[ci][cj] as char);

                if ci > 0 && map[ci-1][cj] != b'I' && !pts.contains(&(ci-1, cj)) {
                    map[ci-1][cj] = b'I';
                    q.push_back((ci-1, cj));
                }
                if cj > 0 && map[ci][cj-1] != b'I' && !pts.contains(&(ci, cj-1)) {
                    map[ci][cj-1] = b'I';
                    q.push_back((ci, cj-1));
                }
                if ci+1 < h && map[ci+1][cj] != b'I' && !pts.contains(&(ci+1, cj)) {
                    map[ci+1][cj] = b'I';
                    q.push_back((ci+1, cj));
                }
                if cj+1 < w && map[ci][cj+1] != b'I' && !pts.contains(&(ci, cj+1)) {
                    map[ci][cj+1] = b'I';
                    q.push_back((ci, cj+1));
                }
            }

            let mut c = 0;
            for i in 0..h {
                for j in 0..w {
                    if map[i][j] == b'I' {
                        c += 1;
                    }
                }
            }

            println!("{}", c);

        }
    }
}

fn mark_right(map: &mut Vec<Vec<u8>>, ci: usize, cj: usize, di: isize, dj: isize, pts: &HashSet<(usize, usize)>) {
    let ni = (ci as isize + dj) as usize;
    let nj = (cj as isize - di) as usize;
    if !pts.contains(&(ni, nj)) {
        map[ni][nj] = b'I';
    }
}
