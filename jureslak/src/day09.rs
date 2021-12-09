use crate::common::Part;
use std::collections::VecDeque;

fn part1(map: Vec<&[u8]>) -> i32 {
    let mut cnt = 0;
    for i in 0..map.len() {
        for j in 0..map[0].len() {
            if is_min(&map, i, j) {
                cnt += (map[i][j] - b'0') as i32 + 1
            }
        }
    }
    cnt
}


fn part2(map: Vec<&[u8]>) -> i32 {
    let mut basins : Vec<i32> = vec![];
    let mut seen : Vec<Vec<bool>> = vec![vec![false; map[0].len()]; map.len()];
    for i in 0..map.len() {
        for j in 0..map[0].len() {
            if !seen[i][j] && map[i][j] != b'9' {
                basins.push(find_cc(&map, i, j, &mut seen));
            }
        }
    }
    println!("{:?}", basins);
    basins.sort();
    let n = basins.len();
    basins[n-1]*basins[n-2]*basins[n-3]
}

fn find_cc(map: &Vec<&[u8]>, i: usize, j: usize, seen: &mut Vec<Vec<bool>>) -> i32 {
    let mut cnt = 0;
    let mut q = VecDeque::from([(i, j)]);
    seen[i][j] = true;
    while !q.is_empty() {
        let (i, j) = q.pop_front().unwrap();
        cnt += 1;

        for (di, dj) in [(-1, 0), (1, 0), (0, 1), (0, -1)] {
            let ni = i as isize + di;
            let nj = j as isize + dj;
            if ni >= 0 && nj >= 0 {
                let ni = ni as usize;
                let nj = nj as usize;
                if ni < map.len() && nj < map[0].len() && !seen[ni][nj] && map[ni][nj] != b'9' {
                    q.push_back((ni, nj));
                    seen[ni][nj] = true;
                }
            }
        }
    }
    cnt
}

fn is_min(map: &Vec<&[u8]>, i: usize, j: usize) -> bool {
    (i == 0 || map[i-1][j] > map[i][j]) &&
    (j == 0 || map[i][j-1] > map[i][j]) &&
    (i == map.len() - 1 || map[i+1][j] > map[i][j]) &&
    (j == map[0].len() - 1 || map[i][j+1] > map[i][j])
}

pub fn solve(data : &Vec<String>, part : Part) {
    let bytes: Vec<&[u8]> = data.iter().map(|s| s.trim().as_bytes()).collect();

    match part {
        Part::First => {
            println!("{}", part1(bytes));
        }
        Part::Second => {
            println!("{}", part2(bytes));
        }
    }
}
