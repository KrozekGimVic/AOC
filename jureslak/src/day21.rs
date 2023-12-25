use std::collections::VecDeque;
use crate::common::Part;

fn dist(starts: &Vec<(i32, i32)>, map: &Vec<Vec<u8>>) -> Vec<Vec<i32>> {
    let h = map.len() as i32;
    let w = map[0].len() as i32;
    let mut dist = vec![vec![-1; w as usize]; h as usize];
    let mut q = VecDeque::new();
    for &(i, j) in starts {
        dist[i as usize][j as usize] = 0;
        q.push_back((0, i, j));
    }
    while !q.is_empty() {
        let (d, i, j) = q.pop_front().unwrap();
        for (di, dj) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
            let ni = i+di;
            let nj = j+dj;

            if 0 <= ni && ni < h && 0 <= nj && nj < w && map[ni as usize][nj as usize] == b'.' && dist[ni as usize][nj as usize] == -1 {
                dist[ni as usize][nj as usize] = d+1;
                q.push_back((d+1, ni, nj));
            }
        }
    }
    dist
}

pub fn solve(data : &Vec<String>, part : Part) {
    let mut map : Vec<Vec<u8>> = data.iter().map(|s| s.as_bytes().to_vec()).collect();
    let h = map.len();
    let w = map[0].len();

    let mut starts = vec![];
    for i in 0..h {
        for j in 0..w {
            if map[i][j] == b'S' {
                starts.push((i as i32, j as i32));
                map[i][j] = b'.';
            }
        }
    }


    match part {
        Part::First => {
            let distmap = dist(&starts, &map);
            const LIM : i32 = 64;
            let mut c = 0;
            for i in 0..h {
                for j in 0..w {
                    let d = distmap[i][j];
                    if d != -1 && d <= LIM && d % 2 == LIM % 2 {
                        c += 1;
                    }
                }
            }
            for s in distmap {
                println!("{:?}", s);
            }
            println!("{}", c);
        }
        Part::Second => {
            println!("{}", 3);
        }
    }
}
