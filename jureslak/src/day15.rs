use crate::common::Part;

fn hash(s: &[u8]) -> i32 {
    let mut h = 0;
    for c in s {
        h += *c as i32;
        h *= 17;
        h %= 256;
    }
    h
}

enum Op<'a> {
    Add(&'a[u8], i32),
    Remove(&'a[u8]),
}

pub fn solve(data : &Vec<String>, part : Part) {
    let parts : Vec<&[u8]> = data[0].split(',').map(|s| s.as_bytes()).collect();
    match part {
        Part::First => {
            let s : i32 = parts.iter().map(|s| hash(s)).sum();
            println!("{}", s);
        }
        Part::Second => {
            let ops : Vec<Op> = parts.iter().map(|&s| {
                if s.contains(&b'=') {
                    let mut splitter = s.split(|&s| s == b'=');
                    let lab = splitter.next().unwrap();
                    let l = String::from_utf8(splitter.next().unwrap().to_vec()).unwrap().parse().unwrap();
                    Op::Add(lab, l)
                } else {
                    let lab = &s[..s.len()-1];
                    Op::Remove(lab)
                }
            }).collect();

            let mut tab : Vec<Vec<(&[u8], i32)>> = vec![vec![]; 256];
            for op in ops {
                match op {
                    Op::Add(lab, len) => {
                        let pos = hash(lab) as usize;
                        if let Some(p) = tab[pos].iter().position(|&v| v.0 == lab) {
                            tab[pos][p].1 = len;
                        } else {
                            tab[pos].push((lab, len));
                        }
                    }
                    Op::Remove(lab) => {
                        let pos = hash(lab) as usize;
                        if let Some(p) = tab[pos].iter().position(|&v| v.0 == lab) {
                            tab[pos].remove(p);
                        }
                    }
                }
            }

            let mut s = 0;
            for (i, b) in tab.iter().enumerate() {
                for (j, (_lab, len)) in b.iter().enumerate() {
                    s += (i+1) * (j+1) * *len as usize;
                }
            }

            println!("{}", s);
        }
    }
}
