use crate::common::Part;

#[derive(Debug)]
struct Elem {
    di: i32,
    dj: i32,
    r: i32,
}

fn get_edge(inst: &Vec<Elem>) -> Vec<(f64, f64)> {
    let mut res = vec![];
    res.reserve(inst.len());
    let mut ci = 0.5;
    let mut cj = 0.5;
    for i in 0..inst.len() {
        let e = &inst[i];
        let pe = if i == 0 { &inst[inst.len() - 1] } else { &inst[i-1] };
        let da = (pe.di+e.di) as f64;
        let db = (pe.dj+e.dj) as f64;
        res.push((ci - 0.5*db, cj + 0.5*da));

        // println!("{} {}", ci, cj);

        ci += (e.di*e.r) as f64;
        cj += (e.dj*e.r) as f64;
    }
    res
}

fn area(pts: &Vec<(f64, f64)>) -> f64 {
    let mut area = 0.0;
    for i in 0..pts.len() {
        let j = if i == pts.len()-1 { 0 } else { i+1 };
        area += pts[i].0*pts[j].1 - pts[j].0*pts[i].1;
    }
    area.abs() / 2.0
}

pub fn solve(data : &Vec<String>, part : Part) {
    match part {
        Part::First => {
            let inst : Vec<Elem> = data.iter().map(|s| {
                let mut splitter = s.split_whitespace();
                let d = splitter.next().unwrap();
                let r = splitter.next().unwrap().parse().unwrap();
                let (di, dj) = match d {
                    "L" => (0, -1),
                    "R" => (0, 1),
                    "U" => (-1, 0),
                    "D" => (1, 0),
                    &_ => panic!("Unknown dir: {}", d),
                };
                Elem { di, dj, r}
            }).collect();
            let coor = get_edge(&inst);
            // println!("{:?}", coor);
            println!("{}", area(&coor));
        }
        Part::Second => {
            let inst : Vec<Elem> = data.iter().map(|s| {
                let mut splitter = s.split_whitespace();
                splitter.next();
                splitter.next();
                let r = splitter.next().unwrap();
                let r = &r[2..r.len()-1];
                let dist = i32::from_str_radix(&r[..5], 16).unwrap();
                let (di, dj) = match r.as_bytes()[r.len()-1] {
                    b'2' => (0, -1),
                    b'0' => (0, 1),
                    b'3' => (-1, 0),
                    b'1' => (1, 0),
                    _ => panic!("Unknown dir: {}", r),
                };
                Elem { di, dj, r: dist}
            }).collect();
            // println!("{:?}", inst);
            let coor = get_edge(&inst);
            println!("{}", area(&coor));

        }
    }
}
