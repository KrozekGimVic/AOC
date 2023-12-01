use crate::common::Part;

use std::cmp::Ordering;
use std::collections::BinaryHeap;

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: i32,
    position: (usize, usize),
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn shortest_path(grid: Vec<&[u8]>) -> i32 {
    // dist[node] = current shortest distance from `start` to `node`
    let n = grid.len();
    let m = grid[0].len();
    let mut dist = vec![vec![i32::MAX; m]; n];

    let mut heap = BinaryHeap::new();

    // We're at `start`, with a zero cost
    dist[0][0] = 0;
    heap.push(State { cost: 0, position: (0, 0) });

    while let Some(State { cost, position }) = heap.pop() {
        if position == (n-1, m-1) { return cost; }
        let (i, j) = position;
        if cost > dist[i][j] { continue; }

        // For each node we can reach, see if we can find a way with
        // a lower cost going through this node
        for (di, dj) in [(0, -1), (0, 1), (-1, 0), (1, 0)] {
            let ni = i as isize + di;
            let nj = j as isize + dj;
            if 0 <= ni && 0 <= nj {
                let ni = ni as usize;
                let nj = nj as usize;
                if ni < n && nj < m {
                    let next = State { cost: cost + (grid[ni][nj]-b'0') as i32, position: (ni, nj) };

                    if next.cost < dist[ni][nj] {
                        heap.push(next);
                        dist[ni][nj] = next.cost;
                    }
                }
            }
        }
    }
    unreachable!();
}

pub fn solve(data : &Vec<String>, part : Part) {
    let grid : Vec<&[u8]> = data.iter().map(|s| s.as_bytes()).collect();

    match part {
        Part::First => {
            println!("{}", shortest_path(grid));
        }
        Part::Second => {
            let n = grid.len();
            let m = grid[0].len();
            let mut big_grid = vec![vec![b'0'; 5*m]; 5*n];
            for di in 0..5 {
                for dj in 0..5 {
                    for i in 0..n {
                        for j in 0..m {
                            big_grid[di*n + i][dj*m + j] = b'1' + (((grid[i][j] - b'1') as usize + di + dj) % 9) as u8;
                        }
                    }
                }
            }
            println!("{}", shortest_path(big_grid.iter().map(|v| &v[..]).collect()));
        }
    }
}
