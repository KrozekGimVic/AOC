use crate::common::Part;

fn first_bus(buses: &Vec<i32>, time: i32) -> (i32, i32) {
    // println!("{:?}", buses);
    // println!("{:?}", buses.iter().map(|s| s - time % s).collect::<Vec<i32>>());
    let best = buses.iter().min_by_key(|&s| s - time % s).unwrap();
    (*best, best - time % best)
}

// Extended GCD. Solves ax + by == gcd(a, b). Returns (gcd, x, y).
pub fn egcd(a: i64, b: i64) -> (i64, i64, i64) {
    let mut a = a;
    let mut b = b;
    let mut x = 0;
    let mut px = 1;
    let mut y = 1;
    let mut py= 0;
    let mut r;
    let mut q;
    while b != 0 {
        r = a % b; q = a / b;  // quotient and reminder
        a = b; b = r;          // gcd swap
        r = px - q * x;        // x swap
        px = x; x = r;
        r = py - q * y;        // y swap
        py = y; y = r;
    }
    (a, px, py)
}

pub fn mul_inverse(a: i64, m: i64) -> Option<i64> {
    let (g, x, _) = egcd(a, m);
    if g != 1 { None } else { Some((x % m + m) % m) }
}

// Solve linear system of congruences using Chinese remainder theorem.
// The input is [(m_i, a_i)], representing a system of x % m_i == a_i
fn solve_congruence_system(eqs: &Vec<(i32, i32)>) -> i64 {
    let final_mod: i64 = eqs.iter().map(|(m, _)| *m as i64).product();
    let mut x = 0;
    for (m, a) in eqs.iter().map(|(m, a)| (*m as i64, *a as i64)) {
        x += a * final_mod / m * mul_inverse(final_mod / m, m).expect("Could not compute inverse.");
        x %= final_mod;
    }
    (x % final_mod + final_mod) % final_mod
}

pub fn solve(data : &Vec<String>, part : Part) {
    match part {
        Part::First => {
            let time : i32 = data[0].parse().expect("Invalid input.");
            let buses : Vec<i32> = data[1].split(',').filter_map(|s| s.parse().ok()).collect();
            let (best, wait) = first_bus(&buses, time);
            println!("{} {} {}", best, wait, best*wait);
        },
        Part::Second => {
            let congruences : Vec<(i32, i32)> = data[1].split(',').enumerate().filter_map(|(i, s)| {
                let num = s.parse();
                match num {
                    Ok(value) => Some((value, value - (i as i32) % value)),  // t % value == rhs
                    Err(_) => None,
                }
            }).collect();
            println!("{:?}", congruences);
            let t = solve_congruence_system(&congruences);
            println!("{}", t);
        },
    }
}
