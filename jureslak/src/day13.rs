use crate::common::Part;
use itertools::Itertools;

#[derive(Debug)]
enum Direction {
    Left,
    Up,
}

#[derive(Debug)]
struct Fold {
    value: i32,
    direction: Direction,
}

fn execute_fold(points: &mut Vec<(i32, i32)>, fold: &Fold) {
    for point in points.iter_mut() {
        let coor = match fold.direction { Direction::Left => &mut point.0, Direction::Up => &mut point.1, };
        if *coor > fold.value {
            *coor -= 2*(*coor - fold.value);
        } else if *coor == fold.value {
            *point = (-1, -1);
        }
    }
    points.sort();
    points.dedup();
    if let Some(idx) = points.iter().position(|&s| s == (-1, -1)) {
        points.remove(idx);
    }
}

fn execute_folds(mut points: Vec<(i32, i32)>, folds: &Vec<Fold>) -> Vec<(i32, i32)> {
    for f in folds.iter() {
        execute_fold(&mut points, f);
    }
    points
}

pub fn solve(data : &Vec<String>, part : Part) {
    let all = data.join("\n");
    let (points, instructions) = all.split("\n\n").collect_tuple().unwrap();

    let points: Vec<(i32, i32)> = points.split("\n").map(|s| {
        let (a, b) = s.split(',').collect_tuple().unwrap();
        (a.parse().unwrap(), b.parse().unwrap())
    }).collect();

    let instructions: Vec<Fold> = instructions.split("\n").map(|s: &str| {
        let s = &s[11..];
        Fold {
            value: s[2..].parse().unwrap(),
            direction: match s.chars().next().unwrap() { 'x' => Direction::Left, 'y' => Direction::Up, _ => panic!("invalid input") }
        }
    }).collect();

    match part {
        Part::First => {
            let mut points = points.clone();
            execute_fold(&mut points, &instructions[0]);
            println!("{}", points.len());
        }
        Part::Second => {
            let points = execute_folds(points, &instructions);
            let mut img = vec![vec![b'.'; 40]; 10];
            for &(i, j) in points.iter() {
                img[j as usize][i as usize] = b'#';
            }
            for r in img.iter() {
                println!("{}", String::from_utf8_lossy(r));
            }
            println!("{}", 1);
        }
    }
}
