use crate::common::Part;

fn from_bp(bytes: &[u8], zero: u8) -> i32 {
    let mut v : i32 = 0;
    for i in 0..bytes.len() {
        if bytes[bytes.len()-1-i] != zero {
            v += 1 << i;
        }
    }
    v
}

const ROW_BITS :usize = 7;
fn to_coord(s: &String) -> (i32, i32) {
    let first = &s.as_bytes()[..ROW_BITS];
    let second = &s.as_bytes()[ROW_BITS..];
    (from_bp(first, b'F'), from_bp(second, b'L'))
}

pub fn solve(data : &Vec<String>, part : Part) {
    let mut coords : Vec<i32> = data.iter().map(to_coord).map(|(i, j)| i*8+j).collect();
    match part {
        Part::First => println!("{}", coords.iter().max().expect("Maximum of an empty sequence.")),
        Part::Second => {
            coords.sort();
            for i in 1..coords.len() {
                if coords[i] - coords[i-1] == 2 {
                    println!("{}", coords[i]-1);
                }
            }
        }
    }
}