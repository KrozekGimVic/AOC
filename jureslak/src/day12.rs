use crate::common::Part;
use crate::day12::Instruction::{North, East, Forward, Left};

fn actual_mod(a: i32, m: i32) -> i32 { ((a % m) + m) % m }

fn sin(phi: i32) -> i32 {
    match actual_mod(phi, 360) {
        0 => 0,
        90 => 1,
        180 => 0,
        270 => -1,
        _ => panic!("Unsupported arg."),
    }
}

fn cos(phi: i32) -> i32 {
    match actual_mod(phi, 360) {
        0 => 1,
        90 => 0,
        180 => -1,
        270 => 0,
        _ => panic!("Unsupported arg."),
    }
}

#[derive(Debug)]
enum Instruction {
    East(i32),
    North(i32),
    Forward(i32),
    Left(i32),
}

fn parse_instruction(s: &String) -> Instruction {
    let code = s.as_bytes().get(0).expect("Invalid input.");
    let value : i32 = s[1..].parse().expect("Invalid input.");
    match code {
        b'N' => North(value),
        b'S' => North(-value),
        b'E' => East(value),
        b'W' => East(-value),
        b'L' => Left(value),
        b'R' => Left(-value),
        b'F' => Forward(value),
        _ => panic!("Invalid input."),
    }
}

fn execute_instructions(instructions: &Vec<Instruction>, x0: i32, y0: i32, dx0: i32, dy0: i32, move_waypoint: bool) -> (i32, i32) {
    // println!("{:?}", instructions);
    let (mut x, mut y, mut vx, mut vy) = (x0, y0, dx0, dy0);
    for i in instructions {
        match i {
            East(value) => if move_waypoint { vx += value; } else { x += value; },
            North(value) => if move_waypoint { vy += value; } else { y += value; },
            Forward(value) => { x += vx*value; y += vy*value; },
            Left(phi) => {
                let (c, s) = (cos(*phi), sin(*phi));
                let tmp = c*vx - s*vy;
                vy = s*vx + c*vy;
                vx = tmp;
            }
        }
        // println!("p: {} {}, v: {} {}", x, y, vx, vy);
    }
    (x, y)
}

pub fn solve(data : &Vec<String>, part : Part) {
    let instruction_list: Vec<Instruction> = data.iter().map(parse_instruction).collect();

    match part {
        Part::First => {
            let (xf, yf) = execute_instructions(&instruction_list, 0, 0, 1, 0, false);
            println!("{}", xf.abs() + yf.abs());
        },
        Part::Second => {
            let (xf, yf) = execute_instructions(&instruction_list, 0, 0, 10, 1, true);
            println!("{}", xf.abs() + yf.abs());
        },
    }
}
