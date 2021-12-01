use crate::common::Part;

fn count_trees(map : &Vec<Vec<u8>>, dx : usize, dy: usize) -> i32 {
    let m = map[0].len();
    let mut j : usize = 0;
    let mut c = 0;
    for i in (dy..map.len()).step_by(dy) {
        j += dx;
        j %= m;
        c += (map[i][j] == b'#') as i32;
    }
    c
}

fn count_trees_for_slopes(map : &Vec<Vec<u8>>, slopes : &[(usize, usize)]) -> i64 {
    let mut p : i64 = 1;
    for (dx, dy) in slopes.iter() {
        p *= count_trees(map, *dx, *dy) as i64;
    }
    p
}

pub fn solve(data : &Vec<String>, part : Part) {
    let map : Vec<Vec<u8>> = data.iter()
        .map(|l| l.trim().into())
        .collect();
    match part {
        Part::First => println!("{}", count_trees(&map, 3, 1)),
        Part::Second => println!("{}", count_trees_for_slopes(&map, &[(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)])),
    }
}