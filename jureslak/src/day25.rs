use crate::common::Part;

fn discrete_log(ga: i64, g: i64, modulo: i64) -> i64 {
    let mut attempt = 1;
    for a in 1..modulo {
        attempt *= g;
        attempt %= modulo;
        if ga == attempt {
            return a;
        }
    }
    panic!("No a satisfies g^a === ga (mod m).");
}

fn bruteforce_diffie_hellman(ga: i64, gb: i64, g: i64, modulo: i64) -> i64 {
    let a = discrete_log(ga, g, modulo);
    let mut key = 1;
    for _ in 0..a {
        key *= gb;
        key %= modulo;
    }
    key
}

pub fn solve(data : &Vec<String>, part : Part) {
    let ga : i64 = data[0].parse().unwrap();
    let gb : i64 = data[1].parse().unwrap();
    let g = 7;
    let m = 20201227;

    match part {
        Part::First => {
            println!("{}", bruteforce_diffie_hellman(ga, gb, g, m));
        },
        Part::Second => {
            println!("{}", 1);
        },
    }
}
