use crate::common::Part;

fn connect_adaptors(voltages: &Vec<i32>) -> Result<(i32, i32), &str> {
    let mut diff_count = vec![0, 0, 0, 0];
    for i in 0..voltages.len()-1 {
        let diff = voltages[i+1] - voltages[i];
        if diff >= 4 {
            return Err("Difference too large.")
        }
        diff_count[diff as usize] += 1
    }
    Ok((diff_count[1], diff_count[3]))
}

fn num_arrangements(voltages: &Vec<i32>) -> i64 {
    let mut num_order : Vec<i64> = vec![0; voltages.len()];
    // num_order[i] = number of arrangements from i to the end
    num_order[voltages.len() - 1] = 1;
    for i in (0..voltages.len()-1).rev() {
        let mut j = 1;
        while i+j < voltages.len() && voltages[i+j] - voltages[i] <= 3 {
            num_order[i] += num_order[i+j];
            j += 1;
        }
    }
    num_order[0]
}

pub fn solve(data : &Vec<String>, part : Part) {
    let mut voltages : Vec<i32> = data.iter().map(|s| s.parse().unwrap()).collect();
    let device = voltages.iter().max().expect("Max of empty.") + 3;
    voltages.push(0);
    voltages.push(device);
    voltages.sort();
    match part {
        Part::First => {
            let (i, j) = connect_adaptors(&voltages).expect("Invalid voltages.");
            println!("{}", i*j)
        },
        Part::Second => println!("{}", num_arrangements(&voltages)),
    }
}
