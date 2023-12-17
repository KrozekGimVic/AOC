use std::collections::{BinaryHeap, HashMap, HashSet};
use crate::common::Part;

#[derive(Ord, Eq, PartialOrd, PartialEq, Debug)]
struct Elem {
    loss: i32,
    state: State,
    prev: Option<State>,
}

#[derive(Ord, Eq, PartialOrd, PartialEq, Hash, Clone, Debug)]
struct State {
    i: i32,
    j: i32,
    di: i32,
    dj: i32,
    straight: i32,
}

fn turn_right(di: i32, dj: i32) -> (i32, i32) {
    (dj, -di)
}

fn turn_left(di: i32, dj: i32) -> (i32, i32) {
    (-dj, di)
}

fn make_move(i: i32, j: i32, di: i32, dj: i32, h: i32, w: i32, s: i32, loss: i32, map: &Vec<Vec<i32>>, q: &mut BinaryHeap<Elem>, prev: State) {
    if 0 <= i+di && i+di < h && 0 <= j+dj && j+dj < w {
        let ni = (i+di) as usize;
        let nj = (j+dj) as usize;
        q.push(Elem{ loss: loss - map[ni][nj], state: State{ i: ni as i32, j: nj as i32, di, dj, straight: s}, prev: Some(prev)});
    }
}

fn shortest_path(map: &Vec<Vec<i32>>) -> i32 {
    let h = map.len() as i32;
    let w = map[0].len() as i32;
    let mut prev: HashMap<State, State> = HashMap::new();
    let mut draw = vec![vec![b'.'; w as usize]; h as usize];
    let mut visited : HashSet<State> = HashSet::new();
    let mut q = BinaryHeap::new();
    q.push(Elem{ loss: -map[0][1], state: State {i: 0, j: 1, di: 0, dj: 1, straight: 1}, prev: None});
    q.push(Elem{ loss: -map[1][0], state: State {i: 1, j: 0, di: 1, dj: 0, straight: 1}, prev: None});
    while !q.is_empty() {
        let cur = q.pop().unwrap();

        let fresh = visited.insert(cur.state.clone());
        if !fresh { continue; }
        if let Some(p) = cur.prev {
            prev.insert(cur.state.clone(), p);
        }
        // println!("{:?}", cur);

        let State {i, j, di, dj, straight} = cur.state;
        if i == h-1 && j == w-1 {
            let mut op = cur.state;
            draw[op.i as usize][op.j as usize] = if op.di == 0 {
                if op.dj == 1 { b'>' } else { b'<' }
            } else {
                if op.di == 1 { b'v' } else { b'^' }
            };
            while let Some(p) = prev.get(&op) {
                // println!("{:?}", p);
                draw[p.i as usize][p.j as usize] = if p.di == 0 {
                    if p.dj == 1 { b'>' } else { b'<' }
                } else {
                    if p.di == 1 { b'v' } else { b'^' }
                };
                op = p.clone();
            }

            // for l in draw {
            //     println!("{}", String::from_utf8(l).unwrap());
            // }
            return -cur.loss;
        }


        if straight < 3 {
            make_move(i, j, di, dj, h, w, straight+1, cur.loss, &map, &mut q, cur.state.clone());
        }
        {
            let (di, dj) = turn_left(di, dj);
            make_move(i, j, di, dj, h, w, 1, cur.loss, &map, &mut q, cur.state.clone());
        }
        {
            let (di, dj) = turn_right(di, dj);
            make_move(i, j, di, dj, h, w, 1, cur.loss, &map, &mut q, cur.state.clone());
        }
    }
    panic!("No path!");
}

fn shortest_path2(map: &Vec<Vec<i32>>) -> i32 {
    let h = map.len() as i32;
    let w = map[0].len() as i32;
    let mut prev: HashMap<State, State> = HashMap::new();
    let mut draw = vec![vec![b'.'; w as usize]; h as usize];
    let mut visited : HashSet<State> = HashSet::new();
    let mut q = BinaryHeap::new();
    q.push(Elem{ loss: -map[0][1], state: State {i: 0, j: 1, di: 0, dj: 1, straight: 1}, prev: None});
    q.push(Elem{ loss: -map[1][0], state: State {i: 1, j: 0, di: 1, dj: 0, straight: 1}, prev: None});
    while !q.is_empty() {
        let cur = q.pop().unwrap();

        let fresh = visited.insert(cur.state.clone());
        if !fresh { continue; }
        if let Some(p) = cur.prev {
            prev.insert(cur.state.clone(), p);
        }
        // println!("{:?}", cur);

        let State {i, j, di, dj, straight} = cur.state;
        if i == h-1 && j == w-1 && straight >= 4 {
            let mut op = cur.state;
            draw[op.i as usize][op.j as usize] = if op.di == 0 {
                if op.dj == 1 { b'>' } else { b'<' }
            } else {
                if op.di == 1 { b'v' } else { b'^' }
            };
            while let Some(p) = prev.get(&op) {
                draw[p.i as usize][p.j as usize] = if p.di == 0 {
                    if p.dj == 1 { b'>' } else { b'<' }
                } else {
                    if p.di == 1 { b'v' } else { b'^' }
                };
                op = p.clone();
            }

            for l in draw {
                println!("{}", String::from_utf8(l).unwrap());
            }
            return -cur.loss;
        }


        if straight < 10 {
            make_move(i, j, di, dj, h, w, straight+1, cur.loss, &map, &mut q, cur.state.clone());
        }
        if straight >= 4 {
            let (di, dj) = turn_left(di, dj);
            make_move(i, j, di, dj, h, w, 1, cur.loss, &map, &mut q, cur.state.clone());
        }
        if straight >= 4 {
            let (di, dj) = turn_right(di, dj);
            make_move(i, j, di, dj, h, w, 1, cur.loss, &map, &mut q, cur.state.clone());
        }
    }
    panic!("No path!");
}


pub fn solve(data : &Vec<String>, part : Part) {
    let map : Vec<Vec<i32>> = data.iter().map(|s| s.as_bytes().iter().map(|&c| (c - b'0') as i32).collect()).collect();

    match part {
        Part::First => {
            println!("{}", shortest_path(&map));
        }
        Part::Second => {
            println!("{}", shortest_path2(&map));

        }
    }
}
