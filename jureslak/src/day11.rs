use crate::common::Part;

fn get_coords(map: &Vec<Vec<u8>>, f: isize) -> Vec<isize> {
    let mut y = vec![];
    let mut c = 0;
    let h = map.len();
    let w = map[0].len();
    for i in 0..h {
        if map[i].iter().all(|c| *c == b'.') {
            c += f;
        } else {
            for j in 0..w {
                if map[i][j] == b'#' {
                    y.push(c);
                }
            }
            c += 1;
        }
    }
    y
}

fn rot(map: &Vec<Vec<u8>>) -> Vec<Vec<u8>> {
    let h = map.len();
    let w = map[0].len();
    let mut r = vec![vec![b'.'; h]; w];
    for i in 0..h {
        for j in 0..w {
            r[j][h-i-1] = map[i][j];
        }
    }
    r
}

fn sumall(coor: &Vec<isize>) -> isize {
    let mut total = 0;
    let mut s = coor[0];
    for i in 1..coor.len() {
        total += (i as isize) * (coor.len()-i) as isize * (coor[i] - s);
        s = coor[i];
    }
    total
}

pub fn solve(data : &Vec<String>, part : Part) {
    let map : Vec<Vec<u8>> = data.iter().map(|s| s.as_bytes().to_vec()).collect();
    const FF : isize = 1000000;
    let x = get_coords(&map, 2);
    let x2 = get_coords(&map, FF);
    let rotmap = rot(&map);
    let y = get_coords(&rotmap, 2);
    let y2 = get_coords(&rotmap, FF);

    assert_eq!(sumall(&vec![1,2]), 1);
    assert_eq!(sumall(&vec![1,2,3]), 4);

    match part {
        Part::First => {
            println!("{}", sumall(&x) + sumall(&y));
        }
        Part::Second => {
            println!("{}", sumall(&x2) + sumall(&y2));
        }
    }
}
