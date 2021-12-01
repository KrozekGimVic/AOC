use crate::common::Part;
use std::collections::HashSet;

type Board = HashSet<Vec<i32>>;

fn neighbours(p: &Vec<i32>) -> Vec<Vec<i32>> {
    let mut neighbours = vec![p.clone()];
    for d in 0..p.len() {
        let n = neighbours.len();
        for j in 0..n {
            neighbours.push(neighbours[j].clone());
            let last = neighbours.len()-1;
            neighbours[last][d] = p[d]-1;
            neighbours.push(neighbours[j].clone());
            let last = neighbours.len()-1;
            neighbours[last][d] = p[d]+1;
        }
    }
    neighbours[1..].into()
}

fn count_neighbours(board: &Board, p: &Vec<i32>) -> usize {
    neighbours(p).iter().filter(|&p| board.contains(p)).count()
}

fn one_step(board: &Board) -> Board {
    let mut new_board = HashSet::new();
    for p in board.iter() {
        let active = count_neighbours(&board, p);
        if active == 2 || active == 3 {
            new_board.insert(p.clone());
        }
    }
    for p in board.iter() {
        for n in neighbours(p).iter() {
            if !board.contains(n) && !new_board.contains(n) && count_neighbours(board, n) == 3 {
                new_board.insert(n.clone());
            }
        }
    }
    new_board
}

pub fn solve(data : &Vec<String>, part : Part) {
    let coords : Vec<(i32, i32)> = data.iter().enumerate().map(|(i, s)| {
        s.chars().enumerate().filter_map(move |(j, c)| match c {
            '#' => Some((i as i32, j as i32)), _ => None
        })
    }).flatten().collect();
    match part {
        Part::First => {
            let mut board = HashSet::new();
            for (i, j) in coords {
                board.insert(vec![i, j, 0]);
            }
            for _ in 0..6 { board = one_step(&board); }
            println!("{}", board.len())
        },

        Part::Second => {
            let mut board = HashSet::new();
            for (i, j) in coords {
                board.insert(vec![i, j, 0, 0]);
            }
            for _ in 0..6 { board = one_step(&board); }
            println!("{}", board.len());
        },
    }
}
