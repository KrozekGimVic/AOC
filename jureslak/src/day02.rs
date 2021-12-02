use crate::common::Part;

fn get_position(instructions: Vec<(&str, i32)>) -> (i32, i32) {
    let mut depth = 0;
    let mut dist = 0;
    for (inst, value) in instructions {
        match inst {
            "forward" => { dist += value }
            "up" => { depth -= value }
            "down" => { depth += value }
            _ => panic!("Invalid instruction.")
        }
    }
    (dist, depth)
}

fn get_position2(instructions: Vec<(&str, i32)>) -> (i32, i32) {
    let mut depth = 0;
    let mut dist = 0;
    let mut aim = 0;
    for (inst, value) in instructions {
        match inst {
            "forward" => { dist += value; depth += aim*value; }
            "up" => { aim -= value }
            "down" => { aim += value }
            _ => panic!("Invalid instruction.")
        }
    }
    (dist, depth)
}

pub fn solve(data : &Vec<String>, part : Part) {
    let instructions: Vec<(&str, i32)> = data.iter().map(|l| l.split_whitespace().collect()).map(|s: Vec<&str>| {
        assert_eq!(s.len(), 2);
        (s[0], s[1].parse().expect("Invalid input."))
    }).collect();

    match part {
        Part::First => {
            let (dist, depth) = get_position(instructions);
            println!("{}", dist*depth);
        }
        Part::Second => {
            let (dist, depth) = get_position2(instructions);
            println!("{}", dist*depth);

        }
    }
}
