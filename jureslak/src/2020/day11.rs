use crate::common::Part;

type Board = Vec<Vec<u8>>;

const DIRS : [(isize, isize); 8] = [(-1, 0), (1, 0), (0, 1), (0, -1), (-1, -1), (1, -1), (-1, 1), (1, 1)];

fn count_occupied_near_neighbours(i: usize, j: usize, board : &Board, n: usize, m: usize) -> i32 {
    let mut cnt = 0;
    for (di, dj) in DIRS.iter() {
        let ui = i as isize + di;
        let uj = j as isize + dj;
        if 0 <= ui && ui < n as isize && 0 <= uj && uj < m as isize && board[ui as usize][uj as usize] == b'#' {
            cnt += 1;
        }
    }
    cnt
}

fn count_occupied_visible_neighbours(i: usize, j: usize, board : &Board, n: usize, m: usize) -> i32 {
    let mut cnt = 0;
    for (di, dj) in DIRS.iter() {
        let mut k = 1;
        loop {
            let ui = i as isize + di*k;
            let uj = j as isize + dj*k;
            if !(0 <= ui && ui < n as isize && 0 <= uj && uj < m as isize) { break; }
            match board[ui as usize][uj as usize] {
                b'#' => {
                    cnt += 1;
                    break;
                }
                b'L' => break,
                _ => (),
            }
            k += 1;
        }
    }
    cnt
}

fn one_step(board : &Board, neighbour_fn : fn(usize, usize, &Board, usize, usize) -> i32, die_limit: i32) -> Board {
    let mut new_board = board.clone();
    let n = board.len();
    let m = board[0].len();
    for i in 0..n {
        for j in 0..m {
            if board[i][j] != b'.' {
                let occ_neighbours = neighbour_fn(i, j, &board, n, m);
                if board[i][j] == b'#' && occ_neighbours >= die_limit {
                    new_board[i][j] = b'L';
                } else if board[i][j] == b'L' && occ_neighbours == 0 {
                    new_board[i][j] = b'#';
                }
            }
        }
    }
    new_board
}

fn simulate(initial_state: Board, neighbour_fn : fn(usize, usize, &Board, usize, usize) -> i32, die_limit: i32) -> Board {
    let mut state1 = initial_state;
    let mut state2 = one_step(&state1, neighbour_fn, die_limit);
    while state1 != state2 {
        state1 = state2;
        state2 = one_step(&state1, neighbour_fn, die_limit);
    }
    state2
}

fn count_occupied(board: &Board) -> usize {
    board.iter().map(|v| v.iter().filter(|&b| *b == b'#').count()).sum::<usize>()
}

pub fn solve(data : &Vec<String>, part : Part) {
    let board : Board = data.iter().map(|l| l.trim().into()).collect();
    match part {
        Part::First => {
            let final_state = simulate(board, count_occupied_near_neighbours, 4);
            println!("{}", count_occupied(&final_state));
        },
        Part::Second => {
            let final_state = simulate(board, count_occupied_visible_neighbours, 5);
            println!("{}", count_occupied(&final_state));
        },
    }
}
