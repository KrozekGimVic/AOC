use crate::common::Part;

struct PasswordData<'a> {
    password: &'a [u8],
    ch: u8,
    lo: u32,
    hi: u32,
}

fn satisfies_char_count(password: &[u8], ch: u8, lo: u32, hi: u32) -> bool {
    let count = password.iter().filter(|c| **c == ch).count() as u32;
    return lo <= count && count <= hi;
}

fn appears_only_once(password: &[u8], ch: u8, lo: u32, hi: u32) -> bool {
    let first = password[lo as usize - 1] == ch;
    let second = password[hi as usize - 1] == ch;
    return first ^ second;
}

fn parse_password_data(s : &String) -> PasswordData {
    let parts : Vec<&str> = s.split(':').collect();
    assert_eq!(parts.len(), 2);
    let password = parts[1].trim();
    let desc_parts : Vec<&str> = parts[0].split(' ').collect();
    assert_eq!(desc_parts.len(), 2);
    assert_eq!(desc_parts[1].len(), 1);
    let ch = desc_parts[1].as_bytes()[0];
    let limits : Vec<u32> = desc_parts[0].split('-').map(|s| s.parse().expect("Expected an integer.")).collect();
    assert_eq!(limits.len(), 2);
    PasswordData{
        password: password.as_bytes(),
        ch: ch,
        lo: limits[0],
        hi: limits[1],
    }
}

fn num_valid_1(data : &Vec<PasswordData>) -> usize {
    data.iter().filter(|d| satisfies_char_count(d.password, d.ch, d.lo, d.hi)).count()
}

fn num_valid_2(data : &Vec<PasswordData>) -> usize {
    data.iter().filter(|d| appears_only_once(d.password, d.ch, d.lo, d.hi)).count()
}

pub fn solve(data : &Vec<String>, part : Part) {
    let parsed_data: Vec<PasswordData> = data.iter().map(parse_password_data).collect();
    match part {
        Part::First => println!("{}", num_valid_1(&parsed_data)),
        Part::Second => println!("{}", num_valid_2(&parsed_data)),
    }
}
