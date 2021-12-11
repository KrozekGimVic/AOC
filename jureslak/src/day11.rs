use crate::common::Part;
use std::collections::VecDeque;

fn evolve(mut board: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let n = board.len();
    let m = board[0].len();
    let mut flashed = vec![vec![false; m]; n];
    let mut q = VecDeque::new();
    for i in 0..n {
        for j in 0..m {
            board[i][j] += 1;
            if board[i][j] > 9 {
                q.push_back((i, j));
            }
        }
    }
    while !q.is_empty() {
        let (i, j) = q.pop_front().unwrap();
        if flashed[i][j] { continue; }
        flashed[i][j] = true;

        for di in [-1, 0, 1] {
            for dj in [-1, 0, 1] {
                let ni = i as isize + di;
                let nj = j as isize + dj;
                if ni >= 0 && nj >= 0 {
                    let ni = ni as usize;
                    let nj = nj as usize;
                    if ni < n && nj < m && !flashed[ni][nj] {
                        board[ni][nj] += 1;
                        if board[ni][nj] > 9 {
                            q.push_back((ni, nj));
                        }
                    }
                }
            }
        }
    }
    for i in 0..n {
        for j in 0..m {
            if board[i][j] > 9 {
                board[i][j] = 0;
            }
        }
    }
    board
}

pub fn solve(data : &Vec<String>, part : Part) {
    let mut board : Vec<Vec<i32>> = data.iter().map(
        |s| s.as_bytes().iter().map(|c| (c-b'0') as i32).collect()
    ).collect();

    match part {
        Part::First => {
            let mut tot_flash = 0;
            for _i in 0..100 {
                board = evolve(board);
                tot_flash += board.iter().map(|v| v.iter()).flatten().filter(|&&c| c == 0).count();
                // for l in board.iter() {
                //     println!("{:?}", l);
                // }
                // println!("------------------");
            }
            println!("{}", tot_flash);
        }
        Part::Second => {
            let mut i = 0;
            loop {
                board = evolve(board);
                i += 1;
                if board.iter().map(|v| v.iter()).flatten().all(|&c| c == 0) {
                    break;
                }
            };
            println!("{}", i);
        }
    }
}
