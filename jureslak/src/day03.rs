use crate::common::Part;

fn find_most_common(data: &Vec<&[u8]>) -> Vec<u8> {
    assert!(!data.is_empty());
    let w = data[0].len();
    let mut most_common = vec![0; w];
    let total = data.len();
    for i in 0..w {
        let one_count = data.iter().map(|s| s[i]).filter(|c| *c == b'1').count();
        most_common[i] = if 2*one_count > total { b'1' } else { b'0' };
    }
    most_common
}

fn flip(s: &Vec<u8>) -> Vec<u8> {
    s.iter().map(|c| if *c == b'0' { b'1' } else { b'0' }).collect()
}

fn b2i(b: Vec<u8>) -> i32 {
    i32::from_str_radix(&String::from_utf8(b).unwrap(), 2).unwrap()
}

fn isolate_internal(data: Vec<&[u8]>, keep_most_common: bool, i: usize) -> &[u8] {
    assert!(!data.is_empty());
    if data.len() == 1 {
        return data[0];
    }
    assert!(i < data[0].len());
    let one_count = data.iter().map(|s| s[i]).filter(|c| *c == b'1').count();
    let most_common = if 2*one_count < data.len() { 0 } else { 1 };
    let keep = if keep_most_common { most_common } else { 1 - most_common };
    let keep_byte = if keep == 1 { b'1' } else { b'0' };
    isolate_internal(data.iter().filter(|s| s[i] == keep_byte).map(|s| *s).collect(),
                     keep_most_common, i+1)
}

fn isolate(data: Vec<&[u8]>, keep_most_common: bool) -> &[u8] {
    isolate_internal(data, keep_most_common, 0usize)
}

pub fn solve(data : &Vec<String>, part : Part) {
    let bytes : Vec<&[u8]> = data.iter().map(|s| s.as_bytes()).collect();
    match part {
        Part::First => {
            let gamma = find_most_common(&bytes);
            let eps = flip(&gamma);

            println!("{}", b2i(gamma)*b2i(eps));
        }
        Part::Second => {
            let oxy = isolate(bytes.to_vec(), true);
            let co2 = isolate(bytes, false);
            println!("{}", b2i(oxy.to_vec())*b2i(co2.to_vec()));

        }
    }
}
